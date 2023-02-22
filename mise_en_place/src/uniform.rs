use std::marker::PhantomData;

use bevy_ecs::prelude::{Component, Resource};
use wgpu::Buffer;
use wgpu::util::DeviceExt;

#[derive(Component, Resource)]
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
}
