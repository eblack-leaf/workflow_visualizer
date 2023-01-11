use std::ops::Deref;
use std::path::Path;

use bevy_ecs::prelude::Resource;
use fontdue::{Font as fdFont, FontSettings};

use crate::text::scale::Scale;

#[derive(Resource)]
pub struct Font {
    pub font_storage: [fdFont; 1],
}

impl Font {
    pub fn new<Data: Deref<Target=[u8]>, T: Into<Scale>>(font_data: Data, opt_scale: T) -> Self {
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
    pub fn advance_width(&self, character: char, scale: f32) -> f32 {
        let metrics = self.font().metrics(character, scale);
        metrics.advance_width
    }
}

impl Default for Font {
    fn default() -> Self {
        Font::new(
            include_bytes!("./JetBrainsMono-Medium.ttf").as_slice(),
            15u32,
        )
    }
}
