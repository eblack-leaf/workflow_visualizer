use crate::text_refactor::instance::Instance;
use crate::text_refactor::instances::Index;
use crate::text_refactor::rasterizer::RasterizedGlyphHash;
use bevy_ecs::prelude::Component;

pub struct Glyph {
    pub instance: Instance,
    pub rasterized_glyph: RasterizedGlyphHash,
    pub index: Option<Index>,
}
#[derive(Component)]
pub struct Glyphs {
    pub glyphs: Vec<Glyph>,
}

impl Glyphs {
    pub fn new() -> Self {
        Self { glyphs: Vec::new() }
    }
}
