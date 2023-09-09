use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use std::marker::PhantomData;

use wgpu::util::DeviceExt;
use wgpu::{BindGroupEntry, BindGroupLayoutEntry, Buffer};

/// Wrapper around wgpu::Buffer for use as a Uniform Buffer
pub struct Uniform<Data: bytemuck::Pod + bytemuck::Zeroable> {
    pub buffer: Buffer,
    _data: PhantomData<Data>,
}

impl<Data: bytemuck::Pod + bytemuck::Zeroable> Uniform<Data> {
    pub fn new(device: &wgpu::Device, data: Data) -> Self {
        return Self {
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Component Buffer"),
                contents: bytemuck::cast_slice(&[data]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }),
            _data: PhantomData,
        };
    }
    pub fn update(&mut self, queue: &wgpu::Queue, data: Data) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[data]));
    }
    pub fn bind_group_entry(&self, binding: u32) -> BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: self.buffer.as_entire_binding(),
        }
    }
}

pub fn vertex_bind_group_layout_entry(binding: u32) -> BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}
pub type AlignedUniformData<Repr> = [Repr; 4];
pub struct AlignedUniform<Repr: Default + Copy + Clone + Pod + Zeroable> {
    pub uniform: Uniform<[Repr; 4]>,
    pub data: [Repr; 4],
}
impl<Repr: Default + Copy + Clone + Pod + Zeroable> AlignedUniform<Repr> {
    pub fn new(device: &wgpu::Device, data: Option<[Repr; 4]>) -> Self {
        let data = data.unwrap_or_default();
        Self {
            uniform: Uniform::new(device, data),
            data,
        }
    }
    pub fn update(&mut self, queue: &wgpu::Queue) {
        self.uniform.update(queue, self.data);
    }
    pub fn set_aspect(&mut self, index: usize, aspect: Repr) {
        self.data[index] = aspect;
    }
}
