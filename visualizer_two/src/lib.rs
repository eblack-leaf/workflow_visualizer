mod color;
mod coord;
mod gfx;
mod job;
mod render;
mod scale_factor;
mod sync;
mod theme;
mod uniform;
mod viewport;
mod window;
pub use crate::color::Color;
pub use crate::coord::{
    area::Area, area::RawArea, layer::Layer, position::Position, position::RawPosition,
    section::Section, Coordinate, DeviceContext, InterfaceContext, NumericalContext,
};
pub use crate::gfx::{GfxOptions, GfxSurface};
use crate::gfx::{GfxSurfaceConfiguration, MsaaRenderAttachment};
pub use crate::job::{EntityName, Job};
use crate::job::{attempt_to_idle, Task, TaskLabel};
use crate::render::{internal_render, invoke_render, Render, RenderFns, RenderPassHandle, RenderPhase};
pub use crate::scale_factor::ScaleFactor;
use crate::sync::set_sync_points;
pub use crate::sync::{SyncPoint, UserSpaceSyncPoint};
pub use crate::theme::Theme;
pub use crate::uniform::Uniform;
pub use crate::window::{WindowAttachment, WindowResize};
use bevy_ecs::prelude::{Component, Entity, IntoSystemConfig, Resource};
use compact_str::CompactString;
pub use job::JobSyncPoint;
pub use viewport::{Viewport, ViewportAttachment, ViewportHandle};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct Attachment(pub Box<fn(&mut Visualizer)>);

impl Attachment {
    pub fn using<T: Attach>() -> Self {
        Self(Box::new(T::attach))
    }
}

pub trait Attach {
    fn attach(visualizer: &mut Visualizer);
}
pub struct Visualizer {
    pub job: Job,
    pub(crate) render_fns: (RenderFns, RenderFns),
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
                    task.add_systems((attempt_to_idle.in_set(JobSyncPoint::Idle), ));
                    task
                });
                job
            },
            render_fns: (vec![], vec![]),
            attachment_queue: vec![],
            gfx_options,
        }
    }
    pub async fn init_gfx(&mut self, window: &Window) {
        let (surface, config, msaa) = GfxSurface::new(window, self.gfx_options.clone()).await;
        self.job.container.insert_resource(surface);
        self.job.container.insert_resource(config);
        self.job.container.insert_resource(msaa);
        self.job
            .container
            .insert_resource(ScaleFactor::new(window.scale_factor()));
    }
    pub fn trigger_resize(&mut self, size: PhysicalSize<u32>, scale_factor: f64) {
        let resize_event = WindowResize::new((size.width, size.height).into(), scale_factor);
        self.job.container.send_event(resize_event);
        self.job
            .container
            .insert_resource(ScaleFactor::new(scale_factor));
    }
    pub fn initialize(&mut self, window: &Window) {
        pollster::block_on(self.init_gfx(window));
        set_sync_points(self);
        self.invoke_attach::<WindowAttachment>();
        self.invoke_attach::<ViewportAttachment>();
        self.attach_from_queue();
        self.setup();
    }
    pub fn set_theme(&mut self, theme: Theme) {
        self.job.container.insert_resource(theme);
    }
    pub fn add_entities<T: Component>(&mut self, mut names: Vec<EntityName>, datum: Vec<T>) {
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
    pub fn register_touch(&mut self) {}
    pub fn register_mouse_click(&mut self) {}
    pub fn set_mouse_location(&mut self) {}
    pub fn set_scale_factor(&mut self) {}
    fn setup(&mut self) {
        self.job.exec(Self::TASK_STARTUP);
        self.job.exec(Self::TASK_RENDER_STARTUP)
    }
    pub fn exec_main_task(&mut self) {
        if !self.job.suspended() {
            self.job.exec(Self::TASK_MAIN);
        }
    }
    pub fn teardown(&mut self) {
        self.job.exec(Self::TASK_TEARDOWN);
    }
    pub fn suspend(&mut self) {
        let _ = self.job.container.remove_resource::<GfxSurface>();
        self.job.suspend();
    }
    pub fn resume(&mut self, window: &Window) {
        pollster::block_on(self.init_gfx(window));
        self.job.activate();
    }
    pub fn render(&mut self) {
        if !self.job.suspended() {
            self.job.exec(Self::TASK_RENDER_MAIN);
            internal_render(self);
        }
    }
    pub fn can_idle(&self) -> bool {
        self.job.can_idle()
    }
    pub fn register_renderer<Renderer: Render + Resource>(&mut self) {
        match Renderer::phase() {
            RenderPhase::Opaque => self.render_fns.0.push(Box::new(invoke_render::<Renderer>)),
            RenderPhase::Alpha(_order) => {
                self.render_fns.1.push(Box::new(invoke_render::<Renderer>))
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
