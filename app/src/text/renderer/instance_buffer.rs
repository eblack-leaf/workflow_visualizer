use wgpu::util::DeviceExt;

use crate::text::instance::Instance;

pub struct GlyphInstanceBuffer {
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
    pub instances: Vec<Instance>,
}

impl GlyphInstanceBuffer {
    pub fn new(device: &wgpu::Device, instances: Vec<Instance>) -> Self {
        Self {
            instance_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("glyph instance buffer"),
                contents: bytemuck::cast_slice(&instances),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
            instance_count: instances.len() as u32,
            instances,
        }
    }
}
