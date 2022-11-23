use crate::text_step_out::rasterization::rasterizations::RasterizedGlyphHash;
use std::collections::HashMap;

pub struct RasterizationReferences {
    pub references: HashMap<RasterizedGlyphHash, u32>,
}
impl RasterizationReferences {
    pub fn new() -> Self {
        Self {
            references: HashMap::new(),
        }
    }
}
