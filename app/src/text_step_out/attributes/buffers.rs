use crate::text_step_out::attributes::Coordinator;
use bevy_ecs::prelude::{Commands, Res};
use std::marker::PhantomData;
use wgpu::BufferAddress;

pub fn setup_attribute_buffers<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync,
>(
    coordinator: Res<Coordinator>,
    device: Res<wgpu::Device>,
    mut cmd: Commands,
) {
    cmd.insert_resource(CpuAttributes::<Attribute>::new(coordinator.max));
    cmd.insert_resource(GpuAttributes::<Attribute>::new(&device, coordinator.max))
}
pub fn attribute_size<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync,
>(
    num: u32,
) -> wgpu::BufferAddress {
    (std::mem::size_of::<Attribute>() * num as usize) as BufferAddress
}
pub struct CpuAttributes<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync>
{
    pub attributes: Vec<Attribute>,
}
impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync>
    CpuAttributes<Attribute>
{
    pub fn new(max: u32) -> Self {
        Self {
            attributes: {
                let mut attrs = Vec::new();
                // reserve does not make value there (Default or blank() with resize(len, value))
                attrs.reserve(max as usize);
                attrs
            },
        }
    }
}
pub struct GpuAttributes<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync>
{
    pub buffer: wgpu::Buffer,
    pub size: wgpu::BufferAddress,
    _phantom_data: PhantomData<Attribute>,
}
impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync>
    GpuAttributes<Attribute>
{
    pub fn new(device: &wgpu::Device, max: u32) -> Self {
        let attr_size = (attribute_size::<Attribute>(max)) as BufferAddress;
        Self {
            buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("instances"),
                size: attr_size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            size: attr_size,
            _phantom_data: PhantomData,
        }
    }
}
