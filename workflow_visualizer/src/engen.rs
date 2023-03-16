use std::rc::Rc;

use bevy_ecs::prelude::Resource;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, Event, MouseButton, StartCause, TouchPhase, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

use crate::area::Area;
use crate::content_panel::ContentPanelAttachment;
use crate::focus::FocusAttachment;
use crate::gfx::{GfxStack, GfxSurface};
use crate::icon::IconAttachment;
use crate::job::{Job, TaskLabel};
use crate::orientation::OrientationAttachment;
use crate::render::{
    extract, invoke_extract, invoke_render, render, Extract, ExtractFns, Render, RenderFns,
    RenderPhase,
};
use crate::scale_factor::ScaleFactor;
use crate::sync::set_sync_points;
use crate::text::TextAttachment;
use crate::text_input::TextInputAttachment;
use crate::theme::{Theme, ThemeAttachment};
use crate::time::TimerAttachment;
use crate::touch::{
    Interactor, MouseAdapter, Touch, TouchAdapter, TouchAttachment, TouchEvent, TouchType,
    TrackedTouch,
};
use crate::viewport::{ViewportAttachment, ViewportHandle};
use crate::virtual_keyboard::VirtualKeyboardAttachment;
use crate::visibility::VisibilityAttachment;
use crate::window::{WindowAttachment, WindowResize};
use crate::{DeviceContext, GfxOptions, Position};

