use crate::text::attribute::Coordinator;
use bevy_ecs::prelude::{Commands, Component, Entity, Query};

pub(crate) struct Indexer {
    pub(crate) current: u32,
    pub(crate) max: u32,
}
impl Indexer {
    pub(crate) fn new(max: u32) -> Self {
        Self { current: 0, max }
    }
    pub(crate) fn next(&mut self) -> Instance {
        self.current += 1;
        Instance(self.current)
    }
    pub(crate) fn decrement(&mut self) {
        self.current -= 1;
    }
}
#[derive(Component, Clone, Copy)]
pub(crate) struct IndexerResponse {
    pub(crate) entity: Entity,
    pub(crate) instance: Instance,
}
impl From<(Entity, Instance)> for IndexerResponse {
    fn from(pair: (Entity, Instance)) -> Self {
        Self {
            entity: pair.0,
            instance: pair.1,
        }
    }
}
#[derive(Eq, Hash, PartialEq, Copy, Clone, Component)]
pub(crate) struct Instance(pub(crate) u32);
pub(crate) fn integrate_indexes(
    indexer_responses: Query<(Entity, &IndexerResponse)>,
    mut cmd: Commands,
) {
    indexer_responses.iter().for_each(|(entity, response)| {
        cmd.entity(response.entity).insert(response.instance);
        cmd.entity(entity).despawn();
    });
}
