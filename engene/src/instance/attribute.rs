use bevy_ecs::prelude::Resource;
use std::marker::PhantomData;
use wgpu::BufferAddress;
#[derive(Resource)]
pub(crate) struct AttributeBuffer<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
> {
    pub(crate) buffer: wgpu::Buffer,
    _phantom_data: PhantomData<Attribute>,
}

impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default>
    AttributeBuffer<Attribute>
{
    pub(crate) fn new(buffer: wgpu::Buffer) -> Self {
        Self {
            buffer,
            _phantom_data: PhantomData,
        }
    }
}

pub(crate) fn gpu_buffer<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
>(
    device: &wgpu::Device,
    max_instances: usize,
) -> AttributeBuffer<Attribute> {
    AttributeBuffer::new(device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("attribute buffer"),
        size: attribute_size::<Attribute>(max_instances) as BufferAddress,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
}

pub(crate) fn attribute_size<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
>(
    num: usize,
) -> usize {
    std::mem::size_of::<Attribute>() * num
}
#[derive(Resource)]
pub(crate) struct CpuBuffer<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
> {
    pub(crate) buffer: Vec<Attribute>,
}
impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default>
    CpuBuffer<Attribute>
{
    pub(crate) fn new(max: usize) -> Self {
        Self {
            buffer: {
                let mut buffer = Vec::new();
                buffer.resize(max, Attribute::default());
                buffer
            },
        }
    }
}
