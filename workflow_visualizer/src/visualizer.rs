use std::fmt::{Debug, Formatter};

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Bundle, Event, Events, IntoSystemConfigs, Resource};
use tracing::{info, trace};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton};
use winit::window::Window;

use crate::animate::{end_animations, start_animations, update_animations};
use crate::button::ButtonAttachment;
use crate::focus::FocusAttachment;
use crate::gfx::GfxSurfaceConfiguration;
use crate::grid::GridAttachment;
use crate::icon::{IconAttachment, IconRendererAttachment};
use crate::images::ImageAttachment;
use crate::interaction::{InteractionAttachment, InteractionDevice, MouseAdapter};
use crate::job::{attempt_to_idle, Task, TaskLabel};
use crate::line::LineAttachment;
use crate::orientation::OrientationAttachment;
use crate::panel::PanelAttachment;
use crate::path::PathAttachment;
use crate::render::{internal_render, invoke_render, Render, RenderPhase, RenderTaskManager};
use crate::sync::set_sync_points;
use crate::text::TextAttachment;
use crate::time::TimerAttachment;
use crate::viewport::ViewportAttachment;
use crate::virtual_keyboard::VirtualKeyboardAttachment;
use crate::visibility::VisibilityAttachment;
use crate::window::WindowAttachment;
use crate::{
    Animate, Area, DeviceContext, GfxOptions, GfxSurface, InteractionEvent, InteractionPhase, Job,
    JobSyncPoint, MsaaRenderAdapter, PrimaryInteraction, PrimaryMouseButton, ScaleFactor, Section,
    SyncPoint, Theme, Viewport, ViewportHandle, WindowResize,
};

/// Used to hold queued attachments until ready to invoke attach to the Visualizer
pub struct Attachment(pub Box<fn(&mut Visualizer)>);

impl Attachment {
    pub fn using<T: Attach>() -> Self {
        Self(Box::new(T::attach))
    }
}
/// Trait for configuring how a part attaches to the Job of a visualizer
pub trait Attach {
    fn attach(visualizer: &mut Visualizer);
}

impl Debug for Visualizer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Visualizer")
    }
}
/// Struct for preparing/configuring/rendering visuals
pub struct Visualizer {
    pub job: Job,
    pub(crate) render_task_manager: RenderTaskManager,
    attachment_queue: Vec<Attachment>,
    gfx_options: GfxOptions,
}

