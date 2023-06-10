use bevy_ecs::prelude::Resource;

/// Scale Factor of the device's Window
#[derive(Resource, Clone, Copy)]
pub struct ScaleFactor {
    pub(crate) factor: f64,
}

impl ScaleFactor {
    pub(crate) fn new(factor: f64) -> Self {
        Self { factor }
    }
    pub fn factor(&self) -> f64 {
        self.factor
    }
}

impl From<f64> for ScaleFactor {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}
