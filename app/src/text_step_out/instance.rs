use wgpu::VertexAttribute;

use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::text_step_out::rasterization::placement::RasterizationPlacement;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct Instance {
    pub position: Position,
    pub area: Area,
    pub depth: Depth,
    pub color: Color,
    pub rasterization_descriptor: RasterizationPlacement,
}

impl Instance {
    pub fn new(
        position: Position,
        area: Area,
        depth: Depth,
        color: Color,
        rasterization_descriptor: RasterizationPlacement,
    ) -> Self {
        Self {
            position,
            area,
            depth,
            color,
            rasterization_descriptor,
        }
    }
}
