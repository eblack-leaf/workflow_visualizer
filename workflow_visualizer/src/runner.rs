use crate::{Area, DeviceContext, Visualizer};
use std::fmt::Debug;
use std::future::Future;
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
pub struct NativeRunner<T: Workflow + Send + 'static> {
    pub event_loop: Option<EventLoop<T::Response>>,
    pub window: Option<Window>,
    pub desktop_dimensions: Option<Area<DeviceContext>>,
    initialized: bool,
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
impl<T: Workflow + Send + 'static> NativeRunner<T> {
    #[cfg(target_os = "android")]
    pub fn android(android_app: AndroidApp) -> Self {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        Self {
            event_loop: Some(
                EventLoopBuilder::<T::Response>::with_user_event()
                    .with_android_app(android_app)
                    .build(),
            ),
            window: None,
            desktop_dimensions: None,
            initialized: false,
        }
    }
    pub fn desktop<A: Into<Area<DeviceContext>>>(dimensions: A) -> Self {
        Self {
            event_loop: Some(EventLoopBuilder::<T::Response>::with_user_event().build()),
            window: None,
            desktop_dimensions: Some(dimensions.into()),
            initialized: false,
        }
    }
    pub(crate) fn initialize_window(&mut self, w_target: &EventLoopWindowTarget<T::Response>) {
        let mut builder = WindowBuilder::new().with_resizable(false);
        #[cfg(not(target_os = "android"))]
        {
            let dim = self.desktop_dimensions.expect("desktop_dimensions");
            builder = builder.with_inner_size(PhysicalSize::new(dim.width, dim.height));
        }
        self.window
            .replace(builder.build(w_target).expect("window"));
    }
    pub fn run<EngenRunner, EngenRunnerFut>(
        mut self,
        mut visualizer: Visualizer,
        engen_runner: EngenRunner,
    ) where
        EngenRunner:
            FnOnce(Responder<T::Response>, Receiver<T::Action>) -> EngenRunnerFut + Send + 'static,
        EngenRunnerFut: Future<Output = ()> + Send + 'static,
    {
        let tokio_runtime = tokio::runtime::Runtime::new().expect("tokio runtime");
        tokio_runtime.block_on(async {
            let event_loop = self.event_loop.take().expect("event loop");
            let (sender, receiver): (
                tokio::sync::mpsc::UnboundedSender<T::Action>,
                tokio::sync::mpsc::UnboundedReceiver<T::Action>,
            ) = tokio::sync::mpsc::unbounded_channel();
            let proxy = event_loop.create_proxy();
            tokio::task::spawn(
                async move { engen_runner(Responder(proxy), Receiver(receiver)).await },
            );
            event_loop.run(move |event, event_loop_window_target, control_flow| {
                control_flow.set_wait();
                match event {
                    Event::NewEvents(cause) => match cause {
                        StartCause::Init => {
                            #[cfg(not(target_os = "android"))]
                            {
                                self.initialize_window(&event_loop_window_target);
                                visualizer.initialize(self.window.as_ref().unwrap());
                                self.initialized = true;
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
                            let scale_factor = self.window.as_ref().unwrap().scale_factor();
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
                        if visualizer.job.active() && self.initialized {
                            self.window.as_ref().unwrap().request_redraw();
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
                            if !self.initialized {
                                self.initialize_window(&event_loop_window_target);
                                visualizer.initialize(self.window.as_ref().unwrap());
                                self.initialized = true;
                            } else {
                                visualizer.resume(self.window.as_ref().unwrap());
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
