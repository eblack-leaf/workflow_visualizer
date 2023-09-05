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
