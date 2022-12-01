use crate::text::attribute;
use std::marker::PhantomData;
use wgpu::BufferAddress;

pub struct Attributes<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
> {
    pub buffer: wgpu::Buffer,
    _phantom: PhantomData<Attribute>,
}

impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default>
    Attributes<Attribute>
{
    pub fn new(device: &wgpu::Device, size: u32) -> Self {
        Self {
            buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("instances"),
                size: attribute::attribute_size::<Attribute>(size) as BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            _phantom: PhantomData,
        }
    }
}
