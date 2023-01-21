use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Added, Changed, Component, Entity, Or, Query};

use crate::{Area, Color, Depth, Position, Section};
use crate::clean_text::extraction::Difference;
use crate::clean_text::glyph::{GlyphId, Key};
use crate::clean_text::place::Placer;

#[derive(Component)]
pub(crate) struct Cache {
    pub(crate) keys: HashSet<Key>,
    pub(crate) glyph_positions: HashMap<Key, Position>,
    pub(crate) glyph_ids: HashMap<Key, GlyphId>,
    pub(crate) bound: Option<Section>,
    pub(crate) position: Position,
    pub(crate) depth: Depth,
    pub(crate) color: Color,
}

pub(crate) fn letter_diff(text: Query<(Entity, &mut Placer, &mut Cache, &mut Difference), (Changed<Placer>)>) {
    // check cache for all placed glyphs not in out of bounds
    // -- check glyph ids
    // -- check glyph positions
    // -- retained_keys / removed_keys / added_keys
}

pub(crate) fn bounds_diff(mut text: Query<(Entity, &Position, Option<&Area>, &mut Cache, &mut Difference), (Changed<Area>)>) {
    for (entity, position, maybe_area, mut cache, mut difference) in text.iter_mut() {
        if let Some(area) = maybe_area {
            let section = Section::new(*position, *area);
            difference.bound.replace(section);
            cache.bound.replace(section);
        } else if cache.bound.is_some() {
            difference.remove_bound = true;
            cache.bound.take();
        }
    }
}
