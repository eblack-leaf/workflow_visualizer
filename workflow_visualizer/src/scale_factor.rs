use crate::{Area, DeviceContext};
use bevy_ecs::prelude::Resource;

/// Scale Factor of the device's Window
#[derive(Resource, Clone, Copy)]
pub struct ScaleFactor {
    factor: f32,
}

impl ScaleFactor {
    pub(crate) fn new(factor: f32) -> Self {
        Self { factor }
    }
    pub fn factor(&self) -> f32 {
        self.factor
    }
}

impl From<f32> for ScaleFactor {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}
#[derive(Resource, Copy, Clone)]
pub struct WindowAppearanceFactor {
    x_factor: f32,
    y_factor: f32,
}
impl WindowAppearanceFactor {
    pub(crate) fn new(requested: Area<DeviceContext>, actual: Area<DeviceContext>) -> Self {
        Self {
            x_factor: requested.width / actual.width,
            y_factor: requested.height / actual.height,
        }
    }
    pub fn x_factor(&self) -> f32 {
        self.x_factor
    }
    pub fn y_factor(&self) -> f32 {
        self.y_factor
    }
}
