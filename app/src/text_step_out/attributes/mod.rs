mod add;
mod buffers;
mod remove;
mod writes;
use crate::color::Color;
use crate::coord::{Area, Depth, Position};
pub use crate::text_step_out::attributes::buffers::{AttributeBuffer, setup_attribute_buffers};
use crate::text_step_out::rasterization::placement::RasterizationPlacement;

pub struct Index(pub usize);
pub struct Coordinator {
    pub cpu_attributes: CpuAttributes,
    pub current: u32,
    pub max: u32,
}
impl Coordinator {
    pub fn new(max: u32) -> Self {
        Self { cpu_attributes: CpuAttributes::new(), current: 0, max }
    }
}
pub struct CpuAttributes {
    pub positions: Vec<Position>,
    pub areas: Vec<Area>,
    pub depths: Vec<Depth>,
    pub colors: Vec<Color>,
    pub rasterization_placements: Vec<RasterizationPlacement>,
}
impl CpuAttributes {
    pub fn new() -> Self {
        Self {
            positions: vec![],
            areas: vec![],
            depths: vec![],
            colors: vec![],
            rasterization_placements: vec![]
        }
    }
}
