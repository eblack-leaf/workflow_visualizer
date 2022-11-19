use wgpu::util::DeviceExt;
use wgpu::BufferAddress;

use crate::gpu_bindings::bindings;

pub struct Rasterizations {
    pub cpu: Vec<u8>,
    pub gpu: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub size: u32,
    pub free: u32,
}

impl Rasterizations {
    pub fn new(device: &wgpu::Device, size: u32) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("rasterizer bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: bindings::RASTERIZATION,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let gpu = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rasterization"),
            size: size as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("rasterizer bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: bindings::RASTERIZATION,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &gpu,
                    offset: 0,
                    size: None,
                }),
            }],
        });
        Self {
            cpu: Vec::new(),
            gpu,
            bind_group,
            bind_group_layout,
            size,
            free: size,
        }
    }
}
