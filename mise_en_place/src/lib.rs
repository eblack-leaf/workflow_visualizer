use std::rc::Rc;

use bevy_ecs::prelude::{Resource, StageLabel, SystemStage};
use winit::dpi::PhysicalSize;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;

pub use job::RecipeDirections;
pub use wasm_server::DeliveryService;

use crate::extract::{Season, Spices, add_seasoning};
use crate::gfx::{GfxOptions, Pan, Duvet};
use crate::job::TaskLabel;
use crate::render::{saute_ingredient, Saute, SauteDirections, SautePhase};
pub use crate::theme::Butter;
use crate::viewport::Spatula;
pub use crate::wasm_compiler::DeliveryTicket;
use crate::window::{Resize, ScaleFactor};

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

pub struct Stove {
    event_loop: Option<EventLoop<()>>,
    ingredient_preparation_queue: Vec<Box<fn(&mut Stove)>>,
    pub(crate) render_fns: (SauteDirections, SauteDirections),
    pub(crate) spices: Spices,
    pub(crate) frontend: RecipeDirections,
    pub(crate) backend: RecipeDirections,
    pub(crate) window: Option<Rc<Window>>,
}

impl Stove {
    pub fn new() -> Self {
        Self {
            event_loop: None,
            ingredient_preparation_queue: vec![],
            render_fns: (vec![], vec![]),
            spices: vec![],
            frontend: {
                let mut job = RecipeDirections::new();
                job.startup
                    .add_stage(FrontEndStages::Startup, SystemStage::parallel());
                job.main
                    .add_stage(FrontEndStages::Initialize, SystemStage::parallel());
                job
            },
            backend: {
                let mut job = RecipeDirections::new();
                job.startup
                    .add_stage(BackendStages::Startup, SystemStage::parallel());
                job.main
                    .add_stage(BackendStages::Initialize, SystemStage::parallel());
                job.main.add_stage(
                    BackendStages::GfxSurfaceResize,
                    SystemStage::single(gfx::resize),
                );
                job.main
                    .add_stage(BackendStages::Resize, SystemStage::parallel());
                job
            },
            window: None,
        }
    }
    pub fn add_ingredient<Ingredient: Preparation + Season + Saute + Resource>(
        &mut self,
    ) {
        self.ingredient_preparation_queue
            .push(Box::new(Ingredient::prepare));
        match Ingredient::phase() {
            SautePhase::Opaque => self
                .render_fns
                .0
                .push(Box::new(saute_ingredient::<Ingredient>)),
            SautePhase::Alpha => self
                .render_fns
                .1
                .push(Box::new(saute_ingredient::<Ingredient>)),
        }
        self.spices
            .push(Box::new(add_seasoning::<Ingredient>));
    }
    pub(crate) fn prepare<IngredientPreparation: Preparation>(&mut self) {
        IngredientPreparation::prepare(self);
    }
    pub fn cook<Recipe: Cook>(mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.event_loop.replace(EventLoop::new());
            self.apply_heat();
        }

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async {
            use wasm_bindgen::{JsCast, prelude::*};
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
            let gfx = Pan::new(&window, GfxOptions::web()).await;
            self.backend.container.insert_resource(gfx.0);
            self.backend.container.insert_resource(gfx.1);
            self.window.replace(window);
            if let Err(error) =
                call_catch(&Closure::once_into_js(move || self.apply_heat()))
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
    fn change_pan(&mut self, size: PhysicalSize<u32>, scale_factor: f64) {
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
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let window = Rc::new(Window::new(event_loop_window_target).expect("no window"));
                            let gfx = futures::executor::block_on(Pan::new(
                                &window,
                                GfxOptions::native(),
                            ));
                            self.window.replace(window);
                            self.backend.container.insert_resource(gfx.0);
                            self.backend.container.insert_resource(gfx.1);
                        }
                        self.prepare::<Resize>();
                        self.prepare::<Spatula>();
                        self.prepare::<Butter>();
                        self.prepare_ingredients_from_queue();
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
                            .window.as_ref().expect("no window")
                            .scale_factor();
                        self.change_pan(size, scale_factor);
                    }
                    WindowEvent::ScaleFactorChanged {
                        new_inner_size,
                        scale_factor,
                    } => {
                        self.change_pan(*new_inner_size, scale_factor);
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
                            let _ = self.backend.container.remove_resource::<Pan>();
                        }
                        self.frontend.suspend();
                        self.backend.suspend();
                    }
                }
                Event::Resumed => {
                    if self.frontend.suspended() {
                        #[cfg(target_os = "android")]
                        {
                            let window = self.window.take().expect("no window");
                            let gfx = futures::executor::block_on(Pan::new(
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
                        extract::season(&mut self);
                        self.backend.exec(TaskLabel::Main);
                        render::saute(&mut self);
                    }
                }
                Event::RedrawEventsCleared => {
                    if self.backend.active() {
                        self.window.as_ref()
                            .expect("no window")
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

    fn prepare_ingredients_from_queue(&mut self) {
        let ingredient_preparation_queue = self
            .ingredient_preparation_queue
            .drain(..)
            .collect::<Vec<Box<fn(&mut Stove)>>>();
        for ingredient_preparation in ingredient_preparation_queue {
            ingredient_preparation(self);
        }
    }
    pub fn order_delivery(togo_ticket: DeliveryTicket) -> Option<DeliveryService> {
        match togo_ticket.order() {
            Ok(_) => Some(DeliveryService::new(togo_ticket)),
            Err(_) => None,
        }
    }
}

pub trait Cook {
    fn recipe(recipe_directions: &mut RecipeDirections);
}

pub trait Preparation {
    fn prepare(stove: &mut Stove);
}
