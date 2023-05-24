use crate::focus::FocusAttachment;
use crate::job::{attempt_to_idle, Task, TaskLabel};
use crate::orientation::OrientationAttachment;
use crate::render::{internal_render, invoke_render, Render, RenderTasks, RenderPhase, RenderTaskManager};
use crate::sync::set_sync_points;
use crate::text::TextAttachment;
use crate::time::TimerAttachment;
use crate::touch::{
    Interactor, MouseAdapter, Touch, TouchAdapter, TouchAttachment, TouchEvent, TouchType,
    TrackedTouch,
};
use crate::virtual_keyboard::VirtualKeyboardAttachment;
use crate::visibility::VisibilityAttachment;
use crate::{
    Area, DeviceContext, EntityName, GfxOptions, GfxSurface, Job, JobSyncPoint, Position,
    ScaleFactor, Section, Theme, Viewport, ViewportAttachment, ViewportHandle, WindowAttachment,
    WindowResize,
};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{IntoSystemConfig, Resource};
use std::fmt::{Debug, Formatter};
use tracing::{info, instrument, trace, warn};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, TouchPhase};
use winit::window::Window;
use crate::gfx::GfxSurfaceConfiguration;

pub struct Attachment(pub Box<fn(&mut Visualizer)>);

impl Attachment {
    pub fn using<T: Attach>() -> Self {
        Self(Box::new(T::attach))
    }
}

pub trait Attach {
    fn attach(visualizer: &mut Visualizer);
}

impl Debug for Visualizer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Visualizer")
    }
}

pub struct Visualizer {
    pub job: Job,
    pub(crate) render_task_manager: RenderTaskManager,
    attachment_queue: Vec<Attachment>,
    gfx_options: GfxOptions,
}

