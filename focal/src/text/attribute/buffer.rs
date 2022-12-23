use std::marker::PhantomData;
use wgpu::BufferAddress;
pub(crate) struct Buffer<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
> {
    pub(crate) buffer: wgpu::Buffer,
    _phantom_data: PhantomData<Attribute>,
}
impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default>
    Buffer<Attribute>
{
    pub(crate) fn new(buffer: wgpu::Buffer) -> Self {
        Self {
            buffer,
            _phantom_data: PhantomData,
        }
    }
}
pub(crate) fn buffer<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
>(
    device: &wgpu::Device,
    max_instances: u32,
) -> Buffer<Attribute> {
    Buffer::new(device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("attribute buffer"),
        size: attribute_size::<Attribute>(max_instances) as BufferAddress,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
}
pub(crate) fn attribute_size<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
>(
    num: u32,
) -> u32 {
    std::mem::size_of::<Attribute>() as u32 * num
}
