use std::ops::Deref;
use std::path::Path;

use crate::r_text::{GlyphHash, Scale};
use bevy_ecs::prelude::Resource;
use fontdue::{Font as fdFont, FontSettings};

#[derive(Resource)]
pub struct Font {
    pub font_storage: [fdFont; 1],
}

impl Font {
    pub fn new<Data: Deref<Target = [u8]>, T: Into<Scale>>(font_data: Data, opt_scale: T) -> Self {
        Self {
            font_storage: [fdFont::from_bytes(
                font_data,
                FontSettings {
                    scale: opt_scale.into().scale,
                    ..FontSettings::default()
                },
            )
            .expect("text font creation")],
        }
    }
    pub fn font_slice(&self) -> &[fdFont] {
        self.font_storage.as_slice()
    }
    pub fn font(&self) -> &fdFont {
        &self.font_storage[0]
    }
    pub fn index() -> usize {
        0
    }
    pub fn character_dimensions(&self, character: char, px: f32) -> [f32; 2] {
        let metrics = self.font().metrics(character, px);
        [metrics.advance_width, metrics.advance_height]
    }
}

impl Default for Font {
    fn default() -> Self {
        Font::new(
            include_bytes!("../r_text/JetBrainsMono-Medium.ttf").as_slice(),
            15u32,
        )
    }
}
