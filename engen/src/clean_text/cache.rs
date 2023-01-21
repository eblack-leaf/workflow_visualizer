use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Changed, Component, Entity, Query};

use crate::clean_text::difference::Difference;
use crate::clean_text::glyph::{Glyph, GlyphId, Key};
use crate::clean_text::place::Placer;
use crate::clean_text::scale::Scale;
use crate::{Area, Color, Depth, Position, Section};

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

impl Cache {
    pub(crate) fn new(position: Position, depth: Depth, color: Color) -> Self {
        Self {
            keys: HashSet::new(),
            glyph_positions: HashMap::new(),
            glyph_ids: HashMap::new(),
            bound: None,
            position,
            depth,
            color,
        }
    }
    pub(crate) fn exists(&self, key: Key) -> bool {
        self.keys.contains(&key)
    }
    pub(crate) fn remove(&mut self, key: Key) {
        self.keys.remove(&key);
        self.glyph_ids.remove(&key);
        self.glyph_positions.remove(&key);
    }
    pub(crate) fn add(&mut self, key: Key, glyph_id: GlyphId, glyph_position: Position) {
        self.keys.insert(key);
        self.glyph_ids.insert(key, glyph_id);
        self.glyph_positions.insert(key, glyph_position);
    }
    pub(crate) fn glyph_position_different(&self, key: Key, glyph_position: Position) -> bool {
        *self
            .glyph_positions
            .get(&key)
            .expect("no glyph position for key")
            == glyph_position
    }
    pub(crate) fn glyph_id_different(&self, key: Key, glyph_id: GlyphId) -> bool {
        *self.glyph_ids.get(&key).expect("no glyph id for key") == glyph_id
    }
}

pub(crate) fn letter_diff(
    mut text: Query<(&Scale, &mut Placer, &mut Cache, &mut Difference), Changed<Placer>>,
) {
    for (scale, mut placer, mut cache, mut difference) in text.iter_mut() {
        let mut retained_keys = HashSet::new();
        let old_keys = cache.keys.clone();
        for placed_glyph in placer.placement.iter() {
            let key = Key::new(placed_glyph.byte_offset as u32);
            let glyph_position = (placed_glyph.x, placed_glyph.y).into();
            let glyph_id = placed_glyph.key;
            let character = placed_glyph.parent;
            let glyph = Glyph::new(character, *scale, glyph_id);
            if cache.exists(key) {
                retained_keys.insert(key);
                if cache.glyph_position_different(key, glyph_position) {
                    difference.update.insert(key, glyph_position);
                    cache.glyph_positions.insert(key, glyph_position);
                }
                if cache.glyph_id_different(key, glyph_id) {
                    difference.glyph_add.insert(key, glyph);
                    cache.glyph_ids.insert(key, glyph_id);
                }
            } else {
                difference.add.insert(key, glyph_position);
                difference.glyph_add.insert(key, glyph);
                cache.add(key, glyph_id, glyph_position);
            }
        }
        let keys_to_remove = old_keys
            .difference(&retained_keys)
            .copied()
            .collect::<HashSet<Key>>();
        for key in keys_to_remove {
            cache.remove(key);
            difference.remove.insert(key);
        }
    }
}

pub(crate) fn bounds_diff(
    mut text: Query<(&Position, Option<&Area>, &mut Cache, &mut Difference), Changed<Area>>,
) {
    for (position, maybe_area, mut cache, mut difference) in text.iter_mut() {
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
