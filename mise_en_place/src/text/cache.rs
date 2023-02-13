use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::Component;

use crate::coord::{Depth, Logical, Position, Section, Unscaled};
use crate::text::glyph::{GlyphId, Key};
use crate::text::render_group::TextBound;
use crate::visibility::VisibleSection;
use crate::Color;

#[derive(Component)]
pub(crate) struct Cache {
    pub(crate) keys: HashSet<Key>,
    pub(crate) glyph_positions: HashMap<Key, Position<Logical>>,
    pub(crate) glyph_ids: HashMap<Key, GlyphId>,
    pub(crate) glyph_colors: HashMap<Key, Color>,
    pub(crate) bound: Option<TextBound>,
    pub(crate) position: Position<Unscaled>,
    pub(crate) depth: Depth,
    pub(crate) visible_section: VisibleSection,
}

impl Cache {
    pub(crate) fn new(
        position: Position<Unscaled>,
        depth: Depth,
        visible_section: VisibleSection,
    ) -> Self {
        Self {
            keys: HashSet::new(),
            glyph_positions: HashMap::new(),
            glyph_ids: HashMap::new(),
            glyph_colors: HashMap::new(),
            bound: None,
            position,
            depth,
            visible_section,
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
    pub(crate) fn add(&mut self, key: Key, glyph_id: GlyphId, glyph_position: Position<Logical>) {
        self.keys.insert(key);
        self.glyph_ids.insert(key, glyph_id);
        self.glyph_positions.insert(key, glyph_position);
    }
    pub(crate) fn glyph_position_different(
        &self,
        key: Key,
        glyph_position: Position<Logical>,
    ) -> bool {
        *self
            .glyph_positions
            .get(&key)
            .expect("no glyph position for key")
            != glyph_position
    }
    pub(crate) fn glyph_id_different(&self, key: Key, glyph_id: GlyphId) -> bool {
        *self.glyph_ids.get(&key).expect("no glyph id for key") != glyph_id
    }
    pub(crate) fn glyph_color_different(&self, key: Key, glyph_color: Color) -> bool {
        *self.glyph_colors.get(&key).expect("no glyph color for key") != glyph_color
    }
}
