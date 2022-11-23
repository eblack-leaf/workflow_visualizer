use crate::text_step_out::attributes::buffers::{attribute_size, CpuAttributes, GpuAttributes};
use crate::text_step_out::attributes::{Coordinator, Index};
use bevy_ecs::prelude::{Res, ResMut};

pub struct Write<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> {
    pub index: Index,
    pub attribute: Attribute,
}
impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> Write<Attribute> {
    pub fn new(index: Index, attribute: Attribute) -> Self {
        Self { index, attribute }
    }
}
pub struct Writes<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> {
    pub writes: Vec<Write<Attribute>>,
}
impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone> Writes<Attribute> {
    pub fn new() -> Self {
        Self {
            writes: Vec::new(),
        }
    }
}
pub fn write_attribute<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone>(
    writes: ResMut<Writes<Attribute>>,
    attributes: Res<GpuAttributes<Attribute>>,
    queue: Res<wgpu::Queue>,
    mut cpu_attributes: ResMut<CpuAttributes<Attribute>>,
) {
    // aggregate first to save writes if needed
    for write in writes.writes.drain(..) {
        *cpu_attributes.attributes.get_mut(write.index.0 as usize) = write.attribute;
        queue.write_buffer(
            &attributes.buffer,
            attribute_size::<Attribute>(write.index.0 as u32),
            bytemuck::cast_slice(&write.attribute),
        );
    }
}
