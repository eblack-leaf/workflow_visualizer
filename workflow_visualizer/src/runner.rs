use crate::visualizer::Visualizer;
use crate::{Area, DeviceContext, SyncPoint};
use bevy_ecs::prelude::{EventReader, Events, IntoSystemConfig, NonSend, Resource};
use gloo_worker::{HandlerId, Spawnable, Worker, WorkerBridge};
use std::fmt::Debug;
use std::future::Future;
use std::marker::PhantomData;
use std::rc::Rc;
use tracing::trace;
use tracing::{info, warn};
use winit::dpi::PhysicalSize;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{
    ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget,
};
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;
use winit::window::{Fullscreen, Window, WindowBuilder};

pub trait Workflow {
    type Action: Debug + Clone + PartialEq + Send + Sync + Sized + 'static;
    type Response: Debug + Clone + PartialEq + Send + Sync + Sized + 'static;
    fn handle_response(visualizer: &mut Visualizer, response: Self::Response);
    fn exit_action() -> Self::Action;
    fn exit_response() -> Self::Response;
}
pub trait WorkflowWebExt
where
    Self: gloo_worker::Worker + Workflow + Send + 'static,
{
    fn action_to_input(action: <Self as Workflow>::Action) -> <Self as Worker>::Input;
    fn output_to_response(output: <Self as Worker>::Output) -> <Self as Workflow>::Response;
}
pub struct Receiver<T: Send + 'static> {
    #[cfg(not(target_family = "wasm"))]
    pub receiver: tokio::sync::mpsc::UnboundedReceiver<T>,
    #[cfg(target_family = "wasm")]
    pub receiver: T,
}

