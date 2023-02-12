use crate::text::scale::TextScale;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Debug)]
pub(crate) struct Key {
    offset: u32,
}

impl Key {
    fn new(offset: u32) -> Self {
        Self { offset }
    }
    pub(crate) fn offset(&self) -> u32 {
        self.offset
    }
}
pub(crate) struct KeyFactory {
    current: u32,
}
impl KeyFactory {
    pub(crate) fn new() -> Self {
        Self {
            current: 0,
        }
    }
    pub(crate) fn generate(&mut self) -> Key {
        let key = Key::new(self.current);
        self.current += 1;
        key
    }
}
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
