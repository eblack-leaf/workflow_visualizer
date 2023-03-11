use crate::text::scale::TextScale;

pub(crate) type GlyphId = fontdue::layout::GlyphRasterConfig;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub(crate) struct Glyph {
    pub(crate) character: char,
    pub(crate) scale: TextScale,
    pub(crate) id: GlyphId,
}

impl Glyph {
    pub(crate) fn new(character: char, scale: TextScale, id: GlyphId) -> Self {
        Self {
            character,
            scale,
            id,
        }
    }
}
