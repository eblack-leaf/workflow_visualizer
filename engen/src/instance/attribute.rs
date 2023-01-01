use std::marker::PhantomData;
use wgpu::BufferAddress;

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

pub(crate) fn cpu_buffer<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
>(
    initial_max: usize,
) -> Vec<Attribute> {
    let mut buffer = Vec::new();
    buffer.resize(initial_max, Attribute::default());
    buffer
}
