use wgpu::util::DeviceExt;
use wgpu::BufferAddress;

use crate::text_refactor::instance::Instance;

pub struct Index(pub usize);

pub struct Instances {
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
    pub size: usize,
}

impl Instances {
    pub fn new(device: &wgpu::Device, size: usize) -> Self {
        Self {
            instance_buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("instances"),
                size: size as BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            instance_count: 0,
            size,
        }
    }
}
