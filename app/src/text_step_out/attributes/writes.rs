use crate::text_step_out::attributes::{Index};
use bevy_ecs::prelude::{Res, ResMut};
use crate::text_step_out::attributes::buffers::{attribute_size, AttributeBuffer};

pub struct Write<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> {
    pub data: Attribute,
    pub index: Index,
}
pub struct Writes<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> {
    pub writes: Vec<Write<Attribute>>,
}
pub fn write_attribute<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone>(
    writes: ResMut<Writes<Attribute>>,
    attributes: Res<AttributeBuffer<Attribute>>,
    queue: Res<wgpu::Queue>,
) {
    // aggregate first to save writes if needed
    // add to cpu_attributes
    for write in writes.writes.drain(..) {
        queue.write_buffer(
            &attributes.attributes,
            attribute_size::<Attribute>(write.index.0),
            bytemuck::cast_slice(&write.data),
        );
    }
}
