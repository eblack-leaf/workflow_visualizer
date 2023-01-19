use bevy_ecs::component::Component;

#[derive(Component, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Scale {
    pub scale: u32,
}

impl Scale {
    pub fn new(scale: u32) -> Self {
        Self { scale }
    }
    pub fn px(&self) -> f32 {
        self.scale as f32
    }
}

impl From<f32> for Scale {
    fn from(scale: f32) -> Self {
        Self {
            scale: scale as u32,
        }
    }
}

impl From<u32> for Scale {
    fn from(scale: u32) -> Self {
        Self { scale }
    }
}
