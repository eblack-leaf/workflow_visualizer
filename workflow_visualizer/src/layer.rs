use bevy_ecs::component::Component;

#[derive(Component, Copy, Clone, PartialOrd, PartialEq, Default)]
pub struct Layer {
    pub z: f32
}

impl Layer {
    pub fn new(z: f32) -> Self {
        Self {
            z
        }
    }
}
impl From<u32> for Layer {
    fn from(value: u32) -> Self {
        Self::new(value as f32)
    }
}