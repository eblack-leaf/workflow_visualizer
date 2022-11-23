use bevy_ecs::prelude::ResMut;
use std::collections::HashMap;

use fontdue::layout::GlyphPosition;
use fontdue::Metrics;

use crate::text_refactor::font::Font;
use crate::text_refactor::rasterization_descriptor::RasterizationDescriptor;
use crate::text_refactor::Rasterizations;

pub type RasterizedGlyphHash = fontdue::layout::GlyphRasterConfig;
pub type RasterizedGlyph = (Metrics, Vec<u8>);

pub struct Rasterizer {
    pub rasterized_glyphs: HashMap<RasterizedGlyphHash, (RasterizedGlyph, RasterizationDescriptor)>,
    pub appends: Vec<u8>,
}

impl Rasterizer {
    pub fn new() -> Self {
        Self {
            rasterized_glyphs: HashMap::new(),
            appends: Vec::new(),
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
        let start: u32 = (self.appends.len() - 1) as u32;
        let row_size: u32 = positioned_glyph.width as u32;
        let rows: u32 = (glyph.1.len() / row_size as usize) as u32;
        let rasterization_descriptor = RasterizationDescriptor::new(start, row_size, rows);
        self.rasterized_glyphs.insert(
            positioned_glyph.key,
            (glyph.clone(), rasterization_descriptor),
        );
        self.appends.extend(glyph.1);
        return rasterization_descriptor;
    }
}
pub struct RasterizationReferences {
    pub refs: HashMap<RasterizedGlyphHash, u32>,
    pub orphaned: Vec<RasterizedGlyphHash>, // no need just check 0 or no entry made
}
impl RasterizationReferences {
    pub fn new() -> Self {
        Self {
            refs: HashMap::new(),
            orphaned: Vec::new(),
        }
    }
    pub fn remove(&mut self, rasterization: RasterizedGlyphHash) {
        if let Some(mut ref_count) = self.refs.get_mut(&rasterization) {
            if ref_count == 0 {
                return;
            }
            ref_count -= 1;
            if ref_count == 0 {
                self.orphaned.push(rasterization);
            }
        }
    }
}
pub fn integrate_appends(
    mut rasterizer: ResMut<Rasterizer>,
    mut rasterization: ResMut<Rasterizations>,
    mut rasterization_references: ResMut<RasterizationReferences>,
) {
    // shrink if can and add appended
    // could just rewrite and update descriptors
}
