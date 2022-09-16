use crate::coord::Panel;
use bevy_ecs::prelude::Component;
use std::collections::HashMap;
use crate::uniform::Uniform;
#[derive(Component, Clone)]
pub struct Text {
    pub text_lines: String,
}
pub struct Font {
    pub font: fontdue::Font,
    pub scale: f32,
}
impl Font {
    pub fn new(data: &[u8], scale: f32) -> Self {
        Self {
            font: fontdue::Font::from_bytes(
                data,
                fontdue::FontSettings {
                    scale,
                    ..fontdue::FontSettings::default()
                },
            )
            .expect("could not build font out of given font bytes"),
            scale,
        }
    }
    pub fn horizontal_line_size(&self) -> Option<f32> {
        if let Some(metrics) = self.font.horizontal_line_metrics(self.scale) {
            return Some(metrics.new_line_size);
        }
        return None;
    }
}
pub struct GlyphData {
    pub character: char,
    pub uniform: Uniform<()>,
    pub bitmap: Vec<u8>,
    pub metrics: fontdue::Metrics
}
pub struct Glyphs {
    pub cache: HashMap<char, GlyphData>,
}