impl<T: Send + 'static> Receiver<T> {
    #[cfg(not(target_family = "wasm"))]
    pub async fn receive(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
    #[cfg(target_family = "wasm")]
    pub fn receive(&mut self) {}
}
pub struct Responder<T: Send + 'static + Debug>(pub EventLoopProxy<T>);
impl<T: Send + 'static + Debug> Responder<T> {
    pub fn respond(&self, response: T) {
        self.0.send_event(response).expect("responder");
    }
}
struct ExitSignal {}
#[cfg(target_os = "android")]
#[derive(Resource)]
pub struct AndroidInterface(pub AndroidApp);
pub struct Runner {
    desktop_dimensions: Option<Area<DeviceContext>>,
    #[cfg(not(target_os = "android"))]
    android_app: Option<()>,
    #[cfg(target_os = "android")]
    android_app: Option<AndroidApp>,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            desktop_dimensions: None,
            android_app: None,
        }
    }
    #[cfg(target_os = "android")]
    pub fn with_android_app(mut self, android_app: AndroidApp) -> Self {
        self.android_app.replace(android_app);
        self
    }
    pub fn with_desktop_dimensions<A: Into<Area<DeviceContext>>>(mut self, dim: A) -> Self {
        self.desktop_dimensions.replace(dim.into());
        self
    }
    #[cfg(not(target_family = "wasm"))]
    pub fn native_run<T: Workflow + Send + 'static, NativeEngenRunner, NativeEngenRunnerFut>(
        mut self,
        mut visualizer: Visualizer,
        native_engen_runner: NativeEngenRunner,
    ) where
        NativeEngenRunner: FnOnce(Responder<T::Response>, Receiver<T::Action>) -> NativeEngenRunnerFut
            + Send
            + 'static,
        NativeEngenRunnerFut: Future<Output = ()> + Send + 'static,
    {
        let tokio_runtime = tokio::runtime::Runtime::new().expect("tokio runtime");
        tokio_runtime.block_on(async {
            let builder = &mut EventLoopBuilder::<T::Response>::with_user_event();
            #[cfg(target_os = "android")]
            {
                use winit::platform::android::EventLoopBuilderExtAndroid;
                let android_app = self.android_app.take().unwrap();
                builder.with_android_app(android_app.clone());
                visualizer
                    .job
                    .container
                    .insert_resource(AndroidInterface(android_app.clone()));
            }
            let event_loop = builder.build();
            let (sender, receiver): (
                tokio::sync::mpsc::UnboundedSender<T::Action>,
                tokio::sync::mpsc::UnboundedReceiver<T::Action>,
            ) = tokio::sync::mpsc::unbounded_channel();
            let proxy = event_loop.create_proxy();
            tokio::task::spawn(async move {
                native_engen_runner(Responder(proxy), Receiver { receiver }).await
            });
            visualizer
                .job
                .container
                .insert_non_send_resource(Sender::new(NativeSender::<T>::new(sender)));
            let mut window: Option<Rc<Window>> = None;
            let mut initialized = false;
            Self::add_exit_signal_handler::<T>(&mut visualizer);
            let desktop_dimensions = self.desktop_dimensions;
            event_loop.run(move |event, event_loop_window_target, control_flow| {
                Self::internal_loop::<T>(
                    &mut visualizer,
                    &mut window,
                    &mut initialized,
                    event,
                    event_loop_window_target,
                    control_flow,
                    desktop_dimensions,
                );
            });
        });
    }
    #[cfg(not(target_family = "wasm"))]
    fn add_exit_signal_handler<T: Workflow + Send + 'static>(visualizer: &mut Visualizer) {
        visualizer
            .job
            .container
            .insert_resource(Events::<ExitSignal>::default());
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            Events::<ExitSignal>::update_system.in_set(SyncPoint::Event),
            Self::send_exit_request::<T>.in_set(SyncPoint::Initialization),
        ));
    }
    #[cfg(target_family = "wasm")]
    fn add_exit_signal_handler<T: Workflow + WorkflowWebExt + Send + 'static>(
        visualizer: &mut Visualizer,
    ) {
        visualizer
            .job
            .container
            .insert_resource(Events::<ExitSignal>::default());
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            Events::<ExitSignal>::update_system.in_set(SyncPoint::Event),
            Self::send_exit_request::<T>.in_set(SyncPoint::Initialization),
        ));
    }
    #[cfg(not(target_family = "wasm"))]
    fn send_exit_request<T: Workflow + Send + 'static>(
        sender: NonSend<Sender<T>>,
        mut exit_requests: EventReader<ExitSignal>,
    ) {
        if !exit_requests.is_empty() {
            sender.send(T::exit_action());
        }
    }
    #[cfg(target_family = "wasm")]
    fn send_exit_request<T: Workflow + WorkflowWebExt + Send + 'static>(
        sender: NonSend<Sender<T>>,
        mut exit_requests: EventReader<ExitSignal>,
    ) {
        if !exit_requests.is_empty() {
            sender.send(T::exit_action());
        }
    }
    fn internal_loop<T: Workflow + Send + 'static>(
        mut visualizer: &mut Visualizer,
        mut window: &mut Option<Rc<Window>>,
        initialized: &mut bool,
        event: Event<<T as Workflow>::Response>,
        event_loop_window_target: &EventLoopWindowTarget<<T as Workflow>::Response>,
        control_flow: &mut ControlFlow,
        desktop_dimensions: Option<Area<DeviceContext>>,
    ) {
        control_flow.set_wait();
        match event {
            Event::NewEvents(cause) => match cause {
                StartCause::Init => {
                    #[cfg(not(target_os = "android"))]
                    {
                        #[cfg(not(target_family = "wasm"))]
                        initialize_native_window(
                            &event_loop_window_target,
                            &mut window,
                            desktop_dimensions,
                        );
                        visualizer.initialize(window.as_ref().unwrap());
                        *initialized = true;
                    }
                }
                _ => {}
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    visualizer.job.container.send_event(ExitSignal {});
                }
                WindowEvent::Resized(size) => {
                    info!("resizing: {:?}", size);
                    let scale_factor = window.as_ref().unwrap().scale_factor();
                    visualizer.trigger_resize(size, scale_factor);
                }
                WindowEvent::ScaleFactorChanged {
                    new_inner_size,
                    scale_factor,
                } => {
                    info!("resizing: {:?}", *new_inner_size);
                    visualizer.trigger_resize(*new_inner_size, scale_factor);
                }
                WindowEvent::Touch(touch) => {
                    visualizer.register_touch(touch);
                    info!("touch {:?}", touch);
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    visualizer.register_mouse_click(state, button);
                }
                WindowEvent::MouseWheel { .. } => {}
                WindowEvent::CursorMoved { position, .. } => {
                    visualizer.set_mouse_location(position);
                }
                WindowEvent::CursorEntered { device_id: _ } => {}
                WindowEvent::CursorLeft { device_id: _ } => {
                    visualizer.cancel_touches();
                }
                WindowEvent::ReceivedCharacter(ch) => {
                    trace!("char: {:?}", ch);
                }
                _ => {}
            },
            Event::UserEvent(event) => {
                info!("visualizer loop received: {:?}", event);
                if event == T::exit_response() {
                    control_flow.set_exit();
                }
                T::handle_response(&mut visualizer, event);
            }
            Event::MainEventsCleared => {
                visualizer.exec();
            }
            Event::RedrawRequested(_) => {
                visualizer.render();
            }
            Event::RedrawEventsCleared => {
                if visualizer.job.resumed() && *initialized {
                    window.as_ref().unwrap().request_redraw();
                }
                if visualizer.can_idle() {
                    control_flow.set_wait();
                }
            }
            Event::Suspended => {
                info!("suspending");
                #[cfg(target_os = "android")]
                {
                    visualizer.suspend();
                }
            }
            Event::Resumed => {
                info!("resuming");
                #[cfg(target_os = "android")]
                {
                    if !initialized {
                        initialize_native_window(
                            &event_loop_window_target,
                            &mut window,
                            desktop_dimensions,
                        );
                        visualizer.initialize(window.as_ref().unwrap());
                        *initialized = true;
                    } else {
                        visualizer.resume(window.as_ref().unwrap());
                    }
                }
            }
            Event::LoopDestroyed => {
                visualizer.teardown();
            }
            _ => {}
        }
    }
    #[cfg(target_family = "wasm")]
    pub fn web_run<T: WorkflowWebExt + gloo_worker::Worker + Send + 'static>(
        mut self,
        visualizer: Visualizer,
        worker_path: String,
    ) {
        #[cfg(target_family = "wasm")]
        wasm_bindgen_futures::spawn_local(self.internal_web_run::<T>(visualizer, worker_path));
    }
    #[cfg(target_family = "wasm")]
    async fn internal_web_run<T: WorkflowWebExt + gloo_worker::Worker + Send + 'static>(
        mut self,
        mut visualizer: Visualizer,
        worker_path: String,
    ) {
        let event_loop = EventLoopBuilder::<T::Response>::with_user_event().build();
        let mut window = Some(Rc::new(
            WindowBuilder::new()
                .with_title("workflow_visualizer")
                .build(&event_loop)
                .expect("window"),
        ));
        Self::add_web_canvas(window.as_ref().unwrap());
        window
            .as_ref()
            .unwrap()
            .set_inner_size(Self::window_dimensions(
                window.as_ref().unwrap().scale_factor(),
            ));
        visualizer.init_gfx(window.as_ref().unwrap()).await;
        Self::web_resizing(window.as_ref().unwrap());
        let proxy = event_loop.create_proxy();
        let bridge = T::spawner()
            .callback(move |response| {
                proxy.send_event(T::output_to_response(response));
            })
            .spawn(worker_path.as_str());
        let bridge = Box::leak(Box::new(bridge));
        visualizer
            .job
            .container
            .insert_non_send_resource(Sender::new(WebSender(bridge)));
        let mut initialized = true;
        Self::add_exit_signal_handler::<T>(&mut visualizer);
        use winit::platform::web::EventLoopExtWebSys;
        event_loop.spawn(move |event, event_loop_window_target, control_flow| {
            Self::internal_loop::<T>(
                &mut visualizer,
                &mut window,
                &mut initialized,
                event,
                event_loop_window_target,
                control_flow,
                None,
            );
        });
    }
    #[cfg(target_arch = "wasm32")]
    fn add_web_canvas(window: &Window) {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
    }
    #[cfg(target_arch = "wasm32")]
    fn window_dimensions(scale_factor: f64) -> PhysicalSize<u32> {
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body: web_sys::HtmlElement| -> Option<PhysicalSize<u32>> {
                let width: u32 = body.client_width().try_into().unwrap();
                let height: u32 = body.client_height().try_into().unwrap();
                Some(PhysicalSize::new(
                    (width as f64 * scale_factor) as u32,
                    (height as f64 * scale_factor) as u32,
                ))
            })
            .expect("could not create inner size")
    }

    #[cfg(target_arch = "wasm32")]
    fn web_resizing(window: &Rc<Window>) {
        use wasm_bindgen::{prelude::*, JsCast};
        let w_window = window.clone();
        let closure = Closure::wrap(Box::new(move |_e: web_sys::Event| {
            let scale_factor = w_window.scale_factor();
            let size = Self::window_dimensions(scale_factor);
            w_window.set_inner_size(size);
        }) as Box<dyn FnMut(_)>);
        let _ = web_sys::window()
            .expect("no web_sys window")
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
        match web_sys::window().expect("no web_sys window").screen() {
            Ok(screen) => {
                let _ = screen
                    .orientation()
                    .add_event_listener_with_callback("onchange", closure.as_ref().unchecked_ref());
            }
            Err(_) => {}
        }
        closure.forget();
    }
}

