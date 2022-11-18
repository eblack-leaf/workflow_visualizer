use wgpu::VertexAttribute;

use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::gpu_bindings::attributes;
use crate::text_refactor::rasterization_descriptor::RasterizationDescriptor;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct Instance {
    pub color: Color,
    pub position: Position,
    pub area: Area,
    pub depth: Depth,
    pub rasterization_descriptor: RasterizationDescriptor,
}

impl Instance {
    pub fn new(
        position: Position,
        area: Area,
        depth: Depth,
        color: Color,
        rasterization_descriptor: RasterizationDescriptor,
    ) -> Self {
        Self {
            position,
            area,
            depth,
            color,
            rasterization_descriptor,
        }
    }
    pub fn attributes() -> [VertexAttribute; 5] {
        wgpu::vertex_attr_array![attributes::TEXT_COLOR => Float32x4, attributes::TEXT_POSITION => Float32x2,
            attributes::TEXT_AREA => Float32x2, attributes::TEXT_DEPTH => Float32, attributes::TEXT_RASTERIZATION_DESCRIPTOR => Uint32x3]
    }
}
