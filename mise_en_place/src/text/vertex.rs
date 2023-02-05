use crate::coord::Position;
use crate::gfx::GfxSurface;
use wgpu::util::DeviceExt;

pub(crate) const GLYPH_AABB: [Vertex; 6] = [
    Vertex::new(Position { x: 0.0, y: 0.0 }),
    Vertex::new(Position { x: 0.0, y: 1.0 }),
    Vertex::new(Position { x: 1.0, y: 0.0 }),
    Vertex::new(Position { x: 1.0, y: 0.0 }),
    Vertex::new(Position { x: 0.0, y: 1.0 }),
    Vertex::new(Position { x: 1.0, y: 1.0 }),
];

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub(crate) struct Vertex {
    pub position: Position,
}

impl Vertex {
    pub const fn new(position: Position) -> Self {
        Self { position }
    }
}

pub(crate) fn vertex_buffer(gfx_surface: &GfxSurface) -> wgpu::Buffer {
    gfx_surface
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("text vertex buffer"),
            contents: bytemuck::cast_slice(&GLYPH_AABB),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
}
