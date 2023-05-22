use crate::{Area, DeviceContext, Visualizer};
use bevy_ecs::prelude::Resource;
use std::fmt::Debug;
use std::future::Future;
use std::marker::PhantomData;
use tracing::{info, warn};
use winit::dpi::PhysicalSize;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget};
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;
use winit::window::{Fullscreen, Window, WindowBuilder};
pub trait Workflow {
    type Action: Debug + Clone + PartialEq + Send + 'static;
    type Response: Debug + Clone + PartialEq + Send + 'static;
    fn handle_response(visualizer: &mut Visualizer, response: Self::Response);
    fn exit_action() -> Self::Action;
    fn exit_response() -> Self::Response;
}
pub struct Receiver<T: Send + 'static>(pub tokio::sync::mpsc::UnboundedReceiver<T>);
impl<T: Send + 'static> Receiver<T> {
    pub async fn receive(&mut self) -> Option<T> {
        self.0.recv().await
    }
}
pub struct Responder<T: Send + 'static + Debug>(pub EventLoopProxy<T>);
impl<T: Send + 'static + Debug> Responder<T> {
    pub fn respond(&self, response: T) {
        self.0.send_event(response).expect("responder");
    }
}
#[cfg(target_os = "android")]
#[derive(Resource)]
pub struct AndroidInterface(pub AndroidApp);
pub struct Runner<T: Workflow + Send + 'static> {
    _t: PhantomData<T>,
    desktop_dimensions: Option<Area<DeviceContext>>,
    #[cfg(not(target_os = "android"))]
    android_app: Option<()>,
    #[cfg(target_os = "android")]
    android_app: Option<AndroidApp>,
}

impl<T: Workflow + Send + 'static> Runner<T> {
    pub fn new() -> Self {
        Self {
            _t: PhantomData,
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
    #[cfg(target_family = "wasm")]
    pub fn web_run(visualizer: Visualizer) {
        #[cfg(target_family = "wasm")]
        wasm_bindgen_futures::spawn_local(self.wasm_run(visualizer));
    }
    #[cfg(target_family = "wasm")]
    async fn wasm_run(mut self, visualizer: Visualizer) {
        use winit::platform::web::WindowExtWebSys;
        // init window and js callbacks
        // event_loop.spawn(); including sender receiver pattern on web
    }
    #[cfg(not(target_family = "wasm"))]
    pub fn native_run<NativeEngenRunner, NativeEngenRunnerFut>(
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
                native_engen_runner(Responder(proxy), Receiver(receiver)).await
            });
            let mut window: Option<Window> = None;
            let mut initialized = false;
            event_loop.run(move |event, event_loop_window_target, control_flow| {
                control_flow.set_wait();
                match event {
                    Event::NewEvents(cause) => match cause {
                        StartCause::Init => {
                            #[cfg(not(target_os = "android"))]
                            {
                                initialize_native_window(
                                    &event_loop_window_target,
                                    &mut window,
                                    self.desktop_dimensions,
                                );
                                visualizer.initialize(window.as_ref().unwrap());
                                initialized = true;
                            }
                        }
                        _ => {}
                    },
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            if let Ok(_) = sender.send(T::exit_action()) {
                                info!("sending is ok");
                            } else {
                                info!("could not send");
                            }
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
                            warn!("char: {:?}", ch);
                        }
                        _ => {}
                    },
                    Event::UserEvent(event) => {
                        warn!("visualizer loop received: {:?}", event);
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
                        if visualizer.job.active() && initialized {
                            window.as_ref().unwrap().request_redraw();
                        }
                        if visualizer.can_idle() {
                            control_flow.set_wait();
                        }
                    }
                    Event::Suspended => {
                        #[cfg(target_os = "android")]
                        {
                            visualizer.suspend();
                        }
                    }
                    Event::Resumed => {
                        #[cfg(target_os = "android")]
                        {
                            if !initialized {
                                initialize_native_window(
                                    &event_loop_window_target,
                                    &mut window,
                                    self.desktop_dimensions,
                                );
                                visualizer.initialize(window.as_ref().unwrap());
                                initialized = true;
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
            });
        });
    }
}
pub(crate) fn initialize_native_window<T>(
    w_target: &EventLoopWindowTarget<T>,
    window: &mut Option<Window>,
    desktop_dimensions: Option<Area<DeviceContext>>,
) {
    let mut builder = WindowBuilder::new().with_resizable(false);
    #[cfg(not(target_os = "android"))]
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
    window.replace(builder.build(w_target).expect("window"));
}
