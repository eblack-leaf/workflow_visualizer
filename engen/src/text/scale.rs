use bevy_ecs::component::Component;

#[derive(Component, Clone, Copy, Hash, Eq, PartialEq)]
pub struct TextScale {
    pub scale: u32,
}

impl TextScale {
    pub fn new(scale: u32) -> Self {
        Self { scale }
    }
    pub fn px(&self) -> f32 {
        self.scale as f32
    }
}

impl From<f32> for TextScale {
    fn from(scale: f32) -> Self {
        Self {
            scale: scale as u32,
        }
    }
}

impl From<u32> for TextScale {
    fn from(scale: u32) -> Self {
        Self { scale }
    }
}
