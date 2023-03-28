use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Query, RemovedComponents, Res, With};

use crate::gfx::GfxSurface;
use crate::panel::renderer::PanelRenderer;
use crate::panel::{Cache, Difference, Extraction, PanelContentArea};
use crate::{
    Area, Color, InterfaceContext, Layer, NullBit, Panel, Position, ScaleFactor, Visibility,
};

pub(crate) fn pull_differences(
    mut extraction: ResMut<Extraction>,
    mut differential: Query<(Entity, &mut Difference), Changed<Difference>>,
) {
    for (entity, mut difference) in differential.iter_mut() {
        extraction.differences.insert(entity, difference.clone());
        *difference = Difference::new();
    }
}

pub fn calc_content_area(
    mut content_changed: Query<
        (&mut PanelContentArea, &Area<InterfaceContext>),
        Changed<Area<InterfaceContext>>,
    >,
) {
    for (mut content_area, area) in content_changed.iter_mut() {
        let calculated_area =
            area - Area::from((Panel::CORNER_DEPTH * 2.0, Panel::CORNER_DEPTH * 2.0));
        *content_area.0 = calculated_area;
    }
}
pub(crate) fn management(
    mut removed: RemovedComponents<PanelContentArea>,
    lost_visibility: Query<(Entity, &Visibility), (With<PanelContentArea>, Changed<Visibility>)>,
    mut extraction: ResMut<Extraction>,
) {
    for entity in removed.iter() {
        extraction.removed.insert(entity);
    }
    for (entity, visibility) in lost_visibility.iter() {
        if !visibility.visible() {
            extraction.removed.insert(entity);
        }
    }
}
pub(crate) fn position_diff(
    mut pos_changed: Query<
        (&Position<InterfaceContext>, &mut Cache, &mut Difference),
        Changed<Position<InterfaceContext>>,
    >,
) {
    for (pos, mut cache, mut diff) in pos_changed.iter_mut() {
        if let Some(cached) = cache.position {
            if *pos != cached {
                cache.position.replace(*pos);
                diff.position.replace(*pos);
            }
        } else {
            cache.position.replace(*pos);
            diff.position.replace(*pos);
        }
    }
}

pub(crate) fn content_area_diff(
    mut content_area_changed: Query<
        (&PanelContentArea, &mut Cache, &mut Difference),
        Changed<PanelContentArea>,
    >,
) {
    for (content_area, mut cache, mut diff) in content_area_changed.iter_mut() {
        let padded_content_area = content_area.0 + Area::from(Panel::PADDING);
        if let Some(cached) = cache.content_area {
            if padded_content_area != cached {
                cache.content_area.replace(padded_content_area);
                diff.content_area.replace(padded_content_area);
            }
        } else {
            cache.content_area.replace(padded_content_area);
            diff.content_area.replace(padded_content_area);
        }
    }
}

pub(crate) fn layer_diff(
    mut layer_changed: Query<(&Layer, &mut Cache, &mut Difference), Changed<Layer>>,
) {
    for (layer, mut cache, mut diff) in layer_changed.iter_mut() {
        if let Some(cached) = cache.layer {
            if *layer != cached {
                cache.layer.replace(*layer);
                diff.layer.replace(*layer);
            }
        } else {
            cache.layer.replace(*layer);
            diff.layer.replace(*layer);
        }
    }
}

pub(crate) fn color_diff(
    mut color_changed: Query<(&Color, &mut Cache, &mut Difference), Changed<Color>>,
) {
    for (color, mut cache, mut diff) in color_changed.iter_mut() {
        if let Some(cached) = cache.color {
            if *color != cached {
                cache.color.replace(*color);
                diff.color.replace(*color);
            }
        } else {
            cache.color.replace(*color);
            diff.color.replace(*color);
        }
    }
}

pub(crate) fn process_extraction(
    mut renderer: ResMut<PanelRenderer>,
    mut extraction: ResMut<Extraction>,
    scale_factor: Res<ScaleFactor>,
    gfx_surface: Res<GfxSurface>,
) {
    for entity in extraction.removed.drain() {
        let old = renderer.indexer.remove(entity);
        if let Some(o) = old {
            renderer.null_bits.queue_write(o, NullBit::null());
        }
    }
    for (entity, _difference) in extraction.differences.iter() {
        if renderer.indexer.get_index(*entity).is_none() {
            let _ = renderer.indexer.next(*entity);
        }
    }
    if renderer.indexer.should_grow() {
        let max = renderer.indexer.max();
        renderer.positions.grow(&gfx_surface, max);
        renderer.content_area.grow(&gfx_surface, max);
        renderer.layers.grow(&gfx_surface, max);
        renderer.colors.grow(&gfx_surface, max);
        renderer.null_bits.grow(&gfx_surface, max);
    }
    for (entity, difference) in extraction.differences.drain() {
        let index = renderer.indexer.get_index(entity).unwrap();
        if let Some(pos) = difference.position {
            renderer
                .positions
                .queue_write(index, pos.to_device(scale_factor.factor).as_raw());
        }
        if let Some(content_area) = difference.content_area {
            renderer
                .content_area
                .queue_write(index, content_area.to_device(scale_factor.factor).as_raw())
        }
        if let Some(layer) = difference.layer {
            renderer.layers.queue_write(index, layer);
        }
        if let Some(color) = difference.color {
            renderer.colors.queue_write(index, color);
        }
    }
    renderer.positions.write_attribute(&gfx_surface);
    renderer.content_area.write_attribute(&gfx_surface);
    renderer.layers.write_attribute(&gfx_surface);
    renderer.colors.write_attribute(&gfx_surface);
    renderer.null_bits.write_attribute(&gfx_surface);
}
