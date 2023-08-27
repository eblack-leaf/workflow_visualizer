use std::collections::HashMap;
use std::ops::Deref;

use bevy_ecs::prelude::Resource;
use fontdue::{Font as fdFont, FontSettings};

use crate::coord::NumericalContext;
use crate::text::component::{TextScale, TextScaleAlignment};
use crate::Area;

#[derive(Resource)]
pub struct MonoSpacedFont {
    pub(crate) font_storage: [fdFont; 1],
}

impl MonoSpacedFont {
    pub fn jet_brains_mono<T: Into<TextScale>>(opt_scale: T) -> Self {
        Self::new(
            include_bytes!("JetBrainsMono-Regular.ttf").as_slice(),
            opt_scale,
        )
    }
    pub fn new<Data: Deref<Target = [u8]>, T: Into<TextScale>>(
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
    pub fn font_slice(&self) -> &[fdFont] {
        self.font_storage.as_slice()
    }
    pub fn font(&self) -> &fdFont {
        &self.font_storage[Self::index()]
    }
    pub(crate) fn index() -> usize {
        0
    }
    pub fn character_dimensions(&self, px: f32) -> Area<NumericalContext> {
        let metrics = self.font().metrics('a', px);
        let height = self
            .font()
            .horizontal_line_metrics(px)
            .expect("no metrics in font")
            .new_line_size;
        (metrics.advance_width.ceil(), height.ceil()).into()
    }
}

#[cfg(test)]
#[test]
fn tester() {
    for x in 0..96u32 {
        let font = MonoSpacedFont::jet_brains_mono(x);
        let x = x as f32;
        let dims = font.character_dimensions(x);
        println!("Scale: {:?} dims: {:?}", x, dims);
    }
}
