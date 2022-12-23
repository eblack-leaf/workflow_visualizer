mod attribute;
mod font;
mod pipeline;
mod rasterization;
mod scale;
mod vertex;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::render::Render;
use crate::text::pipeline::pipeline;
use crate::text::rasterization::Rasterization;
use crate::text::vertex::GLYPH_AABB;
use crate::viewport::Viewport;
use crate::{Gfx, Job};
use wgpu::util::DeviceExt;
use wgpu::RenderPass;

pub struct TextRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) rasterization: Rasterization,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) coordinator: attribute::Coordinator,
}
impl TextRenderer {
    pub fn new(canvas: &Canvas, viewport: &Viewport) -> Self {
        let rasterization = Rasterization::new(&canvas.device);
        let pipeline = pipeline(canvas, viewport, &rasterization);
        let vertex_buffer = vertex::buffer(&canvas.device);
        let coordinator = attribute::Coordinator::new(&canvas.device);
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
        render_pass.set_vertex_buffer(1, self.coordinator.buffer::<Position>().slice(..));
        render_pass.set_vertex_buffer(2, self.coordinator.buffer::<Area>().slice(..));
        render_pass.set_vertex_buffer(3, self.coordinator.buffer::<Depth>().slice(..));
        render_pass.set_vertex_buffer(4, self.coordinator.buffer::<Color>().slice(..));
        render_pass.set_vertex_buffer(
            5,
            self.coordinator
                .buffer::<rasterization::Placement>()
                .slice(..),
        );
        if self.coordinator.current() > 0 {
            render_pass.draw(0..GLYPH_AABB.len() as u32, 0..self.coordinator.current());
        }
    }

    fn extract(&mut self, job: &Job) {
        todo!()
    }

    fn prepare(&mut self, canvas: &Canvas) {
        attribute::add(&mut self.coordinator);
        attribute::push_rasterization_requests(&mut self.coordinator, &mut self.rasterization);
        rasterization::resolve(&mut self.rasterization);
        rasterization::remove(&mut self.rasterization, canvas);
        rasterization::rasterize(&mut self.rasterization);
        rasterization::place(&mut self.rasterization);
        rasterization::write(&mut self.rasterization, canvas);
        attribute::read_rasterizations(&mut self.coordinator, &mut self.rasterization);
    }
    fn respond(&mut self, job: &mut Job) {
        job.container
            .spawn_batch(self.coordinator.indexer_responses.drain(..));
    }
}
