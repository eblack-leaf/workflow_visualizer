use std::ops::Deref;
use std::path::Path;
use bevy_ecs::prelude::Resource;
use fontdue::{Font as fdFont, FontSettings};
use crate::clean_text::scale::Scale;

#[derive(Resource)]
pub struct MonoSpacedFont {
    pub font_storage: [fdFont; 1],
}

impl MonoSpacedFont {
    pub fn new<Data: Deref<Target = [u8]>, T: Into<Scale>>(font_data: Data, opt_scale: T) -> Self {
        Self {
            font_storage: [fdFont::from_bytes(
                font_data,
                FontSettings {
                    scale: opt_scale.into().px(),
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
        [metrics.advance_width.ceil(), px.ceil()]
    }
}
impl Default for MonoSpacedFont {
    fn default() -> Self {
        MonoSpacedFont::new(
            include_bytes!("./JetBrainsMono-Medium.ttf").as_slice(),
            15u32,
        )
    }
}
