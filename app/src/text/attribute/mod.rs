pub use coordinator::Coordinator;
use std::marker::PhantomData;
use wgpu::BufferAddress;

mod coordinator;
pub fn attribute_size<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync,
>(
    num: u32,
) -> u32 {
    std::mem::size_of::<Attribute>() as u32 * num
}

pub struct CpuAttributes<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync>
{
    pub buffer: Vec<Attribute>,
}

pub struct GpuAttributes<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync>
{
    pub buffer: wgpu::Buffer,
    _phantom: PhantomData<Attribute>,
}
