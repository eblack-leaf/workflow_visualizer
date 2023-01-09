use bevy_ecs::prelude::Resource;
use std::marker::PhantomData;
use wgpu::BufferAddress;
#[derive(Resource)]
pub(crate) struct GpuBuffer<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
> {
    pub(crate) buffer: wgpu::Buffer,
    _phantom_data: PhantomData<Attribute>,
}

impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default>
    GpuBuffer<Attribute>
{
    pub(crate) fn new(device: &wgpu::Device, max_instances: usize) -> Self {
        Self {
            buffer: {
                device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("attribute buffer"),
                    size: attribute_size::<Attribute>(max_instances) as BufferAddress,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                })
            },
            _phantom_data: PhantomData,
        }
    }
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

pub trait AttributeHandler<Request>
where
    Self: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
{
    fn extract(request: &Request) -> Self;
    fn null() -> Self;
}