impl Visualizer {
    pub const TASK_MAIN: TaskLabel = TaskLabel("main");
    pub const TASK_STARTUP: TaskLabel = TaskLabel("startup");
    pub const TASK_TEARDOWN: TaskLabel = TaskLabel("teardown");
    pub const TASK_RENDER_STARTUP: TaskLabel = TaskLabel("render_startup");
    pub const TASK_RENDER_MAIN: TaskLabel = TaskLabel("render_main");
    pub fn new(theme: Theme, gfx_options: GfxOptions) -> Self {
        Self {
            job: {
                let mut job = Job::new();
                job.container.insert_resource(theme);
                job.tasks.insert(Self::TASK_STARTUP, Task::new());
                job.tasks.insert(Self::TASK_RENDER_STARTUP, Task::new());
                job.tasks.insert(Self::TASK_TEARDOWN, Task::new());
                job.tasks.insert(Self::TASK_RENDER_MAIN, Task::new());
                job.tasks.insert(Self::TASK_MAIN, {
                    let mut task = Task::default();
                    task.add_systems((attempt_to_idle.in_set(JobSyncPoint::Idle),));
                    task
                });
                job
            },
            render_task_manager: RenderTaskManager::new(),
            attachment_queue: vec![],
            gfx_options,
        }
    }
    pub fn set_gfx_options(&mut self, gfx_options: GfxOptions) {
        self.gfx_options = gfx_options;
    }
    pub async fn init_gfx(&mut self, window: &Window) {
        info!("initializing gfx.");
        let (surface, config, msaa) = GfxSurface::new(window, self.gfx_options.clone()).await;
        info!(
            "gfx -> surface: {:?}/{:?}",
            config.configuration.width, config.configuration.height
        );
        let area = Area::<DeviceContext>::new(
            config.configuration.width as f32,
            config.configuration.height as f32,
        );
        let viewport = Viewport::new(&surface.device, area, msaa.requested());
        let scale_factor = ScaleFactor::new(window.scale_factor());
        let viewport_handle =
            ViewportHandle::new(Section::new((0, 0), area.to_ui(scale_factor.factor)));
        self.job.container.insert_resource(viewport);
        self.job.container.insert_resource(viewport_handle);
        self.job.container.insert_resource(surface);
        self.job.container.insert_resource(config);
        self.job.container.insert_resource(msaa);
        self.job.container.insert_resource(scale_factor);
    }
    pub fn trigger_resize(&mut self, size: PhysicalSize<u32>, scale_factor: f64) {
        let resize_event = WindowResize::new((size.width, size.height).into(), scale_factor);
        self.job.container.send_event(resize_event);
        self.job
            .container
            .insert_resource(ScaleFactor::new(scale_factor));
    }
    pub fn initialize(&mut self, window: &Window) {
        #[cfg(not(target_family = "wasm"))]
        pollster::block_on(self.init_gfx(window));
        set_sync_points(self);
        self.invoke_attach::<WindowAttachment>();
        self.invoke_attach::<ViewportAttachment>();
        self.invoke_attach::<VisibilityAttachment>();
        self.invoke_attach::<TouchAttachment>();
        self.invoke_attach::<FocusAttachment>();
        self.invoke_attach::<OrientationAttachment>();
        self.invoke_attach::<TimerAttachment>();
        self.invoke_attach::<VirtualKeyboardAttachment>();
        self.invoke_attach::<TextAttachment>();
        self.attach_from_queue();
        self.setup();
    }
    pub fn set_theme(&mut self, theme: Theme) {
        self.job.container.insert_resource(theme);
    }
    pub fn add_named_entities<T: Component>(&mut self, mut names: Vec<EntityName>, datum: Vec<T>) {
        let ids = self
            .job
            .container
            .spawn_batch(datum)
            .collect::<Vec<Entity>>();
        let mut names = names.drain(..);
        for id in ids {
            self.job.store_entity(names.next().unwrap(), id);
        }
    }
    pub fn add_entities<T: Component>(&mut self, datum: Vec<T>) -> Vec<Entity> {
        let ids = self
            .job
            .container
            .spawn_batch(datum)
            .collect::<Vec<Entity>>();
        ids
    }
    pub fn register_touch(&mut self, touch: winit::event::Touch) {
        let viewport_section = self
            .job
            .container
            .get_resource::<ViewportHandle>()
            .unwrap()
            .section;
        let mut touch_adapter = self
            .job
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
            self.job.container.send_event(event);
        }
    }
    pub fn register_mouse_click(&mut self, state: ElementState, button: MouseButton) {
        let viewport_handle_section = self
            .job
            .container
            .get_resource::<ViewportHandle>()
            .expect("no viewport handle")
            .section;
        let scale_factor = self
            .job
            .container
            .get_resource::<ScaleFactor>()
            .expect("no scale factor")
            .factor;
        let mut mouse_adapter = self
            .job
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
            self.job.container.send_event(event);
        }
    }
    pub fn set_mouse_location(&mut self, position: PhysicalPosition<f64>) {
        let mut mouse_adapter = self
            .job
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
    pub fn cancel_touches(&mut self) {
        self.job
            .container
            .send_event(TouchEvent::new(TouchType::Cancelled, Touch::default()));
        self.job
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
    pub fn set_scale_factor(&mut self, scale_factor: f64) {
        self.job
            .container
            .insert_resource(ScaleFactor::new(scale_factor));
    }
    fn setup(&mut self) {
        self.job.exec(Self::TASK_STARTUP);
        self.job.exec(Self::TASK_RENDER_STARTUP)
    }
    pub fn exec(&mut self) {
        if !self.job.suspended() {
            self.job.exec(Self::TASK_MAIN);
        }
    }
    pub fn teardown(&mut self) {
        self.job.exec(Self::TASK_TEARDOWN);
    }
    pub fn suspend(&mut self) {
        if !self.job.suspended() {
            self.job.suspend();
        }
    }
    pub fn resume(&mut self, window: &Window) {
        if !self.job.resumed() {
            self.recreate_surface(window);
            self.job.resume();
        }
    }
    fn recreate_surface(&mut self, window: &Window) {
        let config = self.job.container.remove_resource::<GfxSurfaceConfiguration>().expect("gfx config");
        let mut gfx = self.job.container.get_resource_mut::<GfxSurface>().expect("gfx");
        gfx.surface = unsafe { gfx.instance.create_surface(window).expect("gfx surface") };
        gfx.surface.configure(&gfx.device, &config.configuration);
        self.job.container.insert_resource(config);
    }
    pub fn render(&mut self) {
        if !self.job.suspended() {
            trace!("starting render main");
            self.job.exec(Self::TASK_RENDER_MAIN);
            internal_render(self);
        }
    }
    pub fn can_idle(&self) -> bool {
        self.job.can_idle()
    }
    pub fn register_renderer<Renderer: Render + Resource>(&mut self) {
        match Renderer::phase() {
            RenderPhase::Opaque => self.render_task_manager.opaque.push(Box::new(invoke_render::<Renderer>)),
            RenderPhase::Alpha(_order) => {
                self.render_task_manager.transparent.push(Box::new(invoke_render::<Renderer>))
            }
        }
    }
    pub fn add_attachment<Attached: Attach>(&mut self) {
        self.attachment_queue.push(Attachment::using::<Attached>());
    }
    fn attach_from_queue(&mut self) {
        let attachment_queue = self.attachment_queue.drain(..).collect::<Vec<Attachment>>();
        for attach_fn in attachment_queue {
            attach_fn.0(self);
        }
    }
    fn invoke_attach<Attachment: Attach>(&mut self) {
        Attachment::attach(self);
    }
}
