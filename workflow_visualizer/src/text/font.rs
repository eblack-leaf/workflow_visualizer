use std::ops::{Deref, Sub};

use bevy_ecs::prelude::Resource;
use fontdue::{Font as fdFont, FontSettings};

use crate::coord::NumericalContext;
use crate::text::component::TextScale;
use crate::{Area, GridPoint, GridView, GridViewBuilder, RawMarker};

#[derive(Resource)]
pub struct MonoSpacedFont {
    pub(crate) font_storage: [fdFont; 1],
}

impl MonoSpacedFont {
    pub const DEFAULT_OPT_SCALE: u32 = 80u32;
    const TEXT_GRID_THRESHOLD: f32 = 0.95f32;

    pub const FACTOR_BASE_SCALE: u32 = 40u32;
    pub const MAX_CHECKED_TEXT_SCALE: u32 = 400u32;
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
    pub fn text_scale_from_dimension(&self, dimension: KnownTextDimension) -> TextScale {
        match dimension {
            KnownTextDimension::Width(width) => {
                for scale in 0..Self::MAX_CHECKED_TEXT_SCALE {
                    let dimensions = self.character_dimensions(scale as f32);
                    if dimensions.width as u32 > width {
                        return TextScale(scale.sub(1).max(0));
                    } else if dimensions.width as u32 == width {
                        return TextScale(scale);
                    }
                }
            }
            KnownTextDimension::Height(height) => {
                for scale in 0..Self::MAX_CHECKED_TEXT_SCALE {
                    let dimensions = self.character_dimensions(scale as f32);
                    if dimensions.height as u32 > height {
                        return TextScale(scale.sub(1).max(0));
                    } else if dimensions.height as u32 == height {
                        return TextScale(scale);
                    }
                }
            }
        }
        TextScale(0)
    }
    pub fn text_grid_view(
        &self,
        position: GridPoint,
        known: TextGridViewKnown,
        characters: u32,
    ) -> TextGridView {
        let scale = match known {
            TextGridViewKnown::Width(markers) => {
                let px = markers.to_pixel();
                let px = px * Self::TEXT_GRID_THRESHOLD / characters as f32;
                self.text_scale_from_dimension(KnownTextDimension::Width(px as u32))
            }
            TextGridViewKnown::Height(markers) => {
                let px = markers.to_pixel();
                let px = px * Self::TEXT_GRID_THRESHOLD;
                self.text_scale_from_dimension(KnownTextDimension::Height(px as u32))
            }
            TextGridViewKnown::Scale(scale) => scale,
        };
        let letter_dims = self.character_dimensions(scale.px());
        let width = RawMarker::from_pixel_inclusive(letter_dims.width * characters as f32);
        let height = RawMarker::from_pixel_inclusive(letter_dims.height);
        TextGridView::new(
            scale,
            GridViewBuilder::new()
                .with_left(position.x)
                .with_top(position.y)
                .with_right(position.x.raw_offset(width))
                .with_bottom(position.y.raw_offset(height))
                .build(),
        )
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
pub struct TextGridView {
    pub scale: TextScale,
    pub view: GridView,
}
impl TextGridView {
    pub fn new<TS: Into<TextScale>, GV: Into<GridView>>(scale: TS, view: GV) -> Self {
        Self {
            scale: scale.into(),
            view: view.into(),
        }
    }
}
pub enum TextGridViewKnown {
    Width(RawMarker),
    Height(RawMarker),
    Scale(TextScale),
}
pub enum KnownTextDimension {
    Width(u32),
    Height(u32),
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