impl Visualizer {
    /// Main logic task for UI element reactions
    pub const TASK_MAIN: TaskLabel = TaskLabel("main");
    /// Preparation for the logic
    pub const TASK_STARTUP: TaskLabel = TaskLabel("startup");
    /// Teardown for logic
    pub const TASK_TEARDOWN: TaskLabel = TaskLabel("teardown");
    /// Preparation for rendering
    pub const TASK_RENDER_STARTUP: TaskLabel = TaskLabel("render_startup");
    /// Render main
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
    pub fn task(&mut self, label: TaskLabel) -> &mut Task {
        self.job.task(label)
    }
    pub fn set_gfx_options(&mut self, gfx_options: GfxOptions) {
        self.gfx_options = gfx_options;
    }
    /// Initializes the `GfxSurface` and `Viewport` for a Visualizer
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
            ViewportHandle::new(Section::new((0, 0), area.to_ui(scale_factor.factor())));
        #[cfg(not(target_family = "wasm"))]
        self.job.container.insert_resource(viewport);
        #[cfg(target_family = "wasm")]
        self.job.container.insert_non_send_resource(viewport);
        self.job.container.insert_resource(viewport_handle);
        #[cfg(not(target_family = "wasm"))]
        self.job.container.insert_resource(surface);
        #[cfg(target_family = "wasm")]
        self.job.container.insert_non_send_resource(surface);
        #[cfg(not(target_family = "wasm"))]
        self.job.container.insert_resource(config);
        #[cfg(target_family = "wasm")]
        self.job.container.insert_non_send_resource(config);
        #[cfg(not(target_family = "wasm"))]
        self.job.container.insert_resource(msaa);
        #[cfg(target_family = "wasm")]
        self.job.container.insert_non_send_resource(msaa);
        self.job.container.insert_resource(scale_factor);
    }
    /// Tells visualizer to send a signal of resizing to be read in systems
    pub fn trigger_resize(&mut self, size: PhysicalSize<u32>, scale_factor: f64) {
        let resize_event = WindowResize::new((size.width, size.height).into(), scale_factor);
        self.job.container.send_event(resize_event);
        self.job
            .container
            .insert_resource(ScaleFactor::new(scale_factor));
    }
    /// invokes queued attachments after system attachments
    pub fn initialize(&mut self, window: &Window) {
        #[cfg(not(target_family = "wasm"))]
        pollster::block_on(self.init_gfx(window));
        set_sync_points(self);
        self.invoke_attach::<WindowAttachment>();
        self.invoke_attach::<ViewportAttachment>();
        self.invoke_attach::<GridAttachment>();
        self.invoke_attach::<PathAttachment>();
        self.invoke_attach::<PanelAttachment>();
        self.invoke_attach::<LineAttachment>();
        self.invoke_attach::<IconAttachment>();
        self.invoke_attach::<VisibilityAttachment>();
        self.invoke_attach::<InteractionAttachment>();
        self.invoke_attach::<FocusAttachment>();
        self.invoke_attach::<OrientationAttachment>();
        self.invoke_attach::<TimerAttachment>();
        self.invoke_attach::<VirtualKeyboardAttachment>();
        self.invoke_attach::<TextAttachment>();
        self.invoke_attach::<ButtonAttachment>();
        self.invoke_attach::<ImageAttachment>();
        self.attach_from_queue();
        self.invoke_attach::<IconRendererAttachment>();
        self.setup();
        self.job.resume();
    }
    pub fn set_theme(&mut self, theme: Theme) {
        self.job.container.insert_resource(theme);
    }
    pub fn spawn<T: Bundle, SQ: Into<SpawnQueue<T>>>(&mut self, spawn_queue: SQ) -> Vec<Entity> {
        let queue = spawn_queue.into();
        self.job
            .container
            .spawn_batch(queue.datum)
            .collect::<Vec<Entity>>()
    }
    pub fn add_event<E: Event + Sync + Send + 'static>(&mut self) {
        self.job
            .task(Self::TASK_MAIN)
            .add_systems((Events::<E>::update_system.in_set(SyncPoint::Event),));
        self.job.container.insert_resource(Events::<E>::default());
    }
    pub fn register_touch(&mut self, touch: winit::event::Touch) {
        self.job.container.send_event(InteractionEvent::new(
            InteractionDevice::Touchscreen,
            (touch.location.x, touch.location.y).into(),
            touch.phase.into(),
            touch.id.into(),
        ));
    }
    pub fn register_mouse_click(&mut self, state: ElementState, button: MouseButton) {
        let location = self
            .job
            .container
            .get_resource::<MouseAdapter>()
            .expect("mouse adapter")
            .location;
        if self
            .job
            .container
            .get_resource_mut::<MouseAdapter>()
            .expect("mouse adapter")
            .cache_invalid(button, state)
        {
            match state {
                ElementState::Pressed => {
                    self.job.container.send_event(InteractionEvent::new(
                        InteractionDevice::Mouse,
                        location,
                        InteractionPhase::Started,
                        button.into(),
                    ));
                }
                ElementState::Released => {
                    self.job.container.send_event(InteractionEvent::new(
                        InteractionDevice::Mouse,
                        location,
                        InteractionPhase::Ended,
                        button.into(),
                    ));
                }
            }
        }
    }
    pub fn set_mouse_location(&mut self, position: PhysicalPosition<f64>) {
        self.job
            .container
            .get_resource_mut::<MouseAdapter>()
            .expect("mouse adapter")
            .location = (position.x, position.y).into();
        let primary_button = self
            .job
            .container
            .get_resource::<PrimaryMouseButton>()
            .expect("primary_mouse_button")
            .0
            .to_mouse_button();
        let prime = self
            .job
            .container
            .get_resource::<PrimaryInteraction>()
            .expect("primary_interaction")
            .0;
        if let Some(pri) = prime {
            if pri == primary_button.into() {
                if let Some(cached) = self
                    .job
                    .container
                    .get_resource_mut::<MouseAdapter>()
                    .expect("mouse adapter")
                    .button_cache
                    .get(&primary_button)
                {
                    if *cached == ElementState::Pressed {
                        self.job.container.send_event(InteractionEvent::new(
                            InteractionDevice::Mouse,
                            (position.x, position.y).into(),
                            InteractionPhase::Moved,
                            primary_button.into(),
                        ));
                    }
                }
            }
        }
    }
    pub fn cancel_touches(&mut self) {
        // self.job.container.send_event(InteractionEvent::new());
    }
    pub fn set_scale_factor(&mut self, scale_factor: f64) {
        self.job
            .container
            .insert_resource(ScaleFactor::new(scale_factor));
    }
    /// Exec setup tasks
    fn setup(&mut self) {
        self.job.exec(Self::TASK_STARTUP);
        self.job.exec(Self::TASK_RENDER_STARTUP)
    }
    /// Run MAIN logic
    pub fn exec(&mut self) {
        if !self.job.suspended() {
            self.job.exec(Self::TASK_MAIN);
        }
    }
    pub fn teardown(&mut self) {
        self.job.exec(Self::TASK_TEARDOWN);
    }
    /// Tells Job to stop execution of tasks
    pub fn suspend(&mut self) {
        if !self.job.suspended() {
            self.job.suspend();
        }
    }
    /// Tells Job to resume execution of tasks
    pub fn resume(&mut self, window: &Window) {
        if !self.job.resumed() {
            self.recreate_surface(window);
            self.job.resume();
        }
    }
    fn recreate_surface(&mut self, window: &Window) {
        #[cfg(not(target_family = "wasm"))]
        let config = self
            .job
            .container
            .remove_resource::<GfxSurfaceConfiguration>()
            .expect("gfx config");
        #[cfg(target_family = "wasm")]
        let config = self
            .job
            .container
            .remove_non_send_resource::<GfxSurfaceConfiguration>()
            .expect("gfx config");
        #[cfg(not(target_family = "wasm"))]
        let mut gfx = self
            .job
            .container
            .get_resource_mut::<GfxSurface>()
            .expect("gfx");
        #[cfg(target_family = "wasm")]
        let mut gfx = self
            .job
            .container
            .get_non_send_resource_mut::<GfxSurface>()
            .expect("gfx");
        gfx.surface = unsafe { gfx.instance.create_surface(window).expect("gfx surface") };
        // send resize event instead to get window sizing?
        gfx.surface.configure(&gfx.device, &config.configuration);
        #[cfg(not(target_family = "wasm"))]
        self.job.container.insert_resource(config);
        #[cfg(target_family = "wasm")]
        self.job.container.insert_non_send_resource(config);
    }
    /// execute RENDER_MAIN then run the render pass
    pub fn render(&mut self) {
        if !self.job.suspended() {
            trace!("starting render main");
            self.job.exec(Self::TASK_RENDER_MAIN);
            internal_render(self);
        }
    }
    /// Job is in state to be awaited
    pub fn can_idle(&self) -> bool {
        self.job.can_idle()
    }
    /// register render fn with the Visualizer
    #[cfg(not(target_family = "wasm"))]
    pub fn register_renderer<Renderer: Render + Resource + 'static>(&mut self) {
        let gfx = self.job.container.get_resource::<GfxSurface>().unwrap();
        let gfx_config = self
            .job
            .container
            .get_resource::<GfxSurfaceConfiguration>()
            .unwrap();
        let msaa = self
            .job
            .container
            .get_resource::<MsaaRenderAdapter>()
            .unwrap();
        let viewport = self.job.container.get_resource::<Viewport>().unwrap();
        let scale_factor = self.job.container.get_resource::<ScaleFactor>().unwrap();
        let renderer: Renderer =
            Render::setup(&self, gfx, viewport, gfx_config, msaa, scale_factor);
        self.job.container.insert_resource(renderer);
        match Renderer::phase() {
            RenderPhase::Opaque => self
                .render_task_manager
                .opaque
                .push(Box::new(invoke_render::<Renderer>)),
            // TODO insert in order + after if same priority
            RenderPhase::Alpha(_order) => self
                .render_task_manager
                .transparent
                .push(Box::new(invoke_render::<Renderer>)),
        }
    }
    #[cfg(target_family = "wasm")]
    pub fn register_renderer<Renderer: Render + 'static>(&mut self) {
        let gfx = self
            .job
            .container
            .get_non_send_resource::<GfxSurface>()
            .unwrap();
        let gfx_config = self
            .job
            .container
            .get_non_send_resource::<GfxSurfaceConfiguration>()
            .unwrap();
        let msaa = self
            .job
            .container
            .get_non_send_resource::<MsaaRenderAdapter>()
            .unwrap();
        let viewport = self
            .job
            .container
            .get_non_send_resource::<Viewport>()
            .unwrap();
        let scale_factor = self.job.container.get_resource::<ScaleFactor>().unwrap();
        let renderer: Renderer =
            Render::setup(&self, gfx, viewport, gfx_config, msaa, scale_factor);
        self.job.container.insert_non_send_resource(renderer);
        match Renderer::phase() {
            RenderPhase::Opaque => self
                .render_task_manager
                .opaque
                .push(Box::new(invoke_render::<Renderer>)),
            // TODO insert in order + after if same priority
            RenderPhase::Alpha(_order) => self
                .render_task_manager
                .transparent
                .push(Box::new(invoke_render::<Renderer>)),
        }
    }
    pub fn register_animation<A: Animate + Send + Sync + 'static + Clone>(&mut self) {
        self.job.task(Self::TASK_MAIN).add_systems((
            update_animations::<A>.in_set(SyncPoint::PostInitialization),
            start_animations::<A>.in_set(SyncPoint::PostProcessPreparation),
            end_animations::<A>.in_set(SyncPoint::Finish),
        ));
    }
    /// queue attachment to the Visualizer
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

pub struct SpawnQueue<T: Bundle> {
    datum: Vec<T>,
}

impl<T: Bundle> SpawnQueue<T> {
    pub fn new() -> Self {
        Self { datum: vec![] }
    }
    pub fn queue(&mut self, data: T) {
        self.datum.push(data);
    }
}

impl<T: Bundle> From<T> for SpawnQueue<T> {
    fn from(value: T) -> Self {
        SpawnQueue { datum: vec![value] }
    }
}

impl<T: Bundle> From<Vec<T>> for SpawnQueue<T> {
    fn from(value: Vec<T>) -> Self {
        SpawnQueue { datum: value }
    }
}
