use bevy_ecs::prelude::Resource;
use crate::color::Color;
#[derive(Resource)]
pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub secondary: Color,
}
impl Theme {
    pub fn new<C: Into<Color>>(background: C, foreground: C, secondary: C) -> Self {
        Self {
            background: background.into(),
            foreground: foreground.into(),
            secondary: secondary.into(),
        }
    }
}
impl Default for Theme {
    fn default() -> Self {
        Self::new((0f32, 0f32, 0f32), (1f32, 1f32, 1f32), (0.5, 0.5, 0.5))
    }
}
