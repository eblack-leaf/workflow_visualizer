use std::rc::Rc;

use bevy_ecs::prelude::Resource;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, Touch, TouchPhase};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::Window;
#[cfg(not(target_arch = "wasm32"))]
use winit::window::WindowBuilder;

pub(crate) use job::TaskLabel;
pub use job::{Container, EntityStore, ExecutionState, Exit, Idle, Job, Task};
pub use options::EngenOptions;
pub use stages::{
    BackEndStartupStages, BackendStages, FrontEndStages, FrontEndStartupStages, FrontEndSystems,
};

use crate::gfx::{
    invoke_extract, invoke_render, Extract, ExtractFns, Render, RenderFns, RenderPhase,
};
#[cfg(not(target_arch = "wasm32"))]
use crate::gfx::{GfxOptions, GfxSurface};
use crate::theme::ThemeAttachment;
use crate::window::ScaleFactor;
use crate::window::WindowResize;
use crate::window::{Click, ClickEvent, ClickEventType, Finger, MouseAdapter, TouchAdapter};
use crate::{DeviceView, Position, VisibleBounds};

mod ignite;
mod job;
mod options;
mod stages;
mod wasm;

pub struct Engen {
    event_loop: Option<EventLoop<()>>,
    attachment_queue: Vec<Attachment>,
    #[allow(dead_code)]
    pub(crate) options: EngenOptions,
    pub(crate) render_fns: (RenderFns, RenderFns),
    pub(crate) extract_fns: ExtractFns,
    pub(crate) frontend: Job,
    pub(crate) backend: Job,
    pub(crate) window: Option<Rc<Window>>,
}

