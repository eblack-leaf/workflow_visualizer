use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};
use winit::dpi::PhysicalSize;
use std::rc::Rc;
use bevy_ecs::prelude::Resource;
use crate::area::Area;
use crate::{DeviceContext, GfxOptions};
use crate::focus::FocusAttachment;
use crate::gfx::{GfxStack, GfxSurface};
use crate::job::{Job, TaskLabel};
use crate::orientation::OrientationAttachment;
use crate::render::{extract, Extract, ExtractFns, invoke_extract, invoke_render, render, Render, RenderFns, RenderPhase};
use crate::scale_factor::ScaleFactor;
use crate::theme::{Theme, ThemeAttachment};
use crate::time::TimerAttachment;
use crate::viewport::ViewportAttachment;
use crate::visibility::VisibilityAttachment;
use crate::window::{WindowAttachment, WindowResize};

pub struct Engen {
    pub options: EngenOptions,
    pub(crate) frontend: Job,
    pub(crate) backend: Job,
    event_loop:Option<EventLoop<()>>,
    window: Option<Rc<Window>>,
    pub(crate) render_fns: (RenderFns, RenderFns),
    pub(crate) extract_fns: ExtractFns,
    attachment_queue: Vec<Attachment>,
}
impl Engen {
    fn ignite(mut self) {
        let event_loop = self.event_loop.take().expect("no event loop provided");
        event_loop.run(move |event, event_loop_window_target, control_flow| match event {
            Event::NewEvents(start_cause) => match start_cause {
                StartCause::Init => {
                    #[cfg(not(target_arch = "wasm32"))]
                    futures::executor::block_on(self.init_gfx(&event_loop_window_target));
                    self.invoke_attach::<TimerAttachment>();
                    self.invoke_attach::<OrientationAttachment>();
                    self.invoke_attach::<WindowAttachment>();
                    self.invoke_attach::<ViewportAttachment>();
                    self.invoke_attach::<FocusAttachment>();
                    self.invoke_attach::<VisibilityAttachment>();
                    self.attach_from_queue();
                    self.frontend.exec(TaskLabel::Startup);
                    self.backend.exec(TaskLabel::Startup);
                }
                _ => {}
            }
            Event::WindowEvent {window_id: _window_id, event: w_event } => match w_event {
                WindowEvent::Resized(size) => {
                    let scale_factor = self.window.as_ref().unwrap().scale_factor();
                    self.resize_callback(size, scale_factor);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, scale_factor} => {
                    self.resize_callback(*new_inner_size, scale_factor);
                }
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                _ => {}
            }
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
                            self.options.gfx_options.clone(),
                        ));
                        self.attach_gfx(gfx);
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
                    extract(&mut self);
                    self.backend.exec(TaskLabel::Main);
                    render(&mut self);
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
        });
    }
}
impl Engen {
    pub fn launch<Launchable: Launch>(event_loop: EventLoop<()>) {
        let options = Launchable::options();
        let mut engen = Engen::new(options);
        engen.invoke_attach::<ThemeAttachment>();
        Launchable::preparation(&mut engen.frontend);
        engen.event_loop.replace(event_loop);
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(engen.web_ignite());
        #[cfg(not(target_arch = "wasm32"))]
        engen.ignite();
    }
    pub fn add_attachment<Attached: Attach>(&mut self) {
        self.attachment_queue.push(Attachment::using::<Attached>());
    }
    pub fn add_renderer<Renderer: Extract + Render + Resource>(&mut self) {
        match Renderer::phase() {
            RenderPhase::Opaque => self.render_fns.0.push(Box::new(invoke_render::<Renderer>)),
            RenderPhase::Alpha(_order) => self.render_fns.1.push(Box::new(invoke_render::<Renderer>)),
        }
        self.extract_fns.push(Box::new(invoke_extract::<Renderer>));
    }
    pub fn add_extraction<Extraction: Extract>(&mut self) {
        self.extract_fns
            .push(Box::new(invoke_extract::<Extraction>));
    }
}
impl Engen {
    fn new(options: EngenOptions) -> Self {
        Self {
            frontend: Job::new(),
            backend: Job::new(),
            event_loop: None,
            window: None,
            options,
            render_fns: (vec![], vec![]),
            extract_fns: vec![],
            attachment_queue: vec![],
        }
    }
    fn invoke_attach<Attachment: Attach>(&mut self) {
        Attachment::attach(self);
    }
    fn attach_from_queue(&mut self) {
        let attachment_queue = self.attachment_queue.drain(..).collect::<Vec<Attachment>>();
        for attach_fn in attachment_queue {
            attach_fn.0(self);
        }
    }
    async fn init_gfx(&mut self, event_loop_window_target: &EventLoopWindowTarget<()>) {
        let mut builder = WindowBuilder::new().with_title("workflow_visualizer");
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(native_dimensions) = self.options.native_dimensions {
            builder = builder.with_inner_size(PhysicalSize::new(
                native_dimensions.width,
                native_dimensions.height,
            ));
        }
        let window = Rc::new(builder.build(event_loop_window_target).expect("no window"));
        self.attach_scale_factor(window.scale_factor());
        #[cfg(not(target_arch = "wasm32"))]
            let gfx_options = self.options.gfx_options.clone().unwrap_or(GfxOptions::native());
        #[cfg(target_arch = "wasm32")]
            let gfx_options = self.options.gfx_options.clone().unwrap_or(GfxOptions::web());
        #[cfg(target_arch = "wasm32")] {
            use winit::platform::web::WindowExtWebSys;
            Self::add_web_canvas(&window);
            window.set_inner_size(Self::window_dimensions(window.scale_factor()));
            Self::web_resizing(&window);
        }
        let gfx = GfxSurface::new(window.as_ref(), gfx_options).await;
        self.window.replace(window);
        self.attach_gfx(gfx);

    }
    fn attach_gfx(&mut self, gfx: GfxStack) {
        self.backend.container.insert_resource(gfx.0);
        self.backend.container.insert_resource(gfx.1);
        self.backend.container.insert_resource(gfx.2);
    }
    fn attach_scale_factor(&mut self, scale_factor: f64) {
        let scale_factor = ScaleFactor::new(scale_factor);
        self.backend.container.insert_resource(scale_factor);
        self.frontend.container.insert_resource(scale_factor);
    }
    fn resize_callback(&mut self, size: PhysicalSize<u32>, scale_factor: f64) {
        let resize_event = WindowResize::new((size.width, size.height).into(), scale_factor);
        self.frontend.container.send_event(resize_event);
        self.backend.container.send_event(resize_event);
        self.attach_scale_factor(scale_factor);
    }
    #[cfg(target_arch = "wasm32")]
    async fn web_ignite(mut self) {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        self.init_gfx(self.event_loop.as_ref().unwrap()).await;
        use wasm_bindgen::{prelude::*, JsCast};
        if let Err(error) = call_catch(&Closure::once_into_js(move || self.ignite())) {
            let is_control_flow_exception = error.dyn_ref::<js_sys::Error>().map_or(false, |e| {
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
    }
    #[cfg(target_arch = "wasm32")]
    fn add_web_canvas(window: &Window) {
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
    }
    #[cfg(target_arch = "wasm32")]
    fn window_dimensions(scale_factor: f64) -> PhysicalSize<u32> {
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
    }

    #[cfg(target_arch = "wasm32")]
    fn web_resizing(window: &Rc<Window>) {
        use wasm_bindgen::{prelude::*, JsCast};
        let w_window = window.clone();
        let closure = Closure::wrap(Box::new(move |_e: web_sys::Event| {
            let scale_factor = w_window.scale_factor();
            let size = window_dimensions(scale_factor);
            w_window.set_inner_size(size);
        }) as Box<dyn FnMut(_)>);
        let _ = web_sys::window()
            .expect("no web_sys window")
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
        match web_sys::window().expect("no web_sys window").screen() {
            Ok(screen) => {
                let _ = screen
                    .orientation()
                    .add_event_listener_with_callback("onchange", closure.as_ref().unchecked_ref());
            }
            Err(_) => {}
        }
        closure.forget();
    }
}
pub struct EngenOptions {
    pub native_dimensions: Option<Area<DeviceContext>>,
    pub theme: Theme,
    pub gfx_options: Option<GfxOptions>,
}

impl EngenOptions {
    pub fn new() -> Self {
        Self {
            native_dimensions: None,
            theme: Theme::default(),
            gfx_options: None,
        }
    }
    pub fn with_native_dimensions<A: Into<Area<DeviceContext>>>(mut self, dimensions: A) -> Self {
        self.native_dimensions.replace(dimensions.into());
        self
    }
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

pub trait Launch {
    fn options() -> EngenOptions;
    fn preparation(frontend: &mut Job);
}

pub struct Attachment(pub Box<fn(&mut Engen)>);

impl Attachment {
    pub fn using<T: Attach>() -> Self {
        Self(Box::new(T::attach))
    }
}

pub trait Attach {
    fn attach(engen: &mut Engen);
}