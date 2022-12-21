mod attribute;
mod font;
mod pipeline;
mod r_rasterization;
mod scale;
mod vertex;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::render::Render;
use crate::text::pipeline::pipeline;
use crate::text::r_rasterization::Rasterization;
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
    pub(crate) position_buffer: wgpu::Buffer,
    pub(crate) area_buffer: wgpu::Buffer,
    pub(crate) depth_buffer: wgpu::Buffer,
    pub(crate) color_buffer: wgpu::Buffer,
    pub(crate) placement_buffer: wgpu::Buffer,
}
impl TextRenderer {
    pub fn new(canvas: &Canvas, viewport: &Viewport) -> Self {
        let rasterization = Rasterization::new(&canvas.device);
        let pipeline = pipeline(canvas, viewport, &rasterization);
        let vertex_buffer = vertex::buffer(&canvas.device);
        let coordinator = attribute::Coordinator::new(100);
        let position_buffer = attribute::buffer::<Position>(&canvas.device, coordinator.max);
        let area_buffer = attribute::buffer::<Area>(&canvas.device, coordinator.max);
        let depth_buffer = attribute::buffer::<Depth>(&canvas.device, coordinator.max);
        let color_buffer = attribute::buffer::<Color>(&canvas.device, coordinator.max);
        let placement_buffer =
            attribute::buffer::<r_rasterization::Placement>(&canvas.device, coordinator.max);
        Self {
            pipeline,
            rasterization,
            vertex_buffer,
            coordinator,
            position_buffer,
            area_buffer,
            depth_buffer,
            color_buffer,
            placement_buffer,
        }
    }
}
impl Render for TextRenderer {
    fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>, viewport: &'a Viewport) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &viewport.bind_group, &[]);
        render_pass.set_bind_group(1, &self.rasterization.buffer.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.position_buffer.slice(..));
        render_pass.set_vertex_buffer(2, self.area_buffer.slice(..));
        render_pass.set_vertex_buffer(3, self.depth_buffer.slice(..));
        render_pass.set_vertex_buffer(4, self.color_buffer.slice(..));
        render_pass.set_vertex_buffer(5, self.placement_buffer.slice(..));
        if self.coordinator.current > 0 {
            render_pass.draw(0..GLYPH_AABB.len() as u32, 0..self.coordinator.current);
        }
    }

    fn extract(&mut self, job: Job) {
        todo!()
    }

    fn prepare(&mut self, canvas: &Canvas) {
        r_rasterization::resolve(&mut self.rasterization);
        r_rasterization::remove(&mut self.rasterization, canvas);
        r_rasterization::rasterize(&mut self.rasterization);
        r_rasterization::place(&mut self.rasterization);
        r_rasterization::write(&mut self.rasterization, canvas);
    }
}
