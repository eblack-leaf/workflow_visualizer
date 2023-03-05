use std::collections::HashMap;

use bevy_ecs::component::Component;
use bevy_ecs::prelude::Resource;

use crate::{Area, Numerical};
use crate::text::font::MonoSpacedFont;

#[derive(Component, Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub(crate) struct TextScale {
    pub(crate) scale: u32,
}

#[derive(Component)]
pub struct TextScaleLetterDimensions {
    pub(crate) dimensions: Area<Numerical>,
}

impl TextScaleLetterDimensions {
    pub(crate) fn new<A: Into<Area<Numerical>>>(area: A) -> Self {
        Self {
            dimensions: area.into(),
        }
    }
}

impl TextScale {
    pub(crate) fn new(scale: u32) -> Self {
        Self { scale }
    }
    pub(crate) fn px(&self) -> f32 {
        self.scale as f32
    }
    pub(crate) fn from_alignment(alignment: TextScaleAlignment, scale_factor: f64) -> Self {
        match alignment {
            TextScaleAlignment::Small => {
                Self::new((TEXT_SCALE_ALIGNMENT_GUIDE[0] as f64 * scale_factor) as u32)
            }
            TextScaleAlignment::Medium => {
                Self::new((TEXT_SCALE_ALIGNMENT_GUIDE[1] as f64 * scale_factor) as u32)
            }
            TextScaleAlignment::Large => {
                Self::new((TEXT_SCALE_ALIGNMENT_GUIDE[2] as f64 * scale_factor) as u32)
            }
        }
    }
}

impl From<f32> for TextScale {
    fn from(scale: f32) -> Self {
        Self {
            scale: scale as u32,
        }
    }
}

impl From<u32> for TextScale {
    fn from(scale: u32) -> Self {
        Self { scale }
    }
}

#[derive(Component, Copy, Clone, Eq, Hash, PartialEq)]
pub enum TextScaleAlignment {
    Small,
    Medium,
    Large,
}

const TEXT_SCALE_ALIGNMENT_GUIDE: [u32; 3] = [15, 18, 22];

#[derive(Resource)]
pub(crate) struct AlignedFonts {
    pub(crate) fonts: HashMap<TextScaleAlignment, MonoSpacedFont>,
}

impl AlignedFonts {
    pub(crate) fn new(scale_factor: f64) -> Self {
        Self {
            fonts: {
                let mut fonts = HashMap::new();
                fonts.insert(
                    TextScaleAlignment::Small,
                    MonoSpacedFont::jet_brains_mono(TextScale::from_alignment(
                        TextScaleAlignment::Small,
                        scale_factor,
                    )),
                );
                fonts.insert(
                    TextScaleAlignment::Medium,
                    MonoSpacedFont::jet_brains_mono(TextScale::from_alignment(
                        TextScaleAlignment::Medium,
                        scale_factor,
                    )),
                );
                fonts.insert(
                    TextScaleAlignment::Large,
                    MonoSpacedFont::jet_brains_mono(TextScale::from_alignment(
                        TextScaleAlignment::Large,
                        scale_factor,
                    )),
                );
                fonts
            },
        }
    }
}
