use wgpu::VertexAttribute;

use crate::coord::Position;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct Vertex {
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
