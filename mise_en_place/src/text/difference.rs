use std::collections::{HashMap, HashSet};

use bevy_ecs::component::Component;

use crate::{Area, Color};
use crate::coord::{Depth, Logical, Position, Section, Unscaled};
use crate::text::glyph::{Glyph, GlyphId, Key};
use crate::text::render_group::TextBound;
use crate::visibility::VisibleSection;

#[derive(Copy, Clone)]
pub(crate) enum TextBoundDifference {
    Changed(TextBound),
    Removed,
}

#[derive(Component, Clone)]
pub(crate) struct Difference {
    pub(crate) bounds: Option<TextBoundDifference>,
    pub(crate) position: Option<Position<Unscaled>>,
    pub(crate) visible_section: Option<VisibleSection>,
    pub(crate) depth: Option<Depth>,
    pub(crate) glyph_add: HashMap<Key, Glyph>,
    pub(crate) glyph_remove: HashSet<GlyphId>,
    pub(crate) add: HashMap<Key, Position<Logical>>,
    pub(crate) update: HashMap<Key, Position<Logical>>,
    pub(crate) remove: HashSet<Key>,
    pub(crate) glyph_color_change: HashMap<Key, Color>,
}

impl Difference {
    pub(crate) fn new() -> Self {
        Self {
            bounds: None,
            position: None,
            visible_section: None,
            depth: None,
            glyph_add: HashMap::new(),
            glyph_remove: HashSet::new(),
            add: HashMap::new(),
            update: HashMap::new(),
            remove: HashSet::new(),
            glyph_color_change: HashMap::new(),
        }
    }
    pub(crate) fn reset(&mut self) {
        *self = Self::new();
    }
}
