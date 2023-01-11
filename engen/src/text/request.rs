use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::text::scale::Scale;
use crate::text::{rasterization, GlyphHash};
#[derive(Clone)]
pub struct RequestData {
    pub character: char,
    pub scale: Scale,
    pub glyph_hash: GlyphHash,
    pub position: Position,
    pub area: Area,
    pub depth: Depth,
    pub color: Color,
    pub placement_descriptor: Option<rasterization::PlacementDescriptor>,
}
impl RequestData {
    pub fn new(
        character: char,
        scale: Scale,
        glyph_hash: GlyphHash,
        position: Position,
        area: Area,
        depth: Depth,
        color: Color,
    ) -> Self {
        Self {
            character,
            scale,
            glyph_hash,
            position,
            area,
            depth,
            color,
            placement_descriptor: None,
        }
    }
}
