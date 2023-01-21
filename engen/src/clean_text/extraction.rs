use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Changed, Component, Entity, Query, ResMut, Resource};

use crate::clean_text::glyph::{Glyph, GlyphId, Key};
use crate::{Color, Depth, Position, Section, Visibility};

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

#[derive(Resource)]
pub(crate) struct Extraction {
    pub(crate) added_render_groups: HashMap<Entity, ()>,
    pub(crate) removed_render_groups: HashSet<Entity>,
}

impl Extraction {
    pub(crate) fn new() -> Self {
        Self {
            added_render_groups: HashMap::new(),
            removed_render_groups: HashSet::new(),
        }
    }
}

pub(crate) fn pull_differences(
    mut extraction: ResMut<Extraction>,
    differences: Query<(Entity, &mut Difference, &Visibility), Changed<Difference>>,
) {
    // drain from diffs into extraction if visible
}
