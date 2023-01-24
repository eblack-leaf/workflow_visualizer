use std::collections::{HashMap, HashSet};

use bevy_ecs::component::Component;

use crate::text::glyph::{Glyph, GlyphId, Key};
use crate::{Color, Depth, Position, Section};

#[derive(Component, Clone)]
pub(crate) struct Difference {
    pub(crate) bounds: Option<Section>,
    pub(crate) position: Option<Position>,
    pub(crate) depth: Option<Depth>,
    pub(crate) color: Option<Color>,
    pub(crate) glyph_add: HashMap<Key, Glyph>,
    pub(crate) glyph_remove: HashSet<GlyphId>,
    pub(crate) add: HashMap<Key, Position>,
    pub(crate) update: HashMap<Key, Position>,
    pub(crate) remove: HashSet<Key>,
}

impl Difference {
    pub(crate) fn new() -> Self {
        Self {
            bounds: None,
            position: None,
            depth: None,
            color: None,
            glyph_add: HashMap::new(),
            glyph_remove: HashSet::new(),
            add: HashMap::new(),
            update: HashMap::new(),
            remove: HashSet::new(),
        }
    }
    pub(crate) fn reset(&mut self) {
        *self = Self::new();
    }
}
