#![allow(unused, dead_code)]

use std::collections::HashMap;
use std::rc::Rc;

use bevy_ecs::prelude::{Resource, StageLabel, SystemStage};
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, StartCause, TouchPhase, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

pub use icon::IconKey;
pub use icon::IconMesh;
pub use icon::IconMeshAddRequest;
pub use job::Job;
pub use wasm_server::WasmServer;

pub use crate::color::Color;
use crate::coord::CoordPlugin;
pub use crate::coord::{
    Area, AreaAdjust, Depth, DepthAdjust, DeviceView, GpuArea, GpuPosition, Location, Numerical,
    Position, PositionAdjust, Section, UIView,
};
use crate::extract::{invoke_extract, Extract, ExtractFns};
use crate::gfx::{GfxOptions, GfxSurface};
pub use crate::icon::{
    ColorHooks, ColorInvert, Icon, IconBundle, IconPlugin, IconSize, IconVertex,
};
use crate::job::{Container, TaskLabel};
pub use crate::job::{Exit, Idle};
use crate::render::{invoke_render, Render, RenderFns, RenderPhase};
pub use crate::text::{
    PartitionMetadata, Text, TextBoundGuide, TextBundle, TextPartition, TextPlugin,
    TextScaleAlignment,
};
pub use crate::theme::Theme;
use crate::theme::ThemePlugin;
use crate::viewport::{Viewport, ViewportPlugin};
use crate::visibility::VisibilityPlugin;
pub use crate::visibility::{Visibility, VisibleBounds, VisibleSection};
pub use crate::wasm_compiler::WasmCompileDescriptor;
use crate::window::{Click, Finger, Resize, VirtualKeyboardAdapter, WindowPlugin};
pub use crate::window::{MouseAdapter, MouseButtonExpt, Orientation, ScaleFactor, TouchAdapter};

mod button;
mod color;
mod coord;
mod extract;
mod gfx;
mod icon;
mod index;
mod instance_tools;
mod job;
mod key;
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
    Initialization,
}

#[derive(StageLabel)]
pub enum FrontEndStages {
    First,
    Resize,
    Process,
    CoordAdjust,
    VisibilityPreparation,
    ResolveVisibility,
    PostProcess,
    Last,
}

#[derive(StageLabel)]
pub enum BackEndStartupStages {
    Startup,
    Setup,
    PostSetup,
}

#[derive(StageLabel)]
pub enum BackendStages {
    Initialize,
    GfxSurfaceResize,
    Resize,
    Prepare,
    Last,
}

pub struct EngenOptions {
    pub native_dimensions: Option<Area<DeviceView>>,
    pub theme: Theme,
}

