pub(crate) mod add;
mod buffers;
pub(crate) mod remove;
pub(crate) mod write;

use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::text_step_out::attributes::add::Adds;
pub use crate::text_step_out::attributes::buffers::{
    setup_attribute_buffers, CpuAttributes, GpuAttributes,
};
use crate::text_step_out::attributes::remove::{Removes, Swaps};
use crate::text_step_out::attributes::write::Writes;
use crate::text_step_out::rasterization::placement::RasterizationPlacement;
use bevy_ecs::prelude::Commands;

#[derive(Clone, Copy)]
pub struct Index(pub u32);
pub struct Coordinator {
    pub current: u32,
    pub max: u32,
    pub growth: Option<u32>,
}
impl Coordinator {
    pub fn new(max: u32) -> Self {
        Self {
            current: 0,
            max,
            growth: None,
        }
    }
    pub fn generate_index(&mut self) -> Index {
        self.current += 1;
        Index(self.current)
    }
    pub fn current_index(&self) -> Index {
        Index(self.current - 1)
    }
    pub fn shrink(&mut self) {
        if self.current == 0 {
            return;
        }
        self.current -= 1;
    }
}
pub fn setup_attribute_queues<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync,
>(
    mut cmd: Commands,
) {
    cmd.insert_resource(Coordinator::new(100));
    cmd.insert_resource(Adds::<Attribute>::new());
    cmd.insert_resource(Writes::<Attribute>::new());
    cmd.insert_resource(Removes::new());
    cmd.insert_resource(Swaps::new());
}