pub(crate) fn initialize_native_window<T>(
    w_target: &EventLoopWindowTarget<T>,
    window: &mut Option<Rc<Window>>,
    desktop_dimensions: Option<Area<DeviceContext>>,
) {
    let mut builder = WindowBuilder::new().with_resizable(false);
    #[cfg(all(not(target_os = "android"), not(target_family = "wasm")))]
    {
        let desktop_dimensions = match desktop_dimensions {
            None => Area::new(600.0, 800.0),
            Some(dim) => dim.into(),
        };
        builder = builder.with_inner_size(PhysicalSize::new(
            desktop_dimensions.width,
            desktop_dimensions.height,
        ));
    }
    window.replace(Rc::new(builder.build(w_target).expect("window")));
}
#[cfg(not(target_family = "wasm"))]
#[derive(Resource)]
pub struct Sender<T: Workflow> {
    sender: NativeSender<T>,
}
#[cfg(target_family = "wasm")]
#[derive(Resource)]
pub struct Sender<T: WorkflowWebExt> {
    sender: WebSender<T>,
}
#[cfg(not(target_family = "wasm"))]
impl<T: Workflow> Sender<T> {
    fn new(sender: NativeSender<T>) -> Self {
        Self { sender }
    }
    pub fn send(&self, action: <T as Workflow>::Action) {
        self.sender.send(action);
    }
}
#[cfg(target_family = "wasm")]
impl<T: WorkflowWebExt> Sender<T> {
    fn new(sender: WebSender<T>) -> Self {
        Self { sender }
    }
    pub fn send(&self, action: <T as Workflow>::Action) {
        self.sender.send(T::action_to_input(action));
    }
}

#[cfg(target_family = "wasm")]
pub struct WebSender<T: Worker>(pub &'static mut WorkerBridge<T>);
#[cfg(target_family = "wasm")]
impl<T: Worker + Send + 'static> WebSender<T> {
    pub(crate) fn send(&self, input: <T as Worker>::Input) {
        self.0.send(input);
    }
}
#[cfg(not(target_family = "wasm"))]
pub struct NativeSender<T: Workflow>(pub tokio::sync::mpsc::UnboundedSender<T::Action>);
#[cfg(not(target_family = "wasm"))]
impl<T: Workflow> NativeSender<T> {
    pub(crate) fn new(sender: tokio::sync::mpsc::UnboundedSender<T::Action>) -> Self {
        Self(sender)
    }
    pub fn send(&self, action: <T as Workflow>::Action) {
        self.0.send(action).expect("native sender");
    }
}
pub struct OutputWrapper<T: WorkflowWebExt> {
    pub handler_id: HandlerId,
    pub response: <T as Worker>::Output,
}
impl<T: WorkflowWebExt> OutputWrapper<T> {
    pub fn new(handler_id: HandlerId, response: <T as Worker>::Output) -> Self {
        Self {
            handler_id,
            response,
        }
    }
}
