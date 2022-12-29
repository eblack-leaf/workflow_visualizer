mod font;
mod pipeline;
mod rasterization;
mod scale;
mod vertex;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::instance_coordinator::{EntityKey, InstanceCoordinator};
use crate::render::Render;
use crate::text::pipeline::pipeline;
use crate::text::rasterization::{GlyphHash, Rasterization};
use crate::text::scale::Scale;
use crate::text::vertex::GLYPH_AABB;
use crate::viewport::Viewport;
use crate::{Gfx, Job};
use wgpu::util::DeviceExt;
use wgpu::RenderPass;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct GlyphOffset(pub(crate) u32);
pub(crate) struct InstanceRequest {
    pub(crate) character: char,
    pub(crate) scale: Scale,
    pub(crate) hash: GlyphHash,
    pub(crate) position: Position,
    pub(crate) area: Area,
    pub(crate) depth: Depth,
    pub(crate) color: Color,
    pub(crate) placement: Option<rasterization::Placement>,
}
pub(crate) type TextInstanceCoordinator =
    InstanceCoordinator<EntityKey<GlyphOffset>, InstanceRequest>;
pub struct TextRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) rasterization: Rasterization,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) coordinator: TextInstanceCoordinator,
}
impl TextRenderer {
    pub fn new(canvas: &Canvas, viewport: &Viewport) -> Self {
        let rasterization = Rasterization::new(&canvas.device);
        let pipeline = pipeline(canvas, viewport, &rasterization);
        let vertex_buffer = vertex::buffer(&canvas.device);
        let mut coordinator = TextInstanceCoordinator::new(10);
        coordinator.setup_attribute::<Position>(&canvas.device);
        coordinator.setup_attribute::<Area>(&canvas.device);
        coordinator.setup_attribute::<Depth>(&canvas.device);
        coordinator.setup_attribute::<Color>(&canvas.device);
        coordinator.setup_attribute::<rasterization::Placement>(&canvas.device);
        Self {
            pipeline,
            rasterization,
            vertex_buffer,
            coordinator,
        }
    }
}
impl Render for TextRenderer {
    fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>, viewport: &'a Viewport) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &viewport.bind_group, &[]);
        render_pass.set_bind_group(1, &self.rasterization.buffer.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.coordinator.attribute_buffer::<Position>().slice(..));
        render_pass.set_vertex_buffer(2, self.coordinator.attribute_buffer::<Area>().slice(..));
        render_pass.set_vertex_buffer(3, self.coordinator.attribute_buffer::<Depth>().slice(..));
        render_pass.set_vertex_buffer(4, self.coordinator.attribute_buffer::<Color>().slice(..));
        render_pass.set_vertex_buffer(
            5,
            self.coordinator
                .attribute_buffer::<rasterization::Placement>()
                .slice(..),
        );
        if self.coordinator.current() > 0 {
            render_pass.draw(
                0..GLYPH_AABB.len() as u32,
                0..self.coordinator.current() as u32,
            );
        }
    }

    fn extract(&mut self, job: &Job) {
        todo!()
    }

    fn prepare(&mut self, canvas: &Canvas) {
        rasterization::read_requests(&mut self.rasterization, &self.coordinator);
        rasterization::resolve(&mut self.rasterization);
        rasterization::remove(&mut self.rasterization, canvas);
        rasterization::rasterize(&mut self.rasterization);
        rasterization::place(&mut self.rasterization);
        rasterization::write(&mut self.rasterization, canvas);
        rasterization::integrate_placements(&self.rasterization, &mut self.coordinator);
        self.coordinator.prepare();
        self.coordinator.process_attribute(|i| i.position);
        self.coordinator.process_attribute(|i| i.area);
        self.coordinator.process_attribute(|i| i.depth);
        self.coordinator.process_attribute(|i| i.color);
        self.coordinator.process_attribute(|i| i.placement.unwrap());
        self.coordinator.write::<Position>();
        // ...
    }
}
