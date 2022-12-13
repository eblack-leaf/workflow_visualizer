use wgpu::BufferAddress;
pub(crate) struct Coordinator {
    pub(crate) current: u32,
    pub(crate) max: u32,
}
impl Coordinator {
    pub(crate) fn new(max: u32) -> Self {
        Self { current: 0, max }
    }
}

pub(crate) fn buffer<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
>(
    device: &wgpu::Device,
    max_instances: u32,
) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("attribute buffer"),
        size: attribute_size::<Attribute>(max_instances) as BufferAddress,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}
pub(crate) fn attribute_size<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
>(
    num: u32,
) -> u32 {
    std::mem::size_of::<Attribute>() as u32 * num
}
