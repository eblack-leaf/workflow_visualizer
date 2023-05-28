use std::ops::Add;

use bevy_ecs::component::Component;
use bytemuck::{Pod, Zeroable};
/// Layer represents what plane this entity resides on. Used to differentiate z coords in
/// rendering.
#[repr(C)]
#[derive(Component, Copy, Clone, PartialOrd, PartialEq, Default, Pod, Zeroable)]
pub struct Layer {
    pub z: f32,
}

impl Layer {
    pub fn new(z: f32) -> Self {
        Self { z }
    }
}
impl From<u32> for Layer {
    fn from(value: u32) -> Self {
        Self::new(value as f32)
    }
}
impl From<i32> for Layer {
    fn from(value: i32) -> Self {
        Layer::new(value as f32)
    }
}
impl Add for Layer {
    type Output = Layer;
    fn add(self, rhs: Self) -> Self::Output {
        Layer::new(self.z + rhs.z)
    }
}
