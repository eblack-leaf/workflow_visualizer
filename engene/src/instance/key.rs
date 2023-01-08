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
