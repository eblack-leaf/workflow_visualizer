use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::gpu_bindings::bindings;
use crate::text::attribute::Coordinator;
use crate::text::attribute::GpuAttributes;
use crate::text::rasterize::RasterizationBinding;
use crate::text::vertex_buffer::{VertexBuffer, GLYPH_AABB};
use crate::text::{Pipeline, RasterizationPlacement};
use crate::viewport::ViewportBinding;
pub fn render<'a>(
    mut render_pass: &mut wgpu::RenderPass<'a>,
    pipeline: &'a Pipeline,
    viewport_binding: &'a ViewportBinding,
    rasterization_binding: &'a RasterizationBinding,
    coordinator: &'a Coordinator,
    positions: &'a GpuAttributes<Position>,
    areas: &'a GpuAttributes<Area>,
    depths: &'a GpuAttributes<Depth>,
    colors: &'a GpuAttributes<Color>,
    rasterization_placements: &'a GpuAttributes<RasterizationPlacement>,
    vertex_buffer: &'a VertexBuffer,
) {
    render_pass.set_pipeline(&pipeline.pipeline);
    render_pass.set_bind_group(bindings::VIEWPORT, &viewport_binding.bind_group, &[]);
    render_pass.set_bind_group(
        bindings::RASTERIZATION,
        &rasterization_binding.bind_group,
        &[],
    );
    render_pass.set_vertex_buffer(0, vertex_buffer.buffer.slice(..));
    render_pass.set_vertex_buffer(1, positions.buffer.slice(..));
    render_pass.set_vertex_buffer(2, areas.buffer.slice(..));
    render_pass.set_vertex_buffer(3, depths.buffer.slice(..));
    render_pass.set_vertex_buffer(4, colors.buffer.slice(..));
    render_pass.set_vertex_buffer(5, rasterization_placements.buffer.slice(..));
    if coordinator.current > 0 {
        render_pass.draw(0..GLYPH_AABB.len() as u32, 0..coordinator.current);
    }
}
