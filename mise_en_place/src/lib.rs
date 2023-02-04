use std::rc::Rc;

use bevy_ecs::prelude::{Resource, StageLabel, SystemStage};
use winit::dpi::PhysicalSize;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::Window;

pub use job::Job;
pub use wasm_server::DeliveryService;

pub use crate::color::Color;
use crate::coord::Coords;
pub use crate::coord::{
    Area, AreaAdjust, Depth, DepthAdjust, Position, PositionAdjust, ScaledSection, Section,
};
use crate::extract::{invoke_extract, Extract, ExtractFns};
use crate::gfx::{GfxOptions, GfxSurface, GfxSurfaceConfiguration};
use crate::job::TaskLabel;
use crate::render::{invoke_render, Render, RenderFns, RenderPhase};
pub use crate::text::{Text, TextBound, TextBundle, TextRenderer, TextScaleAlignment};
pub use crate::theme::Theme;
use crate::viewport::Viewport;
use crate::visibility::Visibility;
pub use crate::wasm_compiler::DeliveryTicket;
use crate::window::{Resize, ScaleFactor};

mod color;
mod coord;
mod extract;
mod gfx;
mod job;
mod render;
mod text;
mod theme;
mod uniform;
mod viewport;
mod visibility;
mod wasm_compiler;
mod wasm_server;
mod window;

#[derive(StageLabel)]
pub enum FrontEndStartupStages {
    Startup,
}

#[derive(StageLabel)]
pub enum FrontEndStages {
    First,
    Resize,
    Process,
    CoordAdjust,
    VisibilityPreparation,
    ResolveVisibility,
    Last,
}

#[derive(StageLabel)]
pub enum BackEndStartupStages {
    Startup,
    Setup,
}

#[derive(StageLabel)]
pub enum BackendStages {
    Initialize,
    GfxSurfaceResize,
    Resize,
    Prepare,
    Last,
}

pub struct Stove {
    event_loop: Option<EventLoop<()>>,
    attachment_queue: Vec<Box<fn(&mut Stove)>>,
    pub(crate) render_fns: (RenderFns, RenderFns),
    pub(crate) extract_fns: ExtractFns,
    pub(crate) frontend: Job,
    pub(crate) backend: Job,
    pub(crate) window: Option<Rc<Window>>,
}

