use bevy_ecs::prelude::Component;
use std::collections::HashMap;
#[derive(Component, Clone)]
pub struct Text {
    pub text: String,
}
pub struct Font {
    pub font: fontdue::Font,
    pub scale: f32,
}
impl Font {
    pub fn new(data: &[u8], scale: f32) -> Self {
        Self {
            font: fontdue::Font::from_bytes(data, fontdue::FontSettings{ scale, ..fontdue::FontSettings::default()})
                .expect("could not build font out of given bytes"),
            scale,
        }
    }
}
pub struct TextRegion {

}
