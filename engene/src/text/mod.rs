mod attribute;
mod font;
mod pipeline;
mod rasterization;
mod request;
mod scale;
mod vertex;

use crate::canvas::Viewport;
use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::instance::EntityKey;
use crate::render::{Render, RenderPassHandle, RenderPhase};
use crate::task::Stage;
use crate::text::scale::Scale;
use crate::text::vertex::{Vertex, GLYPH_AABB};
use crate::{instance, Attach, Canvas, Engen, Task};
use bevy_ecs::prelude::{Commands, Res, ResMut, Resource};
use rasterization::Descriptor;
pub(crate) use request::Request;
pub(crate) type GlyphHash = fontdue::layout::GlyphRasterConfig;
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct GlyphOffset(pub usize);
pub(crate) type InstanceCoordinator = instance::Coordinator<EntityKey<GlyphOffset>, Request>;
#[derive(Resource)]
pub struct TextRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) coordinator: InstanceCoordinator,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) rasterization: rasterization::Handler,
}
impl TextRenderer {
    pub(crate) fn read_rasterization_requests(&mut self) {
        self.rasterization.read_requests(&self.coordinator);
    }
    pub(crate) fn integrate_rasterization_descriptors(&mut self) {
        self.rasterization.integrate_requests(&mut self.coordinator);
    }
    pub(crate) fn new(canvas: &Canvas) -> Self {
        let rasterization = rasterization::Handler::new(&canvas.device);
        Self {
            pipeline: pipeline::pipeline(canvas, &rasterization),
            coordinator: {
                let mut coordinator = instance::Coordinator::new(10);
                coordinator.setup_attribute::<Position>(&canvas.device);
                coordinator.setup_attribute::<Area>(&canvas.device);
                coordinator.setup_attribute::<Depth>(&canvas.device);
                coordinator.setup_attribute::<Color>(&canvas.device);
                coordinator.setup_attribute::<rasterization::Descriptor>(&canvas.device);
                coordinator
            },
            vertex_buffer: vertex::buffer(&canvas.device),
            rasterization,
        }
    }
}
pub fn startup(canvas: Res<Canvas>, mut cmd: Commands) {
    cmd.insert_resource(TextRenderer::new(&canvas));
}
pub fn prepare(canvas: Res<Canvas>, mut renderer: ResMut<TextRenderer>) {
    renderer.read_rasterization_requests();
    // ...
    renderer.integrate_rasterization_descriptors();
    renderer.coordinator.coordinate();
}
impl Attach for TextRenderer {
    fn attach(engen: &mut Engen) {
        engen
            .render
            .startup
            .schedule
            .add_system_to_stage(Stage::During, startup);
        engen
            .render
            .main
            .schedule
            .add_system_to_stage(Stage::During, prepare);
    }
}
impl Render for TextRenderer {
    fn extract(compute: &Task, render: &mut Task)
    where
        Self: Sized,
    {
        todo!()
    }

    fn phase() -> RenderPhase {
        RenderPhase::Alpha
    }

    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        render_pass_handle.0.set_pipeline(&self.pipeline);
        render_pass_handle
            .0
            .set_bind_group(0, &viewport.bind_group, &[]);
        render_pass_handle
            .0
            .set_bind_group(1, &self.rasterization.binding.bind_group, &[]);
        render_pass_handle
            .0
            .set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass_handle
            .0
            .set_vertex_buffer(1, self.coordinator.gpu_buffer::<Position>().slice(..));
        render_pass_handle
            .0
            .set_vertex_buffer(2, self.coordinator.gpu_buffer::<Area>().slice(..));
        render_pass_handle
            .0
            .set_vertex_buffer(3, self.coordinator.gpu_buffer::<Depth>().slice(..));
        render_pass_handle
            .0
            .set_vertex_buffer(4, self.coordinator.gpu_buffer::<Color>().slice(..));
        render_pass_handle.0.set_vertex_buffer(
            5,
            self.coordinator
                .gpu_buffer::<rasterization::Descriptor>()
                .slice(..),
        );
        if self.coordinator.has_instances() {
            render_pass_handle.0.draw(
                0..GLYPH_AABB.len() as u32,
                0..self.coordinator.count() as u32,
            );
        }
    }
}
