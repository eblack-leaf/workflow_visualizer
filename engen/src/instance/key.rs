use crate::instance::indexer::Index;
use bevy_ecs::entity::Entity;
use std::hash::Hash;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct EntityKey<Identifier: Eq + Hash + PartialEq + Copy + Clone> {
    pub entity: Entity,
    pub identifier: Identifier,
}
impl<Identifier: Eq + Hash + PartialEq + Copy + Clone> EntityKey<Identifier> {
    pub fn new(entity: Entity, identifier: Identifier) -> Self {
        Self { entity, identifier }
    }
}
pub(crate) struct IndexedKey<Key: Eq + Hash + PartialEq + Copy + Clone> {
    pub(crate) key: Key,
    pub(crate) index: Index,
}

impl<Key: Eq + Hash + PartialEq + Copy + Clone> IndexedKey<Key> {
    pub(crate) fn new(key: Key, index: Index) -> Self {
        Self { key, index }
    }
}
