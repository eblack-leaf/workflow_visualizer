use crate::gfx::GfxSurface;
use crate::panel::renderer::PanelRenderer;
use crate::panel::{BorderColor, Cache, Difference, Extraction, PanelContentArea, PanelType};
use crate::{
    Area, Color, InterfaceContext, Layer, NullBit, Panel, Position, ScaleFactor, Visibility,
};
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Or, Query, RemovedComponents, Res};
#[cfg(target_family = "wasm")]
use bevy_ecs::prelude::{NonSend, NonSendMut};

pub(crate) fn pull_differences(
    mut extraction: ResMut<Extraction>,
    mut differential: Query<(Entity, &mut Difference, &Visibility), Changed<Difference>>,
) {
    for (entity, mut difference, visibility) in differential.iter_mut() {
        if visibility.visible() {
            extraction.differences.insert(entity, difference.clone());
            *difference = Difference::new();
        }
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
            *area - Area::from((Panel::CORNER_DEPTH * 2.0, Panel::CORNER_DEPTH * 2.0));
        content_area.0 = Area::new(
            calculated_area.width.max(0.0),
            calculated_area.height.max(0.0),
        );
    }
}
pub(crate) fn management(
    mut removed: RemovedComponents<PanelContentArea>,
    mut panels: Query<
        (
            Entity,
            &Visibility,
            &PanelType,
            &Position<InterfaceContext>,
            &Layer,
            &Color,
            &BorderColor,
            &PanelContentArea,
            &mut Difference,
        ),
        Changed<Visibility>,
    >,
    mut extraction: ResMut<Extraction>,
) {
    for entity in removed.iter() {
        extraction.removed.insert(entity);
    }
    for (
        entity,
        visibility,
        panel_type,
        pos,
        layer,
        color,
        border_color,
        content_area,
        mut difference,
    ) in panels.iter_mut()
    {
        if !visibility.visible() {
            extraction.removed.insert(entity);
        } else {
            let mut diff = Difference::new();
            diff.panel_type.replace(*panel_type);
            diff.position.replace(*pos);
            diff.content_area.replace(content_area.0);
            diff.layer.replace(*layer);
            diff.panel_color.replace(*color);
            diff.border_color.replace(border_color.0);
            *difference = diff;
        }
    }
}
pub(crate) fn panel_type_diff(
    mut panel_type_changed: Query<(&PanelType, &mut Cache, &mut Difference), Changed<PanelType>>,
) {
    for (panel_type, mut cache, mut difference) in panel_type_changed.iter_mut() {
        if let Some(p_type) = cache.panel_type {
            if *panel_type != p_type {
                difference.panel_type.replace(*panel_type);
                cache.panel_type.replace(*panel_type);
            }
        } else {
            difference.panel_type.replace(*panel_type);
            cache.panel_type.replace(*panel_type);
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
        let pos = *pos; // - Position::from(Panel::PADDING);
        if let Some(cached) = cache.position {
            if pos != cached {
                cache.position.replace(pos);
                diff.position.replace(pos);
            }
        } else {
            cache.position.replace(pos);
            diff.position.replace(pos);
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
        let padded_content_area = content_area.0;
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
    mut color_changed: Query<
        (&Color, &BorderColor, &mut Cache, &mut Difference),
        Or<(Changed<Color>, Changed<BorderColor>)>,
    >,
) {
    for (panel_color, border_color, mut cache, mut diff) in color_changed.iter_mut() {
        if let Some(cached) = cache.panel_color {
            if *panel_color != cached {
                cache.panel_color.replace(*panel_color);
                diff.panel_color.replace(*panel_color);
            }
        } else {
            cache.panel_color.replace(*panel_color);
            diff.panel_color.replace(*panel_color);
        }
        if let Some(cached) = cache.border_color {
            if border_color.0 != cached {
                cache.border_color.replace(border_color.0);
                diff.border_color.replace(border_color.0);
            }
        } else {
            cache.border_color.replace(border_color.0);
            diff.border_color.replace(border_color.0);
        }
    }
}

pub(crate) fn process_extraction(
    #[cfg(not(target_family = "wasm"))] mut renderer: ResMut<PanelRenderer>,
    #[cfg(target_family = "wasm")] mut renderer: NonSendMut<PanelRenderer>,
    mut extraction: ResMut<Extraction>,
    scale_factor: Res<ScaleFactor>,
    #[cfg(not(target_family = "wasm"))] gfx_surface: Res<GfxSurface>,
    #[cfg(target_family = "wasm")] gfx_surface: NonSend<GfxSurface>,
) {
    for entity in extraction.removed.drain() {
        let old = renderer.indexer.remove(entity);
        if let Some(o) = old {
            renderer.panel_null_bits.queue_write(o, NullBit::null());
            renderer.border_null_bits.queue_write(o, NullBit::null());
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
        renderer.panel_colors.grow(&gfx_surface, max);
        renderer.panel_null_bits.grow(&gfx_surface, max);
        renderer.border_null_bits.grow(&gfx_surface, max);
        renderer.border_colors.grow(&gfx_surface, max);
    }
    for (entity, difference) in extraction.differences.drain() {
        let index = renderer.indexer.get_index(entity).unwrap();
        if let Some(pos) = difference.position {
            renderer
                .positions
                .queue_write(index, pos.to_device(scale_factor.factor()).as_raw());
        }
        if let Some(content_area) = difference.content_area {
            renderer.content_area.queue_write(
                index,
                content_area.to_device(scale_factor.factor()).as_raw(),
            )
        }
        if let Some(layer) = difference.layer {
            renderer.layers.queue_write(index, layer);
        }
        if let Some(color) = difference.panel_color {
            renderer.panel_colors.queue_write(index, color);
        }
        if let Some(border_color) = difference.border_color {
            renderer.border_colors.queue_write(index, border_color);
        }
        if let Some(panel_type) = difference.panel_type {
            match panel_type {
                PanelType::Flat => {
                    renderer
                        .panel_null_bits
                        .queue_write(index, NullBit::not_null());
                    renderer
                        .border_null_bits
                        .queue_write(index, NullBit::null());
                }
                PanelType::Border => {
                    renderer.panel_null_bits.queue_write(index, NullBit::null());
                    renderer
                        .border_null_bits
                        .queue_write(index, NullBit::not_null());
                }
                PanelType::BorderedFlat => {
                    renderer
                        .panel_null_bits
                        .queue_write(index, NullBit::not_null());
                    renderer
                        .border_null_bits
                        .queue_write(index, NullBit::not_null());
                }
            }
        }
    }
    renderer.positions.write(&gfx_surface);
    renderer.content_area.write(&gfx_surface);
    renderer.layers.write(&gfx_surface);
    renderer.panel_colors.write(&gfx_surface);
    renderer.panel_null_bits.write(&gfx_surface);
    renderer.border_null_bits.write(&gfx_surface);
    renderer.border_colors.write(&gfx_surface);
}
