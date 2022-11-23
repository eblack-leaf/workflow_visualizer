pub(crate) mod add;
mod buffers;
pub(crate) mod remove;
pub(crate) mod write;

use bevy_ecs::prelude::Commands;
use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::text_step_out::attributes::add::Adds;
pub use crate::text_step_out::attributes::buffers::{setup_attribute_buffers, GpuAttributes};
use crate::text_step_out::attributes::write::Writes;
use crate::text_step_out::rasterization::placement::RasterizationPlacement;

#[derive(Clone, Copy)]
pub struct Index(pub u32);
pub struct Coordinator {
    pub current: u32,
    pub max: u32,
}
impl Coordinator {
    pub fn new(max: u32) -> Self {
        Self { current: 0, max }
    }
    pub fn generate_index(&mut self) -> Index {
        self.current += 1;
        Index(self.current)
    }
}
pub fn setup_attributes<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone>(mut cmd: Commands) {
    cmd.insert_resource(Adds::<Attribute>::new());
    cmd.insert_resource(Writes::<Attribute>::new());
    // removes
}
