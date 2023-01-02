use crate::app::App;
use crate::canvas::Canvas;
use crate::options::LaunchOptions;
use crate::render;
use crate::render::{Render, RenderMode, Renderers};
use crate::text::TextRenderer;
use crate::theme::Theme;
use bevy_ecs::prelude::Resource;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;

#[derive(Resource)]
pub(crate) struct LauncherWindow(pub(crate) Window);
pub struct Launcher {
    compute: App,
    options: LaunchOptions,
    pub(crate) renderers: Renderers,
    pub(crate) theme: Theme,
    window: Option<LauncherWindow>,
    pub(crate) canvas: Option<Canvas>,
    event_loop: Option<EventLoop<()>>,
}
impl Launcher {
    pub fn new(compute: App, options: LaunchOptions) -> Self {
        Self {
            compute,
            options,
            renderers: Renderers::new(),
            theme: Theme::default(),
            window: None,
            canvas: None,
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
    pub fn attach_renderer<Renderer: Render + 'static>(&mut self) {
        let mut renderer = Renderer::renderer(self.canvas.as_ref().expect("no canvas attached"));
        renderer.instrument(&mut self.compute);
        self.insert_renderer(renderer);
    }
    fn insert_renderer<Renderer: Render + 'static>(&mut self, renderer: Renderer) {
        match Renderer::mode() {
            RenderMode::Opaque => {
                self.renderers
                    .opaque
                    .insert(Renderer::id(), Box::new(renderer));
            }
            RenderMode::Alpha => {
                self.renderers
                    .alpha
                    .insert(Renderer::id(), Box::new(renderer));
            }
        }
    }
    fn attach_event_loop(&mut self, event_loop: EventLoop<()>) {
        self.event_loop.replace(event_loop);
    }
    fn attach_window(&mut self, window: Window) {
        self.window.replace(LauncherWindow(window));
    }
    #[allow(dead_code)]
    fn detach_window(&mut self) -> Window {
        self.window.take().expect("no window attached").0
    }
    fn attach_canvas(&mut self, canvas: Canvas) {
        self.canvas.replace(canvas);
    }
    #[allow(dead_code)]
    fn detach_canvas(&mut self) -> Canvas {
        self.canvas.take().expect("no canvas attached")
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
                        self.attach_renderer::<TextRenderer>();
                    }
                },
                Event::WindowEvent {
                    window_id: _window_id,
                    event,
                } => match event {
                    WindowEvent::Resized(physical_size) => {
                        self.canvas
                            .as_mut()
                            .unwrap()
                            .adjust(physical_size.width, physical_size.height);
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
                        self.canvas
                            .as_mut()
                            .unwrap()
                            .adjust(new_inner_size.width, new_inner_size.height);
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
                    }
                }
                Event::MainEventsCleared => {
                    if self.compute.job.active() {
                        self.compute.job.exec();
                        self.window.as_ref().unwrap().0.request_redraw();
                    }
                    if self.compute.job.should_exit() {
                        control_flow.set_exit();
                    }
                }
                Event::RedrawRequested(_window_id) => {
                    if self.compute.job.active() {
                        render::extract(&mut self.renderers, &mut self.compute);
                        render::prepare(&mut self.renderers, self.canvas.as_ref().unwrap());
                        render::render(
                            &mut self.renderers,
                            self.canvas.as_ref().unwrap(),
                            &self.theme,
                        );
                    }
                }
                Event::RedrawEventsCleared => {
                    if self.compute.job.can_idle() {
                        control_flow.set_wait();
                    }
                }
                Event::LoopDestroyed => {
                    self.compute.job.teardown();
                }
            }
        });
    }
}
