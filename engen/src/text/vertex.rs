use wgpu::util::DeviceExt;
use wgpu::VertexAttribute;

use crate::coord::Position;
pub(crate) const GLYPH_AABB: [Vertex; 6] = [
    Vertex::new(Position::new(0.0, 0.0)),
    Vertex::new(Position::new(0.0, 1.0)),
    Vertex::new(Position::new(1.0, 0.0)),
    Vertex::new(Position::new(1.0, 0.0)),
    Vertex::new(Position::new(0.0, 1.0)),
    Vertex::new(Position::new(1.0, 1.0)),
];
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub(crate) struct Vertex {
    pub position: Position,
}

impl Vertex {
    pub fn attributes<'a>() -> [VertexAttribute; 1] {
        wgpu::vertex_attr_array![0 => Float32x2]
    }
    pub const fn new(position: Position) -> Self {
        Self { position }
    }
}
pub(crate) fn buffer(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("text vertex buffer"),
        contents: bytemuck::cast_slice(&GLYPH_AABB),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    })
}
