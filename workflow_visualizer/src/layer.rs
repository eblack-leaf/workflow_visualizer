use bevy_ecs::component::Component;

#[derive(Component, Copy, Clone)]
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
