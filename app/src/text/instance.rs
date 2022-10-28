use wgpu::VertexAttribute;

use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::gpu_bindings::attributes;
use crate::text::rasterizer::RasterizationKey;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct Instance {
    pub color: Color,
    pub position: Position,
    pub area: Area,
    pub depth: Depth,
    pub rasterization_key: RasterizationKey,
}

impl Instance {
    pub fn new(position: Position, area: Area, depth: Depth, color: Color, rasterization_key: RasterizationKey) -> Self {
        Self {
            position,
            area,
            depth,
            color,
            rasterization_key,
        }
    }
    pub fn attributes() -> [VertexAttribute; 5] {
        wgpu::vertex_attr_array![attributes::TEXT_COLOR => Float32x4, attributes::TEXT_POSITION => Float32x2,
            attributes::TEXT_AREA => Float32x2, attributes::TEXT_DEPTH => Float32, attributes::TEXT_RASTERIZATION_KEY => Uint32x2]
    }
}