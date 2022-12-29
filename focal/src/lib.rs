#![allow(dead_code, unused)]

use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;

use crate::canvas::Canvas;
pub use crate::job::Job;
use crate::render::{surface_texture, Render};
use crate::run::{run, web_run};
use crate::theme::Theme;
use crate::viewport::Viewport;

mod canvas;
mod color;
mod coord;
mod instance_coordinator;
mod job;
mod render;
mod run;
mod text;
mod theme;
mod uniform;
mod viewport;

#[derive(Clone)]
pub struct GfxOptions {
    pub backends: wgpu::Backends,
    pub power_preferences: wgpu::PowerPreference,
    pub force_fallback_adapter: bool,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
    pub present_mode: wgpu::PresentMode,
}

impl GfxOptions {
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
    pub fn web_align(mut self) -> Self {
        self.backends = wgpu::Backends::all();
        self.limits = wgpu::Limits::downlevel_webgl2_defaults();
        return self;
    }
}
pub struct Gfx {
    pub options: GfxOptions,
    pub event_loop: Option<EventLoop<()>>,
    pub window: Option<Window>,
    pub canvas: Option<Canvas>,
    pub viewport: Option<Viewport>,
    pub theme: Theme,
    pub render_implementors: Vec<Box<dyn Render>>,
}

impl Gfx {
    pub fn new() -> Self {
        Self {
            options: GfxOptions::native(),
            event_loop: None,
            window: None,
            canvas: None,
            viewport: None,
            theme: Theme::default(),
            render_implementors: Vec::new(),
        }
    }
    pub fn set_options(&mut self, options: GfxOptions) {
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
    fn attach_event_loop(&mut self, event_loop: EventLoop<()>) {
        self.event_loop = Some(event_loop);
    }
    pub fn attach_render_implementor(&mut self, implementor: Box<dyn Render>) {
        self.render_implementors.push(implementor);
    }
    pub fn launch(mut self, mut job: Job) {
        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(web_run(self, job));
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.attach_event_loop(EventLoop::new());
            run(self, job);
        }
    }
}
