use crate::{Despawn, Disabled};
use bevy_ecs::prelude::Entity;
use bevy_ecs::system::Commands;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Scene<Key: Copy + Clone + Hash + Eq + PartialEq> {
    pub entities: Vec<Entity>,
    pub associations: HashMap<Key, Entity>,
}
impl<Key: Copy + Clone + Hash + Eq + PartialEq> Scene<Key> {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            associations: HashMap::new(),
        }
    }
    pub fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
    pub fn add_associated(&mut self, key: Key, entity: Entity) {
        self.associations.insert(key, entity);
    }
    pub fn enable_all(&self, cmd: &mut Commands) {
        for entity in self.entities.iter() {
            cmd.entity(*entity).insert(Disabled::default());
        }
        for (_, entity) in self.associations.iter() {
            cmd.entity(*entity).insert(Disabled::default());
        }
    }
    pub fn disable_all(&self, cmd: &mut Commands) {
        for entity in self.entities.iter() {
            cmd.entity(*entity).remove::<Disabled>();
        }
        for (_, entity) in self.associations.iter() {
            cmd.entity(*entity).remove::<Disabled>();
        }
    }
    pub fn checked_despawn(&mut self, cmd: &mut Commands) {
        for entity in self.entities.drain(..) {
            cmd.entity(entity).insert(Despawn::default());
        }
        for (_, entity) in self.associations.drain() {
            cmd.entity(entity).insert(Despawn::default());
        }
    }
}
