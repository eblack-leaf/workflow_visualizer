use wgpu::util::DeviceExt;

use crate::coord::GpuPosition;
use crate::gfx::GfxSurface;

pub(crate) const AABB: [Vertex; 6] = [
    Vertex::new(GpuPosition { x: 0.0, y: 0.0 }),
    Vertex::new(GpuPosition { x: 0.0, y: 1.0 }),
    Vertex::new(GpuPosition { x: 1.0, y: 0.0 }),
    Vertex::new(GpuPosition { x: 1.0, y: 0.0 }),
    Vertex::new(GpuPosition { x: 0.0, y: 1.0 }),
    Vertex::new(GpuPosition { x: 1.0, y: 1.0 }),
];

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub(crate) struct Vertex {
    pub position: GpuPosition,
}

impl Vertex {
    pub const fn new(position: GpuPosition) -> Self {
        Self { position }
    }
}

pub(crate) fn aabb_vertex_buffer(gfx_surface: &GfxSurface) -> wgpu::Buffer {
    gfx_surface
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("text vertex buffer"),
            contents: bytemuck::cast_slice(&AABB),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
}
