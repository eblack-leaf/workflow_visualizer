use crate::coord::Position;
use crate::text::attribute::add::Add;
use crate::text::attribute::Coordinator;
use anymap::AnyMap;
use bevy_ecs::prelude::{Commands, Component, Entity, NonSendMut, Query, ResMut, Resource};
use std::collections::{HashMap, HashSet};

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
    pub(crate) fn should_grow(&self) -> bool {
        self.current > self.max
    }
}
#[derive(Eq, Hash, PartialEq, Copy, Clone, Component)]
pub(crate) struct GlyphOffset(pub(crate) u32);
#[derive(Eq, Hash, PartialEq, Copy, Clone, Component)]
pub(crate) struct GlyphKey(pub(crate) (Entity, GlyphOffset));
impl From<(Entity, GlyphOffset)> for GlyphKey {
    fn from(pair: (Entity, GlyphOffset)) -> Self {
        Self(pair)
    }
}
#[derive(Component, Clone, Copy)]
pub(crate) struct IndexerResponse {
    pub(crate) index_key: GlyphKey,
    pub(crate) instance: Instance,
}
impl From<(GlyphKey, Instance)> for IndexerResponse {
    fn from(pair: (GlyphKey, Instance)) -> Self {
        Self {
            index_key: pair.0.into(),
            instance: pair.1,
        }
    }
}
#[derive(Eq, Hash, PartialEq, Copy, Clone, Component)]
pub(crate) struct Instance(pub(crate) u32);
pub(crate) struct GlyphCache {
    pub(crate) instance_cache: HashMap<GlyphKey, Instance>,
    pub(crate) attribute_cache: AnyMap,
    pub(crate) updates: AnyMap,
    pub(crate) removes: HashSet<Instance>,
    pub(crate) adds: Vec<Add>,
}
impl GlyphCache {
    pub(crate) fn new() -> Self {
        Self {
            instance_cache: HashMap::new(),
            attribute_cache: {
                let mut map = AnyMap::new();
                map.insert(HashMap::<GlyphKey, Position>::new());
                // ...
                map
            },
            updates: {
                let mut map = AnyMap::new();
                map.insert(HashMap::<GlyphKey, Position>::new());
                // ...
                map
            },
            removes: HashSet::new(),
            adds: Vec::new(),
        }
    }
    pub(crate) fn integrate(&mut self, response: &IndexerResponse) {
        self.instance_cache
            .insert(response.index_key, response.instance);
    }
    pub(crate) fn assign<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
    >(
        &mut self,
        glyph_key: GlyphKey,
        attr: Attribute,
    ) {
        self.attribute_cache
            .get_mut::<HashMap<GlyphKey, Attribute>>()
            .unwrap()
            .insert(glyph_key, attr);
    }
}
pub(crate) fn integrate_indexes(
    indexer_responses: Query<(Entity, &IndexerResponse)>,
    mut cache: NonSendMut<GlyphCache>,
    mut cmd: Commands,
) {
    for response in indexer_responses.iter() {
        cache.integrate(response.1);
        cmd.entity(response.0).despawn();
    }
}
