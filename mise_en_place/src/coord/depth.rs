use bevy_ecs::component::Component;
use bytemuck::{Pod, Zeroable};

use crate::coord::depth_adjust::DepthAdjust;

#[repr(C)]
#[derive(Component, Pod, Zeroable, Copy, Clone, PartialEq)]
pub struct Depth {
    pub layer: f32,
}

impl Depth {
    pub fn new<T: Into<u32>>(layer: T) -> Self {
        let layer = layer.into();
        Self {
            layer: layer as f32,
        }
    }
    pub fn adjust<Adjust: Into<DepthAdjust>>(&mut self, adjust: Adjust) {
        let adjust = adjust.into();
        self.layer += adjust.layer;
    }
}

impl From<u32> for Depth {
    fn from(value: u32) -> Self {
        Depth::new(value)
    }
}
