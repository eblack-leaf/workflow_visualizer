use crate::text::scale::Scale;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) struct Key {
    pub(crate) offset: u32,
}

impl Key {
    pub(crate) fn new(offset: u32) -> Self {
        Self { offset }
    }
}

pub(crate) type GlyphId = fontdue::layout::GlyphRasterConfig;

#[derive(Clone)]
pub(crate) struct Glyph {
    pub(crate) character: char,
    pub(crate) scale: Scale,
    pub(crate) id: GlyphId,
}

impl Glyph {
    pub(crate) fn new(character: char, scale: Scale, id: GlyphId) -> Self {
        Self {
            character,
            scale,
            id,
        }
    }
}
