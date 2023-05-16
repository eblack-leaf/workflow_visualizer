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
use crate::job::{Task, TaskLabel};
use crate::render::{invoke_render, Render, RenderFns, RenderPassHandle, RenderPhase};
pub use crate::scale_factor::ScaleFactor;
use crate::sync::set_sync_points;
pub use crate::sync::{SyncPoint, UserSpaceSyncPoint};
pub use crate::theme::Theme;
pub use crate::uniform::Uniform;
pub use crate::window::{WindowAttachment, WindowResize};
use bevy_ecs::prelude::{Component, Entity, Resource};
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
    pub render_preparation: Task,
    pub render_initialization: Task,
    pub(crate) render_fns: (RenderFns, RenderFns),
    attachment_queue: Vec<Attachment>,
    gfx_options: GfxOptions,
}
impl Visualizer {
    pub fn new(theme: Theme, gfx_options: GfxOptions) -> Self {
        Self {
            job: {
                let mut job = Job::new();
                job.container.insert_resource(theme);
                job
            },
            render_preparation: Task::new(),
            render_initialization: Task::new(),
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
        self.job.exec(TaskLabel::Startup);
        self.render_initialization.run(&mut self.job.container);
    }
    pub fn compute(&mut self) {
        if !self.job.suspended() {
            self.job.exec(TaskLabel::Main);
        }
    }
    pub fn teardown(&mut self) {
        self.job.exec(TaskLabel::Teardown);
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
            self.render_preparation.run(&mut self.job.container);
            let gfx_surface = self
                .job
                .container
                .get_resource::<GfxSurface>()
                .expect("no gfx surface attached");
            let gfx_surface_configuration = self
                .job
                .container
                .get_resource::<GfxSurfaceConfiguration>()
                .expect("no gfx surface configuration");
            let theme = self
                .job
                .container
                .get_resource::<Theme>()
                .expect("no theme attached");
            let viewport = self
                .job
                .container
                .get_resource::<Viewport>()
                .expect("no viewport attached");
            let msaa_attachment = self
                .job
                .container
                .get_resource::<MsaaRenderAttachment>()
                .expect("no msaa attachment");
            if let Some(surface_texture) = gfx_surface.surface_texture(gfx_surface_configuration) {
                let mut command_encoder =
                    gfx_surface
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("command encoder"),
                        });
                let surface_texture_view = surface_texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                {
                    let depth_texture_view = viewport
                        .depth_texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    let (v, rt) = match &msaa_attachment.view {
                        Some(view) => (view, Some(&surface_texture_view)),
                        None => (&surface_texture_view, None),
                    };
                    let should_store = msaa_attachment.requested == 1;
                    let color_attachment = wgpu::RenderPassColorAttachment {
                        view: v,
                        resolve_target: rt,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(theme.background.into()),
                            store: should_store,
                        },
                    };
                    let render_pass_descriptor = wgpu::RenderPassDescriptor {
                        label: Some("render pass"),
                        color_attachments: &[Some(color_attachment)],
                        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                            view: &depth_texture_view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(viewport.cpu.far_layer()),
                                store: true,
                            }),
                            stencil_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(0u32),
                                store: true,
                            }),
                        }),
                    };
                    let mut render_pass_handle = RenderPassHandle(
                        command_encoder.begin_render_pass(&render_pass_descriptor),
                    );
                    for invoke in self.render_fns.0.iter_mut() {
                        invoke(&self.job, &mut render_pass_handle);
                    }
                }
                gfx_surface
                    .queue
                    .submit(std::iter::once(command_encoder.finish()));
                surface_texture.present();
            }
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
