use crate::clean_text::glyph::{Glyph, GlyphId, Key};
use crate::{Color, Depth, Position, Section};
use bevy_ecs::component::Component;
use std::collections::{HashMap, HashSet};

#[derive(Component)]
pub(crate) struct Difference {
    pub(crate) bound: Option<Section>,
    pub(crate) remove_bound: bool,
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
            bound: None,
            remove_bound: false,
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
}
