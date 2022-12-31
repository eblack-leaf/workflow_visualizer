use crate::text::rasterization::descriptor::DescriptorRequest;
use crate::text::rasterization::Rasterization;
use crate::text::scale::Scale;
use fontdue::Metrics;
#[derive(Clone)]
pub(crate) struct Add {
    pub(crate) hash: GlyphHash,
    pub(crate) character: char,
    pub(crate) scale: Scale,
}
impl Add {
    pub(crate) fn new(hash: GlyphHash, character: char, scale: Scale) -> Self {
        Self {
            hash,
            character,
            scale,
        }
    }
}
pub(crate) type GlyphHash = fontdue::layout::GlyphRasterConfig;
#[derive(Clone)]
pub(crate) struct Glyph {
    pub(crate) metrics: Metrics,
    pub(crate) bitmap: Vec<u32>,
}
impl From<(Metrics, Vec<u8>)> for Glyph {
    fn from(rasterize_response: (Metrics, Vec<u8>)) -> Self {
        Self {
            metrics: rasterize_response.0,
            bitmap: rasterize_response
                .1
                .iter()
                .map(|c| *c as u32)
                .collect::<Vec<u32>>(),
        }
    }
}
pub(crate) fn rasterize(rasterization: &mut Rasterization) {
    for add in rasterization.adds.iter() {
        let glyph = match rasterization.glyphs.get(&add.hash) {
            None => rasterization
                .font
                .font()
                .rasterize(add.character, add.scale.px())
                .into(),
            Some(cached_glyph) => cached_glyph.clone(),
        };
        rasterization
            .descriptor_requests
            .push(DescriptorRequest::new(add.hash, glyph));
    }
    rasterization.adds.clear();
}
