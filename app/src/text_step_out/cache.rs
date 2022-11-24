use crate::text_step_out::attributes::Index;
use crate::text_step_out::scale::Scale;
use crate::text_step_out::RasterizationPlacement;
use std::collections::HashMap;
pub struct TextOffset(pub u32);
pub struct Glyph {
    pub character: char,
    pub scale: Scale,
}
pub struct Glyphs {
    pub glyphs: Vec<Glyph>,
}
// want more than Vec to handle inter
pub struct GlyphIndices {
    pub indices: HashMap<TextOffset, Index>,
}
pub struct GlyphRasterizationPlacements {
    pub placements: HashMap<TextOffset, RasterizationPlacement>,
}
// if cache is out for the offset, then make glyph write to write<rast. desc.>
pub struct GlyphWrite {}
pub struct Cache {}
