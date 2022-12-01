use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::text::attribute::coordinator::Coordinator;
use crate::text::rasterize::placement::Placement;
use bevy_ecs::prelude::{Commands, Res};
use std::marker::PhantomData;
use wgpu::BufferAddress;

pub(crate) mod coordinator;
pub(crate) mod cpu;
pub(crate) mod gpu;

pub fn attribute_size<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
>(
    num: u32,
) -> u32 {
    std::mem::size_of::<Attribute>() as u32 * num
}

pub fn setup(device: Res<wgpu::Device>, mut cmd: Commands) {
    let initial_max = 10;
    cmd.insert_resource(Coordinator::new(initial_max));
    cmd.insert_resource(cpu::Attributes::<Position>::new(initial_max));
    cmd.insert_resource(gpu::Attributes::<Position>::new(&device, initial_max));
    cmd.insert_resource(cpu::Attributes::<Area>::new(initial_max));
    cmd.insert_resource(gpu::Attributes::<Area>::new(&device, initial_max));
    cmd.insert_resource(cpu::Attributes::<Depth>::new(initial_max));
    cmd.insert_resource(gpu::Attributes::<Depth>::new(&device, initial_max));
    cmd.insert_resource(cpu::Attributes::<Color>::new(initial_max));
    cmd.insert_resource(gpu::Attributes::<Color>::new(&device, initial_max));
    cmd.insert_resource(cpu::Attributes::<Placement>::new(initial_max));
    cmd.insert_resource(gpu::Attributes::<Placement>::new(&device, initial_max));
}
