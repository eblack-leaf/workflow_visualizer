use crate::canvas::Canvas;
use crate::render::{Render, RenderPhase, Renderers};
use crate::theme::Theme;
use crate::{run, LaunchOptions, Task};
use bevy_ecs::prelude::Resource;
use winit::event_loop::EventLoop;
use winit::window::Window;

#[derive(Resource)]
pub(crate) struct LauncherWindow(pub(crate) Window);
pub struct Launcher {
    pub(crate) compute: Task,
    pub(crate) options: LaunchOptions,
    pub(crate) renderers: Renderers,
    pub(crate) theme: Theme,
    pub(crate) window: Option<LauncherWindow>,
    pub(crate) canvas: Option<Canvas>,
    pub(crate) event_loop: Option<EventLoop<()>>,
}
impl Launcher {
    pub fn new(compute: Task, options: LaunchOptions) -> Self {
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
                run::run(self);
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.attach_event_loop(EventLoop::new());
            run::run(self);
        }
    }
    pub fn attach_renderer<Renderer: Render + 'static>(&mut self) {
        let mut renderer = Renderer::renderer(self.canvas.as_ref().expect("no canvas attached"));
        renderer.instrument(&mut self.compute);
        self.insert_renderer(renderer);
    }
    fn insert_renderer<Renderer: Render + 'static>(&mut self, renderer: Renderer) {
        match Renderer::phase() {
            RenderPhase::Opaque => {
                self.renderers
                    .opaque
                    .insert(Renderer::id(), Box::new(renderer));
            }
            RenderPhase::Alpha => {
                self.renderers
                    .alpha
                    .insert(Renderer::id(), Box::new(renderer));
            }
        }
    }
    fn attach_event_loop(&mut self, event_loop: EventLoop<()>) {
        self.event_loop.replace(event_loop);
    }
    pub(crate) fn attach_window(&mut self, window: Window) {
        self.window.replace(LauncherWindow(window));
    }
    #[allow(dead_code)]
    fn detach_window(&mut self) -> Window {
        self.window.take().expect("no window attached").0
    }
    pub(crate) fn attach_canvas(&mut self, canvas: Canvas) {
        self.canvas.replace(canvas);
    }
    #[allow(dead_code)]
    pub(crate) fn detach_canvas(&mut self) -> Canvas {
        self.canvas.take().expect("no canvas attached")
    }
}
