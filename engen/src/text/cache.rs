use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Changed, Component, Entity, Query};

use crate::{Area, Color, Depth, Position, Section};
use crate::text::difference::Difference;
use crate::text::glyph::{Glyph, GlyphId, Key};
use crate::text::place::Placer;
use crate::text::scale::Scale;

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
    pub(crate) fn get_glyph_id(&self, key: Key) -> GlyphId {
        *self.glyph_ids.get(&key).expect("no glyph id")
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
