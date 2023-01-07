use anymap::AnyMap;
use bevy_ecs::prelude::Entity;
use std::hash::Hash;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct Index(pub(crate) usize);
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
pub struct Coordinator<Key: Eq + Hash + PartialEq + Copy + Clone, Request> {
    cpu_buffers: AnyMap,
    gpu_buffers: AnyMap,
}
struct Placer {
    current: usize,
    max: usize,
    empty_slots: Vec<usize>,
}
