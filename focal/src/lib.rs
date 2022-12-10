#![allow(dead_code, unused)]

use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;

use crate::canvas::Canvas;
pub use crate::job::Job;
use crate::viewport::Viewport;

mod canvas;
mod coord;
mod job;
mod render;
mod uniform;
mod viewport;
#[derive(Clone)]
pub struct VisualizerOptions {
    pub backends: wgpu::Backends,
    pub power_preferences: wgpu::PowerPreference,
    pub force_fallback_adapter: bool,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
    pub present_mode: wgpu::PresentMode,
}

impl VisualizerOptions {
    pub fn native() -> Self {
        Self {
            backends: wgpu::Backends::PRIMARY,
            power_preferences: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
            present_mode: wgpu::PresentMode::Fifo,
        }
    }
    pub fn web() -> Self {
        let mut options = Self::native();
        options.backends = wgpu::Backends::all();
        options.limits = wgpu::Limits::downlevel_webgl2_defaults();
        return options;
    }
}

pub struct Visualizer {
    pub options: VisualizerOptions,
    pub event_loop: Option<EventLoop<()>>,
    pub window: Option<Window>,
    pub canvas: Option<Canvas>,
    pub viewport: Option<Viewport>,
}

impl Visualizer {
    pub fn new(options: Option<VisualizerOptions>) -> Self {
        Self {
            options: options.unwrap_or(VisualizerOptions::native()),
            event_loop: None,
            window: None,
            canvas: None,
            viewport: None,
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub async fn web(options: Option<VisualizerOptions>) -> Self {
        let mut visualizer = Visualizer::new(Some(options.unwrap_or(VisualizerOptions::web())));
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
        visualizer.attach_canvas(visualizer.create_canvas(&window).await);
        visualizer.attach_window(window);
        visualizer.attach_event_loop(event_loop);
        visualizer
    }
    pub fn set_options(&mut self, options: VisualizerOptions) {
        self.options = options;
    }
    fn attach_window(&mut self, window: Window) {
        self.window = Some(window);
    }
    fn detach_window(&mut self) -> Window {
        self.window.take().expect("no window")
    }
    fn attach_canvas(&mut self, canvas: Canvas) {
        self.canvas = Option::from(canvas);
    }
    fn detach_canvas(&mut self) -> Canvas {
        self.canvas.take().expect("no canvas")
    }
    fn attach_event_loop(&mut self, event_loop: EventLoop<()>) {}
    async fn create_canvas(&self, window: &Window) -> Canvas {
        Canvas::new(window, self.options.clone()).await
    }
    fn adjust_canvas(&mut self, width: u32, height: u32) {
        let mut canvas = self.canvas.as_mut().unwrap();
        canvas.surface_configuration.width = width;
        canvas.surface_configuration.height = height;
        canvas
            .surface
            .configure(&canvas.device, &canvas.surface_configuration);
        self.viewport
            .as_mut()
            .unwrap()
            .adjust(&canvas.device, &canvas.queue, width, height);
    }
    fn render(&mut self) {}
    pub fn launch(mut self, mut job: Job) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.event_loop = Some(EventLoop::new());
        }
        let event_loop = self.event_loop.take().unwrap();
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
                            self.attach_canvas(futures::executor::block_on(
                                self.create_canvas(&window),
                            ));
                            self.attach_window(window);
                        }
                        job.startup();
                        let canvas = self.canvas.as_ref().unwrap();
                        let configuration = &canvas.surface_configuration;
                        self.viewport = Option::from(Viewport::new(
                            &canvas.device,
                            (configuration.width, configuration.height).into(),
                        ));
                    }
                },
                Event::WindowEvent { window_id, event } => match event {
                    WindowEvent::Resized(physical_size) => {
                        self.adjust_canvas(physical_size.width, physical_size.height);
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
                        scale_factor,
                        new_inner_size,
                    } => {
                        self.adjust_canvas(new_inner_size.width, new_inner_size.height);
                    }
                    WindowEvent::ThemeChanged(_) => {}
                    WindowEvent::Occluded(_) => {}
                },
                Event::DeviceEvent { .. } => {}
                Event::UserEvent(_) => {}
                Event::Suspended => {
                    #[cfg(target_os = "android")]
                    {
                        let _ = self.detach_canvas();
                    }
                    job.suspend();
                }
                Event::Resumed => {
                    #[cfg(target_os = "android")]
                    {
                        let window = self.detach_window();
                        self.attach_canvas(futures::executor::block_on(
                            self.create_canvas(&window),
                        ));
                        self.attach_window(window);
                    }
                    job.activate();
                }
                Event::MainEventsCleared => {
                    if job.active() {
                        job.exec();
                    }
                    if job.should_exit() {
                        control_flow.set_exit();
                    }
                    self.window.as_ref().unwrap().request_redraw();
                }
                Event::RedrawRequested(_window_id) => {
                    self.render();
                }
                Event::RedrawEventsCleared => {
                    if job.can_idle() {
                        control_flow.set_wait();
                    }
                }
                Event::LoopDestroyed => {
                    job.teardown();
                }
            }
        });
    }
}
