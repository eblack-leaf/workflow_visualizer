use bevy_ecs::component::Component;
use bytemuck::{Pod, Zeroable};

use crate::coord::depth_adjust::DepthAdjust;

#[repr(C)]
#[derive(Component, Pod, Zeroable, Copy, Clone, PartialEq, PartialOrd, Default, Debug)]
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
    pub fn adjusted<Adjust: Into<DepthAdjust>>(&self, adjust: Adjust) -> Self {
        let adjust = adjust.into();
        Self::new((self.layer + adjust.layer) as u32)
    }
}

impl From<u32> for Depth {
    fn from(value: u32) -> Self {
        Depth::new(value)
    }
}