impl Stove {
    pub fn new() -> Self {
        Self {
            event_loop: None,
            attachment_queue: vec![],
            render_fns: (vec![], vec![]),
            extract_fns: vec![],
            frontend: {
                let mut job = Job::new();
                job.startup
                    .add_stage(FrontEndStartupStages::Startup, SystemStage::parallel());
                job.main
                    .add_stage(FrontEndStages::First, SystemStage::parallel());
                job.main
                    .add_stage(FrontEndStages::Resize, SystemStage::parallel());
                job.main
                    .add_stage(FrontEndStages::Process, SystemStage::parallel());
                job.main
                    .add_stage(FrontEndStages::CoordAdjust, SystemStage::parallel());
                job.main.add_stage(
                    FrontEndStages::VisibilityPreparation,
                    SystemStage::parallel(),
                );
                job.main
                    .add_stage(FrontEndStages::ResolveVisibility, SystemStage::parallel());
                job.main
                    .add_stage(FrontEndStages::Last, SystemStage::parallel());
                job
            },
            backend: {
                let mut job = Job::new();
                job.startup
                    .add_stage(BackEndStartupStages::Startup, SystemStage::parallel());
                job.startup
                    .add_stage(BackEndStartupStages::Setup, SystemStage::parallel());
                job.main
                    .add_stage(BackendStages::Initialize, SystemStage::parallel());
                job.main.add_stage(
                    BackendStages::GfxSurfaceResize,
                    SystemStage::single(gfx::resize),
                );
                job.main
                    .add_stage(BackendStages::Resize, SystemStage::parallel());
                job.main
                    .add_stage(BackendStages::Prepare, SystemStage::parallel());
                job.main
                    .add_stage(BackendStages::Last, SystemStage::parallel());
                job
            },
            window: None,
        }
    }
    pub fn add_ingredient<Ingredient: Attach + Extract + Render + Resource>(&mut self) {
        self.attachment_queue.push(Box::new(Ingredient::attach));
        match Ingredient::phase() {
            RenderPhase::Opaque => self
                .render_fns
                .0
                .push(Box::new(invoke_render::<Ingredient>)),
            RenderPhase::Alpha => self
                .render_fns
                .1
                .push(Box::new(invoke_render::<Ingredient>)),
        }
        self.extract_fns
            .push(Box::new(invoke_extract::<Ingredient>));
    }
    pub(crate) fn invoke_attach<IngredientPreparation: Attach>(&mut self) {
        IngredientPreparation::attach(self);
    }
    pub fn cook<Recipe: Cook>(mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.event_loop.replace(EventLoop::new());
            Recipe::prepare(&mut self.frontend);
            self.apply_heat();
        }

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async {
            use wasm_bindgen::{prelude::*, JsCast};
            use winit::platform::web::WindowExtWebSys;
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init().expect("could not initialize logger");
            let event_loop = EventLoop::new();
            let window = Rc::new(Window::new(&event_loop).expect("could not create window"));
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(window.canvas()))
                        .ok()
                })
                .expect("couldn't append canvas to document body");
            self.event_loop.replace(event_loop);
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
            let scale_factor = ScaleFactor::new(window.scale_factor());
            self.backend.container.insert_resource(scale_factor);
            self.frontend.container.insert_resource(scale_factor);
            window.set_inner_size(inner_size(scale_factor.factor));
            {
                let w_window = window.clone();
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
            let gfx = GfxSurface::new(&window, GfxOptions::web()).await;
            self.backend.container.insert_resource(gfx.0);
            self.backend.container.insert_resource(gfx.1);
            self.window.replace(window);
            if let Err(error) = call_catch(&Closure::once_into_js(move || self.apply_heat())) {
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
    pub fn add_extraction<Extraction: Extract>(&mut self) {
        self.extract_fns
            .push(Box::new(invoke_extract::<Extraction>));
    }
    fn resize_callback(&mut self, size: PhysicalSize<u32>, scale_factor: f64) {
        let resize_event = Resize::new((size.width, size.height).into(), scale_factor);
        self.frontend.container.send_event(resize_event);
        self.backend.container.send_event(resize_event);
    }
    fn apply_heat(mut self) {
        let event_loop = self.event_loop.take().expect("no event loop");
        event_loop.run(
            move |event, event_loop_window_target, control_flow| match event {
                Event::NewEvents(start_cause) => match start_cause {
                    StartCause::Init => {
                        self.init_native_gfx(event_loop_window_target);
                        self.invoke_attach::<Coords>();
                        self.invoke_attach::<Resize>();
                        self.invoke_attach::<Viewport>();
                        self.invoke_attach::<Theme>();
                        self.invoke_attach::<Visibility>();
                        self.attach_from_queue();
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
                        let scale_factor = self.window.as_ref().expect("no window").scale_factor();
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
                            let window = self.window.as_ref().expect("no window");
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
                        self.window.as_ref().expect("no window").request_redraw();
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

    fn init_native_gfx(&mut self, event_loop_window_target: &EventLoopWindowTarget<()>) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let window = Rc::new(Window::new(event_loop_window_target).expect("no window"));
            let scale_factor = ScaleFactor::new(window.scale_factor());
            self.frontend.container.insert_resource(scale_factor);
            self.backend.container.insert_resource(scale_factor);
            let gfx = futures::executor::block_on(GfxSurface::new(&window, GfxOptions::native()));
            self.window.replace(window);
            self.backend.container.insert_resource(gfx.0);
            self.backend.container.insert_resource(gfx.1);
        }
    }

    fn attach_from_queue(&mut self) {
        let attachment_queue = self
            .attachment_queue
            .drain(..)
            .collect::<Vec<Box<fn(&mut Stove)>>>();
        for attach_fn in attachment_queue {
            attach_fn(self);
        }
    }
    pub fn order_delivery(delivery_ticket: DeliveryTicket) -> Option<DeliveryService> {
        match delivery_ticket.order() {
            Ok(_) => Some(DeliveryService::new(delivery_ticket)),
            Err(_) => None,
        }
    }
}

pub type Recipe = Job;

pub trait Cook {
    fn prepare(recipe: &mut Recipe);
}

pub trait Attach {
    fn attach(stove: &mut Stove);
}
