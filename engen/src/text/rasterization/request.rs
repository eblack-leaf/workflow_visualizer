use crate::text::rasterization::glyph::{GlyphReference, RasterizedGlyph};
use crate::text::rasterization::RasterizationHandler;
use crate::text::scale::Scale;
use crate::text::GlyphHash;

#[derive(Clone)]
pub(crate) struct Request {
    pub(crate) glyph_hash: GlyphHash,
    pub(crate) character: char,
    pub(crate) scale: Scale,
}
impl Request {
    pub(crate) fn new(glyph_hash: GlyphHash, character: char, scale: Scale) -> Self {
        Self {
            glyph_hash,
            character,
            scale,
        }
    }
}
pub(crate) fn request(rasterization: &mut RasterizationHandler) {
    for request in rasterization.requests.iter() {
        if !rasterization
            .cached_rasterized_glyphs
            .contains_key(&request.glyph_hash)
        {
            rasterization
                .references
                .insert(request.glyph_hash, GlyphReference::new());
            let rasterized_glyph: RasterizedGlyph = rasterization
                .font
                .font()
                .rasterize(request.character, request.scale.px())
                .into();
            rasterization
                .cached_rasterized_glyphs
                .insert(request.glyph_hash, rasterized_glyph);
            rasterization.write_requests.insert(request.glyph_hash);
        } else {
            rasterization
                .references
                .get_mut(&request.glyph_hash)
                .expect("no ref count setup for key")
                .increment();
        }
    }
}
