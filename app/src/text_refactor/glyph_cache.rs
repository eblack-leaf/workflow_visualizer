use std::collections::HashMap;

use bevy_ecs::prelude::{Entity, Query, ResMut};

use crate::text_refactor::glyphs::{Glyph, Glyphs};
use crate::text_refactor::instances::Index;
use crate::text_refactor::rasterizer::RasterizedGlyphHash;

pub struct GlyphCache {
    pub cache: HashMap<Entity, Glyphs>,
}

impl GlyphCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
}

pub struct RemovedRasterizations {
    pub removed: Vec<RasterizedGlyphHash>,
}

pub fn compare(
    glyphs: Query<(Entity, &Glyphs)>,
    mut glyph_cache: ResMut<GlyphCache>,
    mut removed_rasterizations: ResMut<RemovedRasterizations>,
) {
    glyphs
        .iter()
        .for_each(|(entity, glyphs): (Entity, &Glyphs)| {
            if let Some(cached_glyphs) = glyph_cache.cache.get(&entity) {
                let cached_glyph_length = cached_glyphs.glyphs.len();
                for index in 0..glyphs.glyphs.len() - 1 {
                    let current_glyph = glyphs.glyphs.get(index).unwrap();
                    if let Some(cached_glyph) = cached_glyphs.glyphs.get(index) {
                        if current_glyph.instance != cached_glyph.instance {}
                        if current_glyph.rasterized_glyph != cached_glyph.rasterized_glyph {
                            removed_rasterizations
                                .removed
                                .push(cached_glyph.rasterized_glyph);
                            // write instance rasterization_descriptor
                        }
                    }
                }
            } else {
                // add all glyphs to cache and send all as update
            }
        });
}
