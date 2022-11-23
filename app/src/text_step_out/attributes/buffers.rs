use crate::text_step_out::attributes::Coordinator;
use bevy_ecs::prelude::{Commands, Res};
use wgpu::BufferAddress;

pub fn setup_attribute_buffers<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone>(
    coordinator: Res<Coordinator>,
    device: Res<wgpu::Device>,
    mut cmd: Commands,
) {
    cmd.insert_resource(CpuAttributes::<Attribute>::new(coordinator.max));
    cmd.insert_resource(GpuAttributes::<Attribute>::new(&device, coordinator.max))
}
pub fn attribute_size<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone>(
    num: u32,
) -> wgpu::BufferAddress {
    (std::mem::size_of::<Attribute>() * num) as BufferAddress
}
pub struct CpuAttributes<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> {
    pub attributes: Vec<Attribute>,
}
impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> CpuAttributes<Attribute> {
    pub fn new(max: u32) -> Self {
        Self {
            attributes: {
                let mut attrs = Vec::new();
                attrs.reserve(max as usize);
                attrs
            },
        }
    }
}
pub struct GpuAttributes<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> {
    pub buffer: wgpu::Buffer,
    pub size: wgpu::BufferAddress,
}
impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> GpuAttributes<Attribute> {
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
        }
    }
}
