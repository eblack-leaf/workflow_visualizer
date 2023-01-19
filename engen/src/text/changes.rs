use crate::canvas::Visibility;
use crate::text::component::Key;
use crate::text::instance::Attributes;
use crate::text::rasterization::{Alignment, Glyph, GlyphHash};
use crate::{Area, Section};
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Resource;
use std::collections::{HashMap, HashSet};

#[derive(Resource)]
pub(crate) struct Changes {
    pub added_text_entities: HashMap<Entity, (u32, Alignment)>,
    pub removed_text_entities: HashSet<Entity>,
    pub adds: HashMap<Entity, HashMap<Key, (Area, Attributes)>>,
    pub updates: HashMap<Entity, HashMap<Key, Attributes>>,
    pub removes: HashMap<Entity, HashSet<Key>>,
    pub glyphs: HashMap<Entity, HashSet<(Key, Glyph)>>,
    pub bounds: HashMap<Entity, Section>,
    pub removed_bounds: HashSet<Entity>,
    pub removed_glyphs: HashMap<Entity, HashSet<(Key, GlyphHash)>>,
    pub visibility: HashMap<Entity, Visibility>,
}

impl Changes {
    pub(crate) fn reset(&mut self) {
        self.added_text_entities.clear();
        self.removed_text_entities.clear();
        self.adds.clear();
        self.updates.clear();
        self.removes.clear();
        self.glyphs.clear();
        self.bounds.clear();
        self.removed_bounds.clear();
        self.removed_glyphs.clear();
        self.visibility.clear();
    }
    pub(crate) fn new() -> Self {
        Self {
            added_text_entities: HashMap::new(),
            removed_text_entities: HashSet::new(),
            adds: HashMap::new(),
            updates: HashMap::new(),
            removes: HashMap::new(),
            glyphs: HashMap::new(),
            bounds: HashMap::new(),
            removed_bounds: HashSet::new(),
            removed_glyphs: HashMap::new(),
            visibility: HashMap::new(),
        }
    }
}
