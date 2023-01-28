use std::ops::Deref;

use bevy_ecs::prelude::Resource;
use fontdue::{Font as fdFont, FontSettings};
use crate::coord::Area;

use crate::text::scale::TextScale;

#[derive(Resource)]
pub(crate) struct MonoSpacedFont {
    pub(crate) font_storage: [fdFont; 1],
}

impl MonoSpacedFont {
    pub(crate) fn jet_brains_mono<T: Into<TextScale>>(opt_scale: T) -> Self {
        Self::new(
            include_bytes!("./JetBrainsMono-Medium.ttf").as_slice(),
            opt_scale,
        )
    }
    pub(crate) fn new<Data: Deref<Target = [u8]>, T: Into<TextScale>>(
        font_data: Data,
        opt_scale: T,
    ) -> Self {
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
    pub(crate) fn font_slice(&self) -> &[fdFont] {
        self.font_storage.as_slice()
    }
    pub(crate) fn font(&self) -> &fdFont {
        &self.font_storage[0]
    }
    pub(crate) fn index() -> usize {
        0
    }
    pub(crate) fn character_dimensions(&self, character: char, px: f32) -> Area {
        let metrics = self.font().metrics(character, px);
        (metrics.advance_width.ceil(), px.ceil()).into()
    }
}
