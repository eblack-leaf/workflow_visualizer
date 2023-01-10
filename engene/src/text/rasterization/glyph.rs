use fontdue::Metrics;
pub(crate) struct GlyphReference {
    pub(crate) count: u32,
}
impl GlyphReference {
    pub(crate) fn new() -> Self {
        Self { count: 0 }
    }
    pub(crate) fn increment(&mut self) {
        self.count += 1;
    }
    pub(crate) fn decrement(&mut self) {
        self.count -= 1 * (self.count != 0) as u32;
    }
}
pub(crate) type GlyphHash = fontdue::layout::GlyphRasterConfig;
#[derive(Clone)]
pub(crate) struct RasterizedGlyph {
    pub(crate) metrics: Metrics,
    pub(crate) bitmap: Vec<u32>,
}
impl From<(Metrics, Vec<u8>)> for RasterizedGlyph {
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