impl EngenOptions {
    pub fn new() -> Self {
        Self {
            native_dimensions: None,
            theme: Theme::default(),
        }
    }
    pub fn with_native_dimensions<A: Into<Area<DeviceView>>>(mut self, dimensions: A) -> Self {
        self.native_dimensions.replace(dimensions.into());
        self
    }
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

pub struct Engen {
    event_loop: Option<EventLoop<()>>,
    attachment_queue: Vec<Box<fn(&mut Engen)>>,
    options: EngenOptions,
    pub(crate) render_fns: (RenderFns, RenderFns),
    pub(crate) extract_fns: ExtractFns,
    pub(crate) frontend: Job,
    pub(crate) backend: Job,
    pub(crate) window: Option<Rc<Window>>,
}

impl Engen {
    pub fn new(options: EngenOptions) -> Self {
        Self {
            event_loop: None,
            attachment_queue: vec![],
            options,
            render_fns: (vec![], vec![]),
            extract_fns: vec![],
            frontend: {
                let mut job = Job::new();
                job.startup
                    .add_stage(FrontEndStartupStages::Startup, SystemStage::parallel());
                job.startup.add_stage(
                    FrontEndStartupStages::Initialization,
                    SystemStage::parallel(),
                );
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
                    .add_stage(FrontEndStages::PostProcess, SystemStage::parallel());
                job.main
                    .add_stage(FrontEndStages::Last, SystemStage::parallel());
                job.main.add_stage_after(
                    FrontEndStages::Last,
                    "clear trackers",
                    SystemStage::single(Container::clear_trackers),
                );
                job
            },
            backend: {
                let mut job = Job::new();
                job.startup
                    .add_stage(BackEndStartupStages::Startup, SystemStage::parallel());
                job.startup
                    .add_stage(BackEndStartupStages::Setup, SystemStage::parallel());
                job.startup
                    .add_stage(BackEndStartupStages::PostSetup, SystemStage::parallel());
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
                job.main.add_stage_after(
                    BackendStages::Last,
                    "clear trackers",
                    SystemStage::single(Container::clear_trackers),
                );
                job
            },
            window: None,
        }
    }
    pub fn add_plugin<Plugin: Attach>(&mut self) {
        self.attachment_queue.push(Box::new(Plugin::attach));
    }
    pub fn add_renderer<Renderer: Extract + Render + Resource>(&mut self) {
        match Renderer::phase() {
            RenderPhase::Opaque => self.render_fns.0.push(Box::new(invoke_render::<Renderer>)),
            RenderPhase::Alpha => self.render_fns.1.push(Box::new(invoke_render::<Renderer>)),
        }
        self.extract_fns.push(Box::new(invoke_extract::<Renderer>));
    }
    pub(crate) fn invoke_attach<Attachment: Attach>(&mut self) {
        Attachment::attach(self);
    }
    pub fn launch<Launcher: Launch>(mut self) {
        Launcher::prepare(&mut self.frontend);
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.event_loop.replace(EventLoop::new());
            self.ignition();
        }

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async {
            use wasm_bindgen::{prelude::*, JsCast};
            use winit::platform::web::WindowExtWebSys;
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init().expect("could not initialize logger");
            let event_loop = EventLoop::new();
            let window = Rc::new(
                WindowBuilder::new()
                    .with_title("web engen")
                    .build(&event_loop)
                    .expect("could not create window"),
            );
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
                web_sys::window()
                    .expect("no web_sys window")
                    .screen()
                    .expect("could not get screen")
                    .orientation()
                    .add_event_listener_with_callback("onchange", closure.as_ref().unchecked_ref())
                    .unwrap();
                closure.forget();
            }
            let gfx = GfxSurface::new(&window, GfxOptions::web()).await;
            self.backend.container.insert_resource(gfx.0);
            self.backend.container.insert_resource(gfx.1);
            self.window.replace(window);
            if let Err(error) = call_catch(&Closure::once_into_js(move || self.ignition())) {
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
        self.frontend
            .container
            .insert_resource(ScaleFactor::new(scale_factor));
        self.backend
            .container
            .insert_resource(ScaleFactor::new(scale_factor));
    }
    fn ignition(mut self) {
        let event_loop = self.event_loop.take().expect("no event loop");
        event_loop.run(
            move |event, event_loop_window_target, control_flow| match event {
                Event::NewEvents(start_cause) => match start_cause {
                    StartCause::Init => {
                        self.init_native_gfx(event_loop_window_target);
                        self.invoke_attach::<CoordPlugin>();
                        self.invoke_attach::<WindowPlugin>();
                        self.invoke_attach::<ViewportPlugin>();
                        self.invoke_attach::<ThemePlugin>();
                        self.invoke_attach::<VisibilityPlugin>();
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
                    WindowEvent::Touch(touch) => {
                        match touch.phase {
                            TouchPhase::Started => {
                                let mut vkey = self
                                    .frontend
                                    .container
                                    .get_resource_mut::<VirtualKeyboardAdapter>()
                                    .expect("no vkeyboard");
                                if vkey.is_open() {
                                    vkey.close();
                                } else {
                                    vkey.open();
                                }
                            }
                            TouchPhase::Moved => {}
                            TouchPhase::Ended => {}
                            TouchPhase::Cancelled => {}
                        }
                        let mut touch_adapter = self
                            .frontend
                            .container
                            .get_resource_mut::<TouchAdapter>()
                            .expect("no touch adapter slot");
                        match touch.phase {
                            TouchPhase::Started => {
                                if touch_adapter.primary.is_none() {
                                    touch_adapter.primary.replace(touch.id as Finger);
                                }
                                touch_adapter.tracked.insert(
                                    touch.id as Finger,
                                    Click::new((touch.location.x, touch.location.y)),
                                );
                            }
                            TouchPhase::Moved => {
                                if let Some(click) =
                                    touch_adapter.tracked.get_mut(&(touch.id as Finger))
                                {
                                    click
                                        .current
                                        .replace((touch.location.x, touch.location.y).into());
                                }
                            }
                            TouchPhase::Ended => {
                                if let Some(click) =
                                    touch_adapter.tracked.get_mut(&(touch.id as Finger))
                                {
                                    click
                                        .end
                                        .replace((touch.location.x, touch.location.y).into());
                                }
                                if let Some(finger) = touch_adapter.primary {
                                    if finger == touch.id as Finger {
                                        touch_adapter.primary.take();
                                        let click = *touch_adapter
                                            .tracked
                                            .get(&finger)
                                            .expect("no tracked finger");
                                        touch_adapter.primary_end_event.replace((finger, click));
                                    }
                                }
                            }
                            TouchPhase::Cancelled => {
                                if let Some(finger) = touch_adapter.primary {
                                    if finger == touch.id as Finger {
                                        touch_adapter.primary.take();
                                    }
                                }
                                touch_adapter.tracked.remove(&(touch.id as Finger));
                            }
                        }
                    }
                    WindowEvent::CursorMoved {
                        device_id,
                        position,
                        ..
                    } => {
                        let mut mouse_adapter = self
                            .frontend
                            .container
                            .get_resource_mut::<MouseAdapter>()
                            .expect("no mouse adapter");
                        mouse_adapter.location.replace(Position::<DeviceView>::new(
                            position.x as f32,
                            position.y as f32,
                        ));
                    }
                    WindowEvent::MouseInput {
                        device_id,
                        state,
                        button,
                        ..
                    } => {
                        let mut mouse_adapter = self
                            .frontend
                            .container
                            .get_resource_mut::<MouseAdapter>()
                            .expect("no mouse adapter");
                        let mouse_location = mouse_adapter.location;
                        let mut valid_releases = HashMap::new();
                        match state {
                            ElementState::Pressed => {
                                if let Some(click) = mouse_adapter.tracked_buttons.get_mut(&button)
                                {
                                    click.current = mouse_location;
                                } else {
                                    if let Some(location) = mouse_location {
                                        mouse_adapter
                                            .tracked_buttons
                                            .insert(button, Click::new(location));
                                    }
                                }
                            }
                            ElementState::Released => {
                                if let Some(click) = mouse_adapter.tracked_buttons.get_mut(&button)
                                {
                                    /* need limiter here to only track if was pressed - cache */
                                    click.end = mouse_location;
                                    if let Some(location) = mouse_location {
                                        valid_releases.insert(button, *click);
                                    }
                                }
                            }
                        }
                        for (button, click) in valid_releases {
                            mouse_adapter.valid_releases.insert(button, click);
                        }
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
            let mut builder = WindowBuilder::new().with_title("native engen");
            if let Some(native_dimensions) = self.options.native_dimensions {
                builder = builder.with_inner_size(PhysicalSize::new(
                    native_dimensions.width,
                    native_dimensions.height,
                ));
            }
            let window = Rc::new(builder.build(event_loop_window_target).expect("no window"));
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
            .collect::<Vec<Box<fn(&mut Engen)>>>();
        for attach_fn in attachment_queue {
            attach_fn(self);
        }
    }
}

pub trait Launch {
    fn prepare(job: &mut Job);
}

pub trait Attach {
    fn attach(engen: &mut Engen);
}
