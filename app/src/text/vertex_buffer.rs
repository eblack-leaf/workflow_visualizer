use crate::text::vertex::Vertex;
use crate::Position;
use bevy_ecs::prelude::{Commands, Res};
use wgpu::util::DeviceExt;

pub const GLYPH_AABB: [Vertex; 6] = [
    Vertex::new(Position::new(0.0, 0.0)),
    Vertex::new(Position::new(0.0, 1.0)),
    Vertex::new(Position::new(1.0, 0.0)),
    Vertex::new(Position::new(1.0, 0.0)),
    Vertex::new(Position::new(0.0, 1.0)),
    Vertex::new(Position::new(1.0, 1.0)),
];
pub struct VertexBuffer {
    pub buffer: wgpu::Buffer,
}
pub fn setup(device: Res<wgpu::Device>, mut cmd: Commands) {
    let vertex_buffer = VertexBuffer {
        buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("text vertex buffer"),
            contents: bytemuck::cast_slice(&GLYPH_AABB),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        }),
    };
    cmd.insert_resource(vertex_buffer);
}
