use bevy_ecs::prelude::Component;
#[derive(Component)]
pub struct Scale {
    pub scale: f32,
}
impl From<f32> for Scale {
    fn from(scale: f32) -> Self {
        Self { scale }
    }
}
impl From<u32> for Scale {
    fn from(scale: u32) -> Self {
        Self {
            scale: scale as f32,
        }
    }
}
