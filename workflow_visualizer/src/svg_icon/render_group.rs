use crate::{
    Color, GfxSurface, Indexer, InstanceAttributeManager, Layer, NullBit, RawArea, RawPosition,
};
use bevy_ecs::entity::Entity;

pub(crate) struct RenderGroup {
    pub(crate) positions: InstanceAttributeManager<RawPosition>,
    pub(crate) areas: InstanceAttributeManager<RawArea>,
    pub(crate) layers: InstanceAttributeManager<Layer>,
    pub(crate) colors: InstanceAttributeManager<Color>,
    pub(crate) null_bits: InstanceAttributeManager<NullBit>,
    pub(crate) indexer: Indexer<Entity>,
}

impl RenderGroup {
    pub(crate) fn new(gfx: &GfxSurface, max: u32) -> RenderGroup {
        Self {
            positions: InstanceAttributeManager::new(&gfx, max),
            areas: InstanceAttributeManager::new(&gfx, max),
            layers: InstanceAttributeManager::new(&gfx, max),
            colors: InstanceAttributeManager::new(&gfx, max),
            null_bits: InstanceAttributeManager::new(&gfx, max),
            indexer: Indexer::new(max),
        }
    }
}
