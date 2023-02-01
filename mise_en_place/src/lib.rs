use bevy_ecs::prelude::{Resource, StageLabel, SystemStage};
use winit::dpi::PhysicalSize;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::EventLoop;

pub use job::Job;
pub use wasm_server::WasmServer;

use crate::extract::{Extract, ExtractFns, invoke_extract};
use crate::gfx::{GfxOptions, GfxSurface, GfxSurfaceConfiguration};
use crate::job::TaskLabel;
use crate::render::{invoke_render, Render, RenderFns, RenderPhase};
pub use crate::theme::Theme;
use crate::viewport::Viewport;
pub use crate::wasm_compiler::WasmCompiler;
use crate::window::{EngenWindow, Resize, ScaleFactor};

mod color;
mod coord;
mod extract;
mod gfx;
mod job;
mod render;
mod theme;
mod uniform;
mod viewport;
mod wasm_compiler;
mod wasm_server;
mod window;

#[derive(StageLabel)]
pub enum FrontEndStages {
    Startup,
    Initialize,
}

#[derive(StageLabel)]
pub enum BackendStages {
    Startup,
    Initialize,
    GfxSurfaceResize,
    Resize,
}

pub struct Engen {
    event_loop: Option<EventLoop<()>>,
    attachment_queue: Vec<Box<fn(&mut Engen)>>,
    pub(crate) render_fns: (RenderFns, RenderFns),
    pub(crate) extract_fns: ExtractFns,
    pub(crate) frontend: Job,
    pub(crate) backend: Job,
}

