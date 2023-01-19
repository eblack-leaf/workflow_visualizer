use crate::text::component::Key;
use crate::text::instance::Attributes;
use crate::text::rasterization::GlyphHash;
use crate::Section;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Resource;
use std::collections::{HashMap, HashSet};

#[derive(Resource)]
pub(crate) struct Cache {
    pub glyphs: HashMap<Key, GlyphHash>,
    pub attributes: HashMap<Key, Attributes>,
    pub bounds: HashMap<Entity, Section>,
    pub visible_entities: HashSet<Entity>,
}

impl Cache {
    pub(crate) fn new() -> Self {
        Self {
            glyphs: HashMap::new(),
            attributes: HashMap::new(),
            bounds: HashMap::new(),
            visible_entities: HashSet::new(),
        }
    }
}
