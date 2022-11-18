use std::collections::HashMap;

use fontdue::layout::GlyphPosition;
use fontdue::Metrics;

use crate::text_refactor::font::Font;
use crate::text_refactor::rasterization_descriptor::RasterizationDescriptor;

pub type RasterizedGlyphHash = fontdue::layout::GlyphRasterConfig;
pub type RasterizedGlyph = (Metrics, Vec<u8>);

pub struct Rasterizer {
    pub rasterized_glyphs: HashMap<RasterizedGlyphHash, (RasterizedGlyph, RasterizationDescriptor)>,
    pub buffer: Vec<u8>,
}

impl Rasterizer {
    pub fn new() -> Self {
        Self {
            rasterized_glyphs: HashMap::new(),
            buffer: Vec::new(),
        }
    }
    pub fn rasterize(
        &mut self,
        font: Font,
        positioned_glyph: GlyphPosition,
    ) -> RasterizationDescriptor {
        if let Some(rasterization) = self.rasterized_glyphs.get(&positioned_glyph.key) {
            return rasterization.1;
        }
        let glyph = font
            .font()
            .rasterize(positioned_glyph.parent, positioned_glyph.key.px);
        let start: u32 = (self.buffer.len() - 1) as u32;
        let row_size: u32 = positioned_glyph.width as u32;
        let rows: u32 = (glyph.1.len() / row_size as usize) as u32;
        let rasterization = RasterizationDescriptor::new(start, row_size, rows);
        self.rasterized_glyphs
            .insert(positioned_glyph.key, (glyph.clone(), rasterization));
        self.buffer.extend(glyph.1);
        return rasterization;
    }
}
