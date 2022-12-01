use bevy_ecs::prelude::{Commands, Res};
use wgpu::BufferAddress;

pub(crate) mod cpu;
pub(crate) mod gpu;
pub struct GlyphRasterization {
    pub rasterization: Vec<u32>,
}
pub fn setup(mut cmd: Commands, device: Res<wgpu::Device>) {
    cmd.insert_resource(cpu::Rasterization { buffer: vec![] });
    let size = 1024;
    cmd.insert_resource(gpu::Rasterization {
        buffer: device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rasterization"),
            size: size as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        }),
        size,
    });
}
