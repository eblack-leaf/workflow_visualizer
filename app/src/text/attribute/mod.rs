use wgpu::BufferAddress;

mod coordinator;
mod cpu;
mod gpu;
pub fn attribute_size<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync,
>(
    num: u32,
) -> usize {
    std::mem::size_of::<Attribute>() * num
}