impl Engen {
    pub fn new() -> Self {
        Self {
            event_loop: None,
            attachment_queue: vec![],
            render_fns: (vec![], vec![]),
            extract_fns: vec![],
            frontend: {
                let mut job = Job::new();
                job.startup
                    .add_stage(FrontEndStages::Startup, SystemStage::parallel());
                job.main
                    .add_stage(FrontEndStages::Initialize, SystemStage::parallel());
                job
            },
            backend: {
                let mut job = Job::new();
                job.startup
                    .add_stage(BackendStages::Startup, SystemStage::parallel());
                job.main
                    .add_stage(BackendStages::Initialize, SystemStage::parallel());
                job.main.add_stage(BackendStages::GfxSurfaceResize, SystemStage::single(gfx::resize));
                job.main
                    .add_stage(BackendStages::Resize, SystemStage::parallel());
                job
            },
        }
    }
    pub fn add_render_attachment<RenderAttachment: Attach + Render + Extract + Resource>(
        &mut self,
    ) {
        self.attachment_queue
            .push(Box::new(RenderAttachment::attach));
        match RenderAttachment::phase() {
            RenderPhase::Opaque => self
                .render_fns
                .0
                .push(Box::new(invoke_render::<RenderAttachment>)),
            RenderPhase::Alpha => self
                .render_fns
                .1
                .push(Box::new(invoke_render::<RenderAttachment>)),
        }
        self.extract_fns
            .push(Box::new(invoke_extract::<RenderAttachment>));
    }
    pub(crate) fn invoke_attach<Attachment: Attach>(&mut self) {
        Attachment::attach(self);
    }
    pub fn launch<Job: Launch>(mut self) {
        #[cfg(not(target_arch = "wasm32"))] {
            self.event_loop.replace(EventLoop::new());
            Self::internal_launch(self);
        }

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async {
            use wasm_bindgen::{JsCast, prelude::*};
            use winit::platform::web::WindowExtWebSys;
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init().expect("could not initialize logger");
            let inner_size = |scale_factor: f64| {
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
            };
            let event_loop = EventLoop::new();
            let window = EngenWindow::new(&event_loop);
            let gfx = GfxSurface::new(&window, GfxOptions::web()).await;
            self.backend.container.insert_resource(gfx.0);
            self.backend.container.insert_resource(gfx.1);
            self.event_loop.replace(event_loop);
            let scale_factor = ScaleFactor::new(window.window_ref.scale_factor());
            window
                .window_ref
                .set_inner_size(inner_size(scale_factor.factor));
            {
                let w_window = window.window_ref.clone();
                let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
                    let scale_factor = w_window.scale_factor();
                    let size = inner_size(scale_factor);
                    w_window.set_inner_size(size);
                }) as Box<dyn FnMut(_)>);
                web_sys::window()
                    .expect("no web_sys window")
                    .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                    .unwrap();
                closure.forget();
            }
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(window.canvas()))
                        .ok()
                })
                .expect("couldn't append canvas to document body");
            self.backend.container.insert_resource(window);
            if let Err(error) =
                call_catch(&Closure::once_into_js(move || Self::internal_launch(self)))
            {
                let is_control_flow_exception =
                    error.dyn_ref::<js_sys::Error>().map_or(false, |e| {
                        e.message().includes("Using exceptions for control flow", 0)
                    });
                if !is_control_flow_exception {
                    web_sys::console::error_1(&error);
                }
            }
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(catch, js_namespace = Function, js_name = "prototype.call.call")]
                fn call_catch(this: &JsValue) -> Result<(), JsValue>;
            }
        });
    }
    fn resize_callback(&mut self, size: PhysicalSize<u32>, scale_factor: f64) {
        let resize_event = Resize::new((size.width, size.height).into(), scale_factor);
        self.frontend.container.send_event(resize_event);
        self.backend.container.send_event(resize_event);
    }
    fn internal_launch(mut self) {
        let event_loop = self.event_loop.take().expect("no event loop");
        event_loop.run(
            move |event, event_loop_window_target, control_flow| match event {
                Event::NewEvents(start_cause) => match start_cause {
                    StartCause::Init => {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let window = EngenWindow::new(event_loop_window_target);
                            let gfx = futures::executor::block_on(GfxSurface::new(
                                &window,
                                GfxOptions::native(),
                            ));
                            self.backend.container.insert_resource(window);
                            self.backend.container.insert_resource(gfx.0);
                            self.backend.container.insert_resource(gfx.1);
                        }
                        self.invoke_attach::<Resize>();
                        self.invoke_attach::<Viewport>();
                        self.invoke_attach::<Theme>();
                        self.process_attachment_queue();
                        self.frontend.exec(TaskLabel::Startup);
                        self.backend.exec(TaskLabel::Startup);
                    }
                    _ => {}
                },
                Event::WindowEvent {
                    window_id: _window_id,
                    event: w_event,
                } => match w_event {
                    WindowEvent::Resized(size) => {
                        let scale_factor = self
                            .backend
                            .container
                            .get_resource::<EngenWindow>()
                            .expect("no window")
                            .window_ref
                            .scale_factor();
                        self.resize_callback(size, scale_factor);
                    }
                    WindowEvent::ScaleFactorChanged {
                        new_inner_size,
                        scale_factor,
                    } => {
                        self.resize_callback(*new_inner_size, scale_factor);
                    }
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    }
                    _ => {}
                },
                Event::Suspended => {
                    if self.frontend.active() {
                        #[cfg(target_os = "android")]
                        {
                            let _ = self.backend.container.remove_resource::<GfxSurface>();
                        }
                        self.frontend.suspend();
                        self.backend.suspend();
                    }
                }
                Event::Resumed => {
                    if self.frontend.suspended() {
                        #[cfg(target_os = "android")]
                        {
                            let window = self
                                .backend
                                .container
                                .get_resource::<EngenWindow>()
                                .expect("no window");
                            let gfx = futures::executor::block_on(GfxSurface::new(
                                &window,
                                GfxOptions::native(),
                            ));
                            self.backend.container.insert_resource(gfx.0);
                            self.backend.container.insert_resource(gfx.1);
                        }
                        self.frontend.activate();
                        self.backend.activate();
                    }
                }
                Event::MainEventsCleared => {
                    if self.frontend.active() {
                        self.frontend.exec(TaskLabel::Main);
                    }
                    if self.frontend.should_exit() {
                        control_flow.set_exit();
                    }
                }
                Event::RedrawRequested(_) => {
                    if self.backend.active() {
                        extract::extract(&mut self);
                        self.backend.exec(TaskLabel::Main);
                        render::render(&mut self);
                    }
                }
                Event::RedrawEventsCleared => {
                    if self.backend.active() {
                        self.backend
                            .container
                            .get_resource::<EngenWindow>()
                            .expect("no window")
                            .window_ref
                            .as_ref()
                            .request_redraw();
                    }
                    if self.frontend.can_idle() && self.backend.can_idle() {
                        control_flow.set_wait();
                    }
                }
                Event::LoopDestroyed => {
                    self.frontend.exec(TaskLabel::Teardown);
                    self.backend.exec(TaskLabel::Teardown);
                }
                _ => {}
            },
        );
    }

    fn process_attachment_queue(&mut self) {
        let attachment_queue = self
            .attachment_queue
            .drain(..)
            .collect::<Vec<Box<fn(&mut Engen)>>>();
        for attachment_fn in attachment_queue {
            attachment_fn(self);
        }
    }
    pub fn compile_wasm(wasm_compiler: WasmCompiler) -> Option<WasmServer> {
        match wasm_compiler.compile() {
            Ok(_) => Some(WasmServer::new(wasm_compiler)),
            Err(_) => None,
        }
    }
}

pub trait Launch {
    fn setup(job: &mut Job);
}

pub trait Attach {
    fn attach(engen: &mut Engen);
}
