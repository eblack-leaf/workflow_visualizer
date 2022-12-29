use bevy_ecs::prelude::{Resource, SystemStage};
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;
use crate::app::App;
use crate::canvas::Canvas;
use crate::options::LaunchOptions;
use crate::{canvas};
#[derive(Resource)]
pub(crate) struct LauncherWindow(pub(crate) Window);
pub struct Launcher {
    pub compute: App,
    pub render: App,
    pub options: LaunchOptions,
    event_loop: Option<EventLoop<()>>,
}
impl Launcher {
    pub fn new(compute: App, options: LaunchOptions) -> Self {
        Self {
            compute,
            render: App::new(),
            options,
            event_loop: None,
        }
    }
    pub fn launch(mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(async {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init().expect("could not initialize logger");
                let event_loop = EventLoop::new();
                let window = Window::new(&event_loop).expect("could not create window");
                use winit::platform::web::WindowExtWebSys;
                web_sys::window()
                    .and_then(|win| win.document())
                    .and_then(|doc| doc.body())
                    .and_then(|body| {
                        body.append_child(&web_sys::Element::from(window.canvas()))
                            .ok()
                    })
                    .expect("couldn't append canvas to document body");
                self.attach_canvas(Canvas::new(&window, self.options.clone()).await);
                self.attach_window(window);
                self.attach_event_loop(event_loop);
                self.run();
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.attach_event_loop(EventLoop::new());
            self.run();
        }
    }
    fn attach_event_loop(&mut self, event_loop: EventLoop<()>) {
        self.event_loop = Some(event_loop);
    }
    fn attach_window(&mut self, window: Window) {
        self.render.job.container.insert_resource(LauncherWindow(window));
    }
    #[allow(dead_code)]
    fn detach_window(&mut self) -> Window {
        self.render.job.container.remove_resource::<LauncherWindow>().expect("no window attached").0
    }
    fn attach_canvas(&mut self, canvas: Canvas) {
        self.render.job.container.insert_resource(canvas);
    }
    #[allow(dead_code)]
    fn detach_canvas(&mut self) -> Canvas {
        self.render.job.container.remove_resource::<Canvas>().expect("no canvas attached")
    }
    fn run(mut self) {
        let event_loop = self.event_loop.take().expect("no event loop provided");
        event_loop.run(move |event, _event_loop_window_target, control_flow| {
                control_flow.set_poll();
                match event {
                    Event::NewEvents(start_cause) => match start_cause {
                        StartCause::ResumeTimeReached { .. } => {}
                        StartCause::WaitCancelled { .. } => {}
                        StartCause::Poll => {}
                        StartCause::Init => {
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                let window = Window::new(_event_loop_window_target)
                                    .expect("could not create window");
                                self.attach_canvas(futures::executor::block_on(Canvas::new(
                                    &window,
                                    self.options.clone(),
                                )));
                                self.attach_window(window);
                            }
                            self.compute.job.startup();
                            self.render.job.startup();
                        }
                    },
                    Event::WindowEvent { window_id: _window_id, event } => match event {
                        WindowEvent::Resized(physical_size) => {
                            canvas::adjust(&mut self, physical_size.width, physical_size.height);
                        }
                        WindowEvent::Moved(_) => {}
                        WindowEvent::CloseRequested => {
                            control_flow.set_exit();
                        }
                        WindowEvent::Destroyed => {}
                        WindowEvent::DroppedFile(_) => {}
                        WindowEvent::HoveredFile(_) => {}
                        WindowEvent::HoveredFileCancelled => {}
                        WindowEvent::ReceivedCharacter(_) => {}
                        WindowEvent::Focused(_) => {}
                        WindowEvent::KeyboardInput { .. } => {}
                        WindowEvent::ModifiersChanged(_) => {}
                        WindowEvent::Ime(_) => {}
                        WindowEvent::CursorMoved { .. } => {}
                        WindowEvent::CursorEntered { .. } => {}
                        WindowEvent::CursorLeft { .. } => {}
                        WindowEvent::MouseWheel { .. } => {}
                        WindowEvent::MouseInput { .. } => {}
                        WindowEvent::TouchpadPressure { .. } => {}
                        WindowEvent::AxisMotion { .. } => {}
                        WindowEvent::Touch(_) => {}
                        WindowEvent::ScaleFactorChanged {
                            scale_factor: _scale_factor,
                            new_inner_size,
                        } => {
                            canvas::adjust(&mut self, new_inner_size.width, new_inner_size.height);
                        }
                        WindowEvent::ThemeChanged(_) => {}
                        WindowEvent::Occluded(_) => {}
                    },
                    Event::DeviceEvent { .. } => {}
                    Event::UserEvent(_) => {}
                    Event::Suspended => {
                        if self.compute.job.active() {
                            #[cfg(target_os = "android")]
                            {
                                let _ = gfx.detach_canvas();
                            }
                            self.compute.job.suspend();
                            self.render.job.suspend();
                        }
                    }
                    Event::Resumed => {
                        if self.compute.job.suspended() {
                            #[cfg(target_os = "android")]
                            {
                                let window = self.detach_window();
                                self.attach_canvas(futures::executor::block_on(Canvas::new(
                                    &window,
                                    self.options.clone(),
                                )));
                                self.attach_window(window);
                            }
                            self.compute.job.activate();
                            self.render.job.activate();
                        }
                    }
                    Event::MainEventsCleared => {
                        if self.compute.job.active() {
                            self.compute.job.exec();
                            self.render.job.container.get_resource::<LauncherWindow>().unwrap().0.request_redraw();
                        }
                        if self.compute.job.should_exit() || self.render.job.should_exit() {
                            control_flow.set_exit();
                        }
                    }
                    Event::RedrawRequested(_window_id) => {
                        if self.compute.job.active() && self.render.job.active() {
                            self.render.job.exec();
                        }
                    }
                    Event::RedrawEventsCleared => {
                        if self.compute.job.can_idle() && self.render.job.can_idle() {
                            control_flow.set_wait();
                        }
                    }
                    Event::LoopDestroyed => {
                        self.compute.job.teardown();
                        self.render.job.teardown();
                    }
                }
            });
    }
}