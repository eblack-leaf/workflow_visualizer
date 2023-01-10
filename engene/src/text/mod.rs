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
use crate::text::font::Font;
use crate::text::scale::Scale;
use crate::text::vertex::{Vertex, GLYPH_AABB};
use crate::{instance, Attach, Canvas, Engen, Task};
use bevy_ecs::prelude::{Commands, Res, ResMut, Resource};
use rasterization::PlacementDescriptor;
pub(crate) use request::RequestData;

pub(crate) type GlyphHash = fontdue::layout::GlyphRasterConfig;
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct GlyphOffset(pub usize);
pub(crate) type TextBufferCoordinator =
    instance::BufferCoordinator<EntityKey<GlyphOffset>, RequestData>;
#[derive(Resource)]
pub struct TextRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) buffer_coordinator: TextBufferCoordinator,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) rasterization: rasterization::RasterizationHandler,
}
impl TextRenderer {
    pub(crate) fn prepare_rasterization(&mut self, canvas: &Canvas) {
        self.rasterization.read_requests(&self.buffer_coordinator);
        self.rasterization.prepare(canvas);
        self.rasterization
            .integrate_requests(&mut self.buffer_coordinator);
    }
    pub(crate) fn new(canvas: &Canvas) -> Self {
        let rasterization = rasterization::RasterizationHandler::new(&canvas.device);
        Self {
            pipeline: pipeline::pipeline(canvas, &rasterization),
            buffer_coordinator: {
                let mut coordinator = instance::BufferCoordinator::new(10);
                coordinator.setup_attribute::<Position>(&canvas.device);
                coordinator.setup_attribute::<Area>(&canvas.device);
                coordinator.setup_attribute::<Depth>(&canvas.device);
                coordinator.setup_attribute::<Color>(&canvas.device);
                coordinator.setup_attribute::<rasterization::PlacementDescriptor>(&canvas.device);
                coordinator
            },
            vertex_buffer: vertex::buffer(&canvas.device),
            rasterization,
        }
    }
}
pub fn compute_startup(mut cmd: Commands) {
    cmd.insert_resource(Font::default());
}
pub fn startup(canvas: Res<Canvas>, mut cmd: Commands) {
    cmd.insert_resource(TextRenderer::new(&canvas));
}
pub fn prepare(canvas: Res<Canvas>, mut renderer: ResMut<TextRenderer>) {
    renderer.prepare_rasterization(&canvas);
    renderer.buffer_coordinator.start();
    renderer.buffer_coordinator.prepare::<Position>(&canvas);
    renderer.buffer_coordinator.prepare::<Area>(&canvas);
    renderer.buffer_coordinator.prepare::<Depth>(&canvas);
    renderer.buffer_coordinator.prepare::<Color>(&canvas);
    renderer
        .buffer_coordinator
        .prepare::<rasterization::PlacementDescriptor>(&canvas);
    renderer.buffer_coordinator.finish();
}
impl Attach for TextRenderer {
    fn attach(engen: &mut Engen) {
        engen
            .compute
            .startup
            .schedule
            .add_system_to_stage(Stage::During, compute_startup);
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
        render_pass_handle.0.set_vertex_buffer(
            1,
            self.buffer_coordinator.gpu_buffer::<Position>().slice(..),
        );
        render_pass_handle
            .0
            .set_vertex_buffer(2, self.buffer_coordinator.gpu_buffer::<Area>().slice(..));
        render_pass_handle
            .0
            .set_vertex_buffer(3, self.buffer_coordinator.gpu_buffer::<Depth>().slice(..));
        render_pass_handle
            .0
            .set_vertex_buffer(4, self.buffer_coordinator.gpu_buffer::<Color>().slice(..));
        render_pass_handle.0.set_vertex_buffer(
            5,
            self.buffer_coordinator
                .gpu_buffer::<rasterization::PlacementDescriptor>()
                .slice(..),
        );
        if self.buffer_coordinator.has_instances() {
            render_pass_handle.0.draw(
                0..GLYPH_AABB.len() as u32,
                0..self.buffer_coordinator.count() as u32,
            );
        }
    }
}
