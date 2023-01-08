use crate::text::InstanceCoordinator;
use bevy_ecs::prelude::Resource;

mod descriptor;
pub(crate) use descriptor::Descriptor;
pub(crate) struct Binding {
    pub(crate) cpu: Vec<u32>,
    pub(crate) gpu: wgpu::Buffer,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) gpu_len: usize,
    pub(crate) growth_factor: usize,
}
impl Binding {
    pub(crate) fn new(device: &wgpu::Device, num_elements: usize) -> Self {
        let size = bytes(num_elements);
        let mut cpu = Vec::new();
        cpu.resize(num_elements, 0);
        let gpu = Self::buffer(device, size);
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("rasterizer bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("rasterizer bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &gpu,
                    offset: 0,
                    size: None,
                }),
            }],
        });
        Self {
            cpu,
            gpu,
            bind_group,
            bind_group_layout,
            gpu_len: 0,
            growth_factor: 256,
        }
    }

    fn buffer(device: &wgpu::Device, size: usize) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rasterization buffer"),
            size: size as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        })
    }
}
pub(crate) fn bytes(num: usize) -> usize {
    num * std::mem::size_of::<u32>()
}
pub(crate) struct Handler {
    pub(crate) binding: Binding,
}
impl Handler {
    pub(crate) fn new(device: &wgpu::Device) -> Self {
        Self {
            binding: Binding::new(device, 10),
        }
    }
    pub(crate) fn read_requests(&mut self, coordinator: &InstanceCoordinator) {}
    pub(crate) fn integrate_requests(&mut self, coordinator: &mut InstanceCoordinator) {}
}
