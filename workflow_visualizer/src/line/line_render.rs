use bevy_ecs::component::Component;
use wgpu::util::DeviceExt;

use crate::{DeviceContext, GfxSurface, Position};

#[derive(Component)]
pub struct LineRenderPoints {
    pub(crate) points: Vec<Position<DeviceContext>>,
}

#[derive(Component)]
pub struct LineRender {
    pub(crate) capacity: usize,
}

impl LineRender {
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            capacity,
        }
    }
}
