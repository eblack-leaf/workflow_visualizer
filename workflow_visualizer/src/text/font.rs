use std::ops::{Deref, Sub};

use bevy_ecs::prelude::Resource;
use fontdue::{Font as fdFont, FontSettings};

use crate::coord::NumericalContext;
use crate::text::component::TextScale;
use crate::{Area, CoordinateUnit, InterfaceContext, Position, Section};

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
    pub fn text_section_descriptor(
        &self,
        position: Position<InterfaceContext>,
        known: TextSectionDescriptorKnown,
        characters: u32,
    ) -> TextSectionDescriptor {
        let scale = match known {
            TextSectionDescriptorKnown::Width(width) => {
                let px = width * Self::TEXT_GRID_THRESHOLD / characters as f32;
                self.text_scale_from_dimension(KnownTextDimension::Width(px as u32))
            }
            TextSectionDescriptorKnown::Height(markers) => {
                let px = markers.to_pixel();
                let px = px * Self::TEXT_GRID_THRESHOLD;
                self.text_scale_from_dimension(KnownTextDimension::Height(px as u32))
            }
            TextSectionDescriptorKnown::Scale(scale) => scale,
            TextSectionDescriptorKnown::WidthAndHeight(area) => {
                let px = area.width * Self::TEXT_GRID_THRESHOLD / characters as f32;
                let height_px = area.height * Self::TEXT_GRID_THRESHOLD;
                self.text_scale_from_dimension(KnownTextDimension::Width(px as u32))
                    .0
                    .min(
                        self.text_scale_from_dimension(KnownTextDimension::Height(
                            height_px as u32,
                        ))
                        .0,
                    )
                    .into()
            }
        };
        let letter_dims = self.character_dimensions(scale.px());
        let width = letter_dims.width * characters as f32;
        let height = letter_dims.height;
        TextSectionDescriptor::new(scale, Section::new(position, Area::new(width, height)))
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
pub struct TextSectionDescriptor {
    pub scale: TextScale,
    pub section: Section<InterfaceContext>,
}
impl TextSectionDescriptor {
    pub fn new<TS: Into<TextScale>, S: Into<Section<InterfaceContext>>>(
        scale: TS,
        section: S,
    ) -> Self {
        Self {
            scale: scale.into(),
            section: section.into(),
        }
    }
}
pub enum TextSectionDescriptorKnown {
    Width(CoordinateUnit),
    Height(CoordinateUnit),
    Scale(TextScale),
    WidthAndHeight(Area<InterfaceContext>),
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
