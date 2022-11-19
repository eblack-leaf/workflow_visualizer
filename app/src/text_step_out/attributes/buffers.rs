use crate::text_step_out::attributes::Coordinator;
use bevy_ecs::prelude::{Commands, Res};
use wgpu::BufferAddress;

pub fn setup_attribute_buffers<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone>(
    coordinator: Res<Coordinator>,
    device: Res<wgpu::Device>,
    mut cmd: Commands,
) {
    cmd.insert_resource(AttributeBuffer::<Attribute>::new(&device, coordinator.max))
}
pub fn attribute_size<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone>(
    num: usize,
) -> wgpu::BufferAddress {
    (std::mem::size_of::<Attribute>() * num) as BufferAddress
}
pub struct AttributeBuffer<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> {
    pub attributes: wgpu::Buffer,
    pub size: wgpu::BufferAddress,
}
impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> AttributeBuffer<Attribute> {
    pub fn new(device: &wgpu::Device, max: usize) -> Self {
        let attr_size = (attribute_size::<Attribute>(max)) as BufferAddress;
        Self {
            attributes: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("instances"),
                size: attr_size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            size: attr_size,
        }
    }
}