pub struct Engen {
    pub options: EngenOptions,
    pub(crate) frontend: Job,
    pub(crate) backend: Job,
    event_loop: Option<EventLoop<()>>,
    window: Option<Rc<Window>>,
    pub(crate) render_fns: (RenderFns, RenderFns),
    pub(crate) extract_fns: ExtractFns,
    attachment_queue: Vec<Attachment>,
}
impl Engen {
    fn ignite(mut self) {
        let event_loop = self.event_loop.take().expect("no event loop provided");
        event_loop.run(
            move |event, event_loop_window_target, control_flow| match event {
                Event::NewEvents(start_cause) => match start_cause {
                    StartCause::Init => {
                        #[cfg(not(target_arch = "wasm32"))]
                        futures::executor::block_on(self.init_gfx(event_loop_window_target));
                        self.invoke_attach::<TimerAttachment>();
                        self.invoke_attach::<ViewportAttachment>();
                        self.invoke_attach::<OrientationAttachment>();
                        self.invoke_attach::<WindowAttachment>();
                        self.invoke_attach::<FocusAttachment>();
                        self.invoke_attach::<VisibilityAttachment>();
                        self.invoke_attach::<TouchAttachment>();
                        self.invoke_attach::<IconAttachment>();
                        self.invoke_attach::<TextAttachment>();
                        self.invoke_attach::<VirtualKeyboardAttachment>();
                        self.invoke_attach::<TextInputAttachment>();
                        self.invoke_attach::<ContentPanelAttachment>();
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
                        let scale_factor = self.window.as_ref().unwrap().scale_factor();
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
                        self.register_touch(touch);
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        self.register_mouse_click(state, button);
                    }
                    WindowEvent::MouseWheel { .. } => {}
                    WindowEvent::CursorMoved { position, .. } => {
                        self.set_mouse_location(position);
                    }
                    WindowEvent::CursorEntered { device_id: _ } => {}
                    WindowEvent::CursorLeft { device_id: _ } => {
                        self.frontend
                            .container
                            .send_event(TouchEvent::new(TouchType::Cancelled, Touch::default()));
                        self.frontend
                            .container
                            .get_resource_mut::<MouseAdapter>()
                            .expect("no mouse adapter")
                            .tracked
                            .iter_mut()
                            .for_each(|(_button, track_state)| {
                                if let Some(registered_touch) = track_state.1.as_mut() {
                                    registered_touch.cancelled = true;
                                }
                            });
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
            },
        );
    }
}
impl Engen {
    pub fn launch<Launchable: Launch>(event_loop: EventLoop<()>) {
        let options = Launchable::options();
        let mut engen = Engen::new(options);
        set_sync_points(&mut engen);
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
            RenderPhase::Alpha(_order) => {
                self.render_fns.1.push(Box::new(invoke_render::<Renderer>))
            }
        }
        self.extract_fns.push(Box::new(invoke_extract::<Renderer>));
    }
    pub fn add_extraction<Extraction: Extract>(&mut self) {
        self.extract_fns
            .push(Box::new(invoke_extract::<Extraction>));
    }
}
impl Engen {
    fn register_mouse_click(&mut self, state: ElementState, button: MouseButton) {
        let viewport_handle_section = self
            .frontend
            .container
            .get_resource::<ViewportHandle>()
            .expect("no viewport handle")
            .section;
        let scale_factor = self
            .frontend
            .container
            .get_resource::<ScaleFactor>()
            .expect("no scale factor")
            .factor;
        let mut mouse_adapter = self
            .frontend
            .container
            .get_resource_mut::<MouseAdapter>()
            .expect("no mouse adapter");
        let cached = mouse_adapter
            .tracked
            .get(&button)
            .cloned()
            .unwrap_or((ElementState::Released, None));
        if let Some(cached) = mouse_adapter.tracked.get_mut(&button) {
            cached.0 = state;
        }
        let mut touch_events = Vec::new();
        let mouse_location = mouse_adapter.location.unwrap_or_default();
        let in_bounds = viewport_handle_section.contains(mouse_location.to_ui(scale_factor));
        let interactor = Interactor::from_button(button);
        let is_primary = interactor == MouseAdapter::PRIMARY_INTERACTOR;
        if cached.0 != state {
            match state {
                ElementState::Pressed => {
                    if in_bounds {
                        if is_primary {
                            touch_events.push(TouchEvent::new(TouchType::OnPress, mouse_location));
                        }
                        mouse_adapter
                            .tracked
                            .insert(button, (state, Some(TrackedTouch::new(mouse_location))));
                    }
                }
                ElementState::Released => {
                    if in_bounds {
                        if is_primary {
                            touch_events
                                .push(TouchEvent::new(TouchType::OnRelease, mouse_location));
                            if let Some(r_touch) = mouse_adapter.tracked.get_mut(&button) {
                                if let Some(r_touch) = r_touch.1.as_mut() {
                                    r_touch.end.replace(mouse_location);
                                }
                            }
                        }
                    } else {
                        if is_primary {
                            touch_events
                                .push(TouchEvent::new(TouchType::Cancelled, Touch::default()));
                        }
                        mouse_adapter
                            .tracked
                            .get_mut(&button)
                            .unwrap()
                            .1
                            .unwrap()
                            .cancelled = true;
                    }
                }
            }
        }
        for event in touch_events {
            self.frontend.container.send_event(event);
        }
    }
    fn set_mouse_location(&mut self, position: PhysicalPosition<f64>) {
        let mut mouse_adapter = self
            .frontend
            .container
            .get_resource_mut::<MouseAdapter>()
            .expect("no mouse adapter");
        let mouse_position = Position::<DeviceContext>::new(position.x as f32, position.y as f32);
        mouse_adapter.location.replace(mouse_position);
        let mut click_events = Vec::new();
        for (button, track_state) in mouse_adapter.tracked.iter_mut() {
            if let Some(t_state) = track_state.1.as_mut() {
                t_state.current = mouse_position;
            }
            if *button == MouseButton::Left {
                click_events.push(TouchEvent::new(TouchType::OnMove, mouse_position));
            }
        }
    }
    fn register_touch(&mut self, touch: winit::event::Touch) {
        let viewport_section = self
            .frontend
            .container
            .get_resource::<ViewportHandle>()
            .unwrap()
            .section;
        let mut touch_adapter = self
            .frontend
            .container
            .get_resource_mut::<TouchAdapter>()
            .expect("no touch adapter slot");
        let mut touch_events = Vec::new();
        let touch_location = (
            touch.location.x - viewport_section.position.x as f64,
            touch.location.y - viewport_section.position.y as f64,
        );
        let interactor = Interactor(touch.id as u32);
        match touch.phase {
            TouchPhase::Started => {
                if touch_adapter.primary.is_none() {
                    touch_adapter.primary.replace(interactor);
                    touch_events.push(TouchEvent::new(TouchType::OnPress, touch_location));
                }
                touch_adapter
                    .tracked
                    .insert(interactor, TrackedTouch::new(touch_location));
            }
            TouchPhase::Moved => {
                if let Some(registered_touch) = touch_adapter.tracked.get_mut(&interactor) {
                    registered_touch.current = (touch_location.0, touch_location.1).into();
                }
                let primary = touch_adapter.primary;
                if let Some(prime) = primary {
                    if prime == interactor {
                        let internal_touch = touch_adapter.tracked.get_mut(&prime).unwrap();
                        touch_events
                            .push(TouchEvent::new(TouchType::OnMove, internal_touch.current));
                    }
                }
            }
            TouchPhase::Ended => {
                if let Some(click) = touch_adapter.tracked.get_mut(&interactor) {
                    click.end.replace(touch_location.into());
                }
                if let Some(prime) = touch_adapter.primary {
                    if prime == interactor {
                        touch_adapter.primary.take();
                        touch_events.push(TouchEvent::new(TouchType::OnRelease, touch_location));
                    }
                }
            }
            TouchPhase::Cancelled => {
                if let Some(prime) = touch_adapter.primary {
                    if prime == interactor {
                        touch_adapter.primary.take();
                        touch_events.push(TouchEvent::new(TouchType::Cancelled, Touch::default()));
                    }
                }
                if let Some(tracked) = touch_adapter.tracked.get_mut(&interactor) {
                    tracked.cancelled = true;
                }
            }
        }
        for event in touch_events {
            self.frontend.container.send_event(event);
        }
    }
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
        let gfx_options = self
            .options
            .gfx_options
            .clone()
            .unwrap_or(GfxOptions::native());
        #[cfg(target_arch = "wasm32")]
        let gfx_options = self
            .options
            .gfx_options
            .clone()
            .unwrap_or(GfxOptions::web())
            .web_align();
        #[cfg(target_arch = "wasm32")]
        {
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
        let event_loop = self.event_loop.take();
        self.init_gfx(event_loop.as_ref().unwrap()).await;
        self.event_loop.replace(event_loop.unwrap());
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
        use winit::platform::web::WindowExtWebSys;
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
            let size = Self::window_dimensions(scale_factor);
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
    pub fn with_gfx_options(mut self, gfx_options: GfxOptions) -> Self {
        self.gfx_options.replace(gfx_options);
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
