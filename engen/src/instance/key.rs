use crate::instance::indexer::Index;
use bevy_ecs::entity::Entity;
use std::hash::Hash;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct EntityKey<Identifier: Eq + Hash + PartialEq + Copy + Clone> {
    pub(crate) entity: Entity,
    pub(crate) identifier: Identifier,
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
