use crate::text::extract::Extraction;
use crate::text::Renderer;
use crate::Canvas;
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::prelude::Res;
use std::collections::{HashMap, HashSet};

pub(crate) fn grow(
    extraction: Res<Extraction>,
    mut renderer: ResMut<Renderer>,
    canvas: Res<Canvas>,
) {
    let mut changed_entities = HashSet::new();
    let mut to_remove = HashMap::new();
    let mut to_add = HashMap::new();
    for (entity, set) in extraction.changes.removes.iter() {
        let num_to_remove = set.len();
        to_remove.insert(*entity, num_to_remove);
        changed_entities.insert(*entity);
    }
    for (entity, set) in extraction.changes.adds.iter() {
        let num_to_add = set.len();
        to_add.insert(*entity, num_to_add);
        changed_entities.insert(*entity);
    }
    for entity in changed_entities.iter() {
        let num_to_remove = to_remove.get(entity).copied();
        let num_to_add = to_add.get(entity).copied();
        let rasterization = renderer
            .rasterizations
            .get_mut(entity)
            .expect("no rasterization");
        let maybe_growth = rasterization.instances.indexer.growth_check(
            num_to_add.unwrap_or_default() as u32,
            num_to_remove.unwrap_or_default() as u32,
        );
        if let Some(growth) = maybe_growth {}
    }
    // if projected glyphs larger - grow
    // glyph growing is based on num_added_unique glyph hashes not just size from instances
}
