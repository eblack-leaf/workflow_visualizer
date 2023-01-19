use crate::text::changes::Changes;
use crate::text::instance::Instance;
use crate::text::Renderer;
use crate::Canvas;
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::prelude::{Res, Resource};

pub(crate) fn reset_extraction(mut extraction: ResMut<Extraction>) {
    extraction.changes.reset();
}

pub(crate) fn integrate_extraction(
    mut extraction: ResMut<Extraction>,
    mut renderer: ResMut<Renderer>,
    canvas: Res<Canvas>,
) {
    for (entity, visibility) in extraction.changes.visibility.iter() {
        if visibility.visible {
            renderer.visible_text_entities.insert(*entity);
        } else {
            renderer.visible_text_entities.remove(entity);
        }
    }
    for (entity, section) in extraction.changes.bounds.iter() {
        renderer
            .rasterizations
            .get_mut(entity)
            .expect("no rasterization for entity")
            .bounds
            .replace(*section);
    }
    for entity in extraction.changes.removed_bounds.iter() {
        renderer
            .rasterizations
            .get_mut(entity)
            .expect("no rasterization for entity")
            .bounds = None;
    }
    for (entity, removes) in extraction.changes.removes.iter() {
        let rasterization = renderer
            .rasterizations
            .get_mut(&entity)
            .expect("no rasterization");
        for key in removes.iter() {
            rasterization.instances.remove(*key);
        }
    }
    for (entity, adds) in extraction.changes.adds.iter() {
        let rasterization = renderer
            .rasterizations
            .get_mut(&entity)
            .expect("no rasterization");
        for (key, (area, attributes)) in adds.iter() {
            let tex_coords = rasterization
                .tex_coords
                .get(
                    rasterization
                        .keyed_glyph_hashes
                        .get(key)
                        .expect("no glyph hash"),
                )
                .expect("no tex coords");
            rasterization
                .instances
                .add(*key, Instance::new(*attributes, *area, *tex_coords));
        }
    }
    for (entity, updates) in extraction.changes.updates.iter() {
        let rasterization = renderer
            .rasterizations
            .get_mut(&entity)
            .expect("no rasterization");
        for (key, attributes) in updates.iter() {
            rasterization.instances.update(*key, *attributes);
        }
    }
    for (entity, glyph_changes) in extraction.changes.glyphs.iter() {
        let rasterization = renderer
            .rasterizations
            .get_mut(&entity)
            .expect("no rasterization");
        for (key, _glyph) in glyph_changes.iter() {
            let area = ();
            let tex_coords = ();
            rasterization
                .instances
                .update_non_attributes(*key, area, tex_coords);
        }
    }
    for (_entity, mut rasterization) in renderer.rasterizations.iter_mut() {
        rasterization.instances.write(&canvas);
    }
}

#[derive(Resource)]
pub(crate) struct Extraction {
    pub(crate) changes: Changes,
}

impl Extraction {
    pub(crate) fn new() -> Self {
        Self {
            changes: Changes::new(),
        }
    }
}
