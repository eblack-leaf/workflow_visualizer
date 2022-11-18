use crate::text_refactor::instance::Instance;
use crate::text_refactor::rasterizer::RasterizedGlyphHash;

pub struct Glyph {
    pub instance: Instance,
    pub rasterized_glyph: RasterizedGlyphHash,
}

pub struct Glyphs {
    pub glyphs: Vec<Glyph>,
}

impl Glyphs {
    pub fn new() -> Self {
        Self { glyphs: Vec::new() }
    }
}
