use bytemuck::{Pod, Zeroable};

use crate::coord::NumericalContext;
use crate::text::atlas::AtlasTextureDimensions;
use crate::{Area, Position, Section};

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Default, Debug)]
pub(crate) struct Coords {
    pub(crate) data: [f32; 4],
}

impl Coords {
    pub(crate) fn from_section(
        glyph_section: Section<NumericalContext>,
        texture_dimensions: AtlasTextureDimensions,
    ) -> Self {
        let normalized_position = Position::<NumericalContext>::new(
            glyph_section.position.x / texture_dimensions.dimensions.width,
            glyph_section.position.y / texture_dimensions.dimensions.height,
        );
        let normalized_area = Area::<NumericalContext>::new(
            glyph_section.width() / texture_dimensions.dimensions.width,
            glyph_section.height() / texture_dimensions.dimensions.height,
        );
        let normalized_section = Section::new(normalized_position, normalized_area);
        Coords::new(
            normalized_section.left(),
            normalized_section.top(),
            normalized_section.right(),
            normalized_section.bottom(),
        )
    }
    pub(crate) fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            data: [left, top, right, bottom],
        }
    }
}
