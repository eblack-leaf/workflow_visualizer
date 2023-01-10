use crate::text::rasterization::glyph::{GlyphReference, RasterizedGlyph};
use crate::text::rasterization::RasterizationHandler;
use crate::text::scale::Scale;
use crate::text::GlyphHash;

#[derive(Clone)]
pub(crate) struct Request {
    pub(crate) glyph: GlyphHash,
    pub(crate) character: char,
    pub(crate) scale: Scale,
}
impl Request {
    pub(crate) fn new(hash: GlyphHash, character: char, scale: Scale) -> Self {
        Self {
            glyph: hash,
            character,
            scale,
        }
    }
}
pub(crate) fn rasterize(rasterization: &mut RasterizationHandler) {
    for request in rasterization.requests.iter() {
        if !rasterization.cached_glyphs.contains_key(&request.glyph) {
            rasterization
                .references
                .insert(request.glyph, GlyphReference::new());
            let rasterized_glyph: RasterizedGlyph = rasterization
                .font
                .font()
                .rasterize(request.character, request.scale.px())
                .into();
            rasterization
                .cached_glyphs
                .insert(request.glyph, rasterized_glyph);
            rasterization.write_requests.insert(request.glyph);
        } else {
            rasterization
                .references
                .get_mut(&request.glyph)
                .expect("no ref count setup for key")
                .increment();
        }
    }
}