impl Engen {
    pub(crate) fn new(options: EngenOptions) -> Self {
        Self {
            event_loop: None,
            attachment_queue: vec![],
            options,
            render_fns: (vec![], vec![]),
            extract_fns: vec![],
            frontend: stages::staged_frontend(),
            backend: stages::staged_backend(),
            window: None,
        }
    }
    pub fn add_attachment<Attached: Attach>(&mut self) {
        self.attachment_queue.push(Attachment::using::<Attached>());
    }
    pub fn add_renderer<Renderer: Extract + Render + Resource>(&mut self) {
        match Renderer::phase() {
            RenderPhase::Opaque => self.render_fns.0.push(Box::new(invoke_render::<Renderer>)),
            RenderPhase::Alpha => self.render_fns.1.push(Box::new(invoke_render::<Renderer>)),
        }
        self.extract_fns.push(Box::new(invoke_extract::<Renderer>));
    }
    pub fn add_extraction<Extraction: Extract>(&mut self) {
        self.extract_fns
            .push(Box::new(invoke_extract::<Extraction>));
    }
    pub(crate) fn invoke_attach<Attachment: Attach>(&mut self) {
        Attachment::attach(self);
    }
    pub fn launch<Launcher: Launch>() {
        let options = Launcher::options();
        let mut engen = Engen::new(options);
        engen.attachment_queue = Launcher::attachments();
        engen.invoke_attach::<ThemeAttachment>();
        Launcher::prepare(&mut engen.frontend);
        #[cfg(not(target_arch = "wasm32"))]
        {
            engen.event_loop.replace(EventLoop::new());
            ignite::ignite(engen);
        }

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(wasm::web_ignite(engen));
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
    fn register_mouse_click(&mut self, state: ElementState, button: MouseButton) {
        let visible_bounds_section = self
            .frontend
            .container
            .get_resource::<VisibleBounds>()
            .expect("no visible bounds")
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
            .button_cache
            .get(&button)
            .cloned()
            .unwrap_or(ElementState::Released);
        let mut click_events = Vec::new();
        let mouse_location = mouse_adapter.location.unwrap_or_default();
        let in_bounds = visible_bounds_section.contains(mouse_location.to_ui(scale_factor));
        if cached != state {
            match cached {
                ElementState::Pressed => {
                    if in_bounds {
                        let click = mouse_adapter.clicks.get_mut(&button).unwrap();
                        click.end.replace(mouse_location);
                        if button == MouseButton::Left {
                            click_events.push(ClickEvent::new(ClickEventType::OnRelease, *click));
                        }
                    } else {
                        if button == MouseButton::Left {
                            click_events
                                .push(ClickEvent::new(ClickEventType::Cancelled, Click::default()));
                        }
                    }
                }
                ElementState::Released => {
                    if in_bounds {
                        let click = Click::new(mouse_location);
                        mouse_adapter.clicks.insert(button, click);
                        if button == MouseButton::Left {
                            click_events.push(ClickEvent::new(ClickEventType::OnPress, click));
                        }
                    }
                }
            }
        }
        mouse_adapter.button_cache.insert(button, state);
        for event in click_events {
            self.frontend.container.send_event(event);
        }
    }
    fn set_mouse_location(&mut self, position: PhysicalPosition<f64>) {
        let mut mouse_adapter = self
            .frontend
            .container
            .get_resource_mut::<MouseAdapter>()
            .expect("no mouse adapter");
        let mouse_position = Position::<DeviceView>::new(position.x as f32, position.y as f32);
        mouse_adapter.location.replace(mouse_position);
        let mut click_events = Vec::new();
        for click in mouse_adapter.clicks.iter_mut() {
            click.1.current.replace(mouse_position);
            if *click.0 == MouseButton::Left {
                click_events.push(ClickEvent::new(ClickEventType::OnMove, *click.1));
            }
        }
    }
    fn register_touch(&mut self, touch: Touch) {
        let viewport_offset = self
            .frontend
            .container
            .get_resource::<VisibleBounds>()
            .unwrap()
            .section;
        let mut touch_adapter = self
            .frontend
            .container
            .get_resource_mut::<TouchAdapter>()
            .expect("no touch adapter slot");
        let mut click_events = Vec::new();
        let touch_location = (
            touch.location.x - viewport_offset.position.x as f64,
            touch.location.y - viewport_offset.position.y as f64,
        );
        match touch.phase {
            TouchPhase::Started => {
                let click = Click::new(touch_location);
                if touch_adapter.primary.is_none() {
                    touch_adapter.primary.replace(touch.id as Finger);
                    click_events.push(ClickEvent::new(ClickEventType::OnPress, click));
                }
                touch_adapter.tracked.insert(touch.id as Finger, click);
            }
            TouchPhase::Moved => {
                if let Some(click) = touch_adapter.tracked.get_mut(&(touch.id as Finger)) {
                    click
                        .current
                        .replace((touch_location.0, touch_location.1).into());
                }
                let primary = touch_adapter.primary.clone();
                if let Some(prime) = primary {
                    if prime == touch.id as Finger {
                        let click = touch_adapter.tracked.get_mut(&prime).unwrap();
                        click_events.push(ClickEvent::new(ClickEventType::OnMove, *click));
                    }
                }
            }
            TouchPhase::Ended => {
                if let Some(click) = touch_adapter.tracked.get_mut(&(touch.id as Finger)) {
                    click.end.replace(touch_location.into());
                }
                if let Some(finger) = touch_adapter.primary {
                    if finger == touch.id as Finger {
                        touch_adapter.primary.take();
                        let click = *touch_adapter
                            .tracked
                            .get(&finger)
                            .expect("no tracked finger");
                        click_events.push(ClickEvent::new(ClickEventType::OnRelease, click));
                    }
                }
            }
            TouchPhase::Cancelled => {
                if let Some(finger) = touch_adapter.primary {
                    if finger == touch.id as Finger {
                        touch_adapter.primary.take();
                        click_events
                            .push(ClickEvent::new(ClickEventType::Cancelled, Click::default()));
                    }
                }
                touch_adapter.tracked.remove(&(touch.id as Finger));
            }
        }
        for event in click_events {
            self.frontend.container.send_event(event);
        }
    }
    fn init_native_gfx(&mut self, _event_loop_window_target: &EventLoopWindowTarget<()>) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut builder = WindowBuilder::new().with_title("native engen");
            if let Some(native_dimensions) = self.options.native_dimensions {
                builder = builder.with_inner_size(PhysicalSize::new(
                    native_dimensions.width,
                    native_dimensions.height,
                ));
            }
            let window = Rc::new(builder.build(_event_loop_window_target).expect("no window"));
            self.attach_scale_factor(window.scale_factor());
            let gfx = futures::executor::block_on(GfxSurface::new(&window, GfxOptions::native()));
            self.window.replace(window);
            self.backend.container.insert_resource(gfx.0);
            self.backend.container.insert_resource(gfx.1);
        }
    }
    fn attach_from_queue(&mut self) {
        let attachment_queue = self.attachment_queue.drain(..).collect::<Vec<Attachment>>();
        for attach_fn in attachment_queue {
            attach_fn.0(self);
        }
    }
}

pub struct Attachment(pub Box<fn(&mut Engen)>);

impl Attachment {
    pub fn using<T: Attach>() -> Self {
        Self(Box::new(T::attach))
    }
}

pub trait Launch {
    fn options() -> EngenOptions;
    fn attachments() -> Vec<Attachment>;
    fn prepare(job: &mut Job);
}

pub trait Attach {
    fn attach(engen: &mut Engen);
}
