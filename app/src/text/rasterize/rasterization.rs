use bevy_ecs::prelude::{Commands, Res};
use wgpu::BufferAddress;

pub struct GlyphRasterization {
    pub rasterization: Vec<u32>,
}
pub struct CpuRasterization {
    pub buffer: Vec<GlyphRasterization>,
}
pub struct GpuRasterization {
    pub buffer: wgpu::Buffer,
    pub size: u32,
}
pub fn setup(
    mut cmd: Commands,
    device: Res<wgpu::Device>,

) {
    cmd.insert_resource(CpuRasterization{ buffer: vec![] });
    let size = 1024;
    cmd.insert_resource(GpuRasterization{
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
