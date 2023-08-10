use bevy_ecs::prelude::{Changed, Entity, Query, RemovedComponents, Res, ResMut};

use crate::icon::bitmap::IconBitmapLayout;
use crate::icon::cache::{Cache, Difference};
use crate::icon::component::{IconId, IconScale};
use crate::icon::renderer::IconRenderer;
use crate::{
    Area, Color, GfxSurface, InterfaceContext, Layer, NullBit, Position, ScaleFactor, Visibility,
};

pub(crate) fn calc_area(
    mut icons: Query<(&mut Area<InterfaceContext>, &IconScale), Changed<IconScale>>,
) {
    for (mut area, scale) in icons.iter_mut() {
        let px = scale.px();
        let new_area = Area::new(px, px);
        *area = new_area;
    }
}

pub(crate) fn management(
    mut icons: Query<
        (
            Entity,
            &Position<InterfaceContext>,
            &Area<InterfaceContext>,
            &Layer,
            &Color,
            &IconId,
            &Visibility,
            &mut Difference,
        ),
        Changed<Visibility>,
    >,
    mut removed: RemovedComponents<IconScale>,
    mut icon_renderer: ResMut<IconRenderer>,
) {
    for (entity, pos, area, layer, color, id, visibility, mut difference) in icons.iter_mut() {
        if visibility.visible() {
            difference.attributes.position.replace(*pos);
            difference.attributes.area.replace(*area);
            difference.attributes.layer.replace(*layer);
            difference.attributes.positive_space_color.replace(*color);
            difference.attributes.icon_id.replace(id.clone());
            difference.create = true;
        } else {
            difference.remove = true;
        }
    }
    for entity in removed.iter() {
        let index = icon_renderer.indexer.remove(entity);
        if let Some(i) = index {
            icon_renderer
                .null_bit_attribute
                .queue_write(i, NullBit::null());
        }
    }
}

pub(crate) fn position_diff(
    mut icons: Query<
        (&Position<InterfaceContext>, &mut Cache, &mut Difference),
        Changed<Position<InterfaceContext>>,
    >,
) {
    for (pos, mut cache, mut difference) in icons.iter_mut() {
        if let Some(cached_pos) = cache.attributes.position.as_ref() {
            if *pos != *cached_pos {
                difference.attributes.position.replace(*pos);
            }
        }
        cache.attributes.position.replace(*pos);
    }
}

pub(crate) fn area_diff(
    mut icons: Query<
        (&Area<InterfaceContext>, &mut Cache, &mut Difference),
        Changed<Area<InterfaceContext>>,
    >,
) {
    for (area, mut cache, mut difference) in icons.iter_mut() {
        if let Some(cached_area) = cache.attributes.area.as_ref() {
            if *area != *cached_area {
                difference.attributes.area.replace(*area);
            }
        }
        cache.attributes.area.replace(*area);
    }
}

pub(crate) fn layer_diff(mut icons: Query<(&Layer, &mut Cache, &mut Difference), Changed<Layer>>) {
    for (layer, mut cache, mut difference) in icons.iter_mut() {
        if let Some(cached_layer) = cache.attributes.layer.as_ref() {
            if *cached_layer != *layer {
                difference.attributes.layer.replace(*layer);
            }
        }
        cache.attributes.layer.replace(*layer);
    }
}

pub(crate) fn positive_space_color_diff(
    mut icons: Query<(&Color, &mut Cache, &mut Difference), Changed<Color>>,
) {
    for (color, mut cache, mut difference) in icons.iter_mut() {
        if let Some(cached_pos_color) = cache.attributes.positive_space_color.as_ref() {
            if *cached_pos_color != *color {
                difference.attributes.positive_space_color.replace(*color);
            }
        }
        cache.attributes.positive_space_color.replace(*color);
    }
}

pub(crate) fn icon_id_diff(
    mut icons: Query<(&IconId, &mut Cache, &mut Difference), Changed<IconId>>,
) {
    for (id, mut cache, mut difference) in icons.iter_mut() {
        if let Some(cached_id) = cache.attributes.icon_id.as_ref() {
            if *cached_id != *id {
                difference.attributes.icon_id.replace(id.clone());
            }
        }
        cache.attributes.icon_id.replace(id.clone());
    }
}

pub(crate) fn read_differences(
    mut icons: Query<(Entity, &mut Difference), Changed<Difference>>,
    mut icon_renderer: ResMut<IconRenderer>,
    scale_factor: Res<ScaleFactor>,
    icon_bitmap_layout: Res<IconBitmapLayout>,
    gfx: Res<GfxSurface>,
) {
    for (entity, mut difference) in icons.iter_mut() {
        if difference.remove {
            let index = icon_renderer.indexer.remove(entity);
            if let Some(i) = index {
                icon_renderer
                    .null_bit_attribute
                    .queue_write(i, NullBit::null());
            }
            difference.remove = false;
        }
        if difference.create {
            let index = icon_renderer.indexer.next(entity);
            icon_renderer
                .null_bit_attribute
                .queue_write(index, NullBit::not_null());
            difference.create = false;
        }
        if let Some(pos) = difference.attributes.position.take() {
            if let Some(index) = icon_renderer.indexer.get_index(entity) {
                icon_renderer
                    .pos_attribute
                    .queue_write(index, pos.to_device(scale_factor.factor()).as_raw());
            }
        }
        if let Some(area) = difference.attributes.area.take() {
            if let Some(index) = icon_renderer.indexer.get_index(entity) {
                icon_renderer
                    .area_attribute
                    .queue_write(index, area.to_device(scale_factor.factor()).as_raw());
            }
        }
        if let Some(layer) = difference.attributes.layer.take() {
            if let Some(index) = icon_renderer.indexer.get_index(entity) {
                icon_renderer.layer_attribute.queue_write(index, layer);
            }
        }
        if let Some(color) = difference.attributes.positive_space_color.take() {
            if let Some(index) = icon_renderer.indexer.get_index(entity) {
                icon_renderer.color_attribute.queue_write(index, color);
            }
        }
        if let Some(id) = difference.attributes.icon_id.take() {
            if let Some(index) = icon_renderer.indexer.get_index(entity) {
                icon_renderer.tex_coords_attribute.queue_write(
                    index,
                    icon_bitmap_layout
                        .bitmap_locations
                        .get(&id)
                        .copied()
                        .expect("icon bitmap layout"),
                );
            }
        }
    }
    if icon_renderer.indexer.should_grow() {
        let max = icon_renderer.indexer.max();
        icon_renderer.pos_attribute.grow(&gfx, max);
        icon_renderer.area_attribute.grow(&gfx, max);
        icon_renderer.layer_attribute.grow(&gfx, max);
        icon_renderer.color_attribute.grow(&gfx, max);
        icon_renderer.tex_coords_attribute.grow(&gfx, max);
        icon_renderer.null_bit_attribute.grow(&gfx, max);
    }
    icon_renderer.pos_attribute.write(&gfx);
    icon_renderer.area_attribute.write(&gfx);
    icon_renderer.layer_attribute.write(&gfx);
    icon_renderer.color_attribute.write(&gfx);
    icon_renderer.tex_coords_attribute.write(&gfx);
    icon_renderer.null_bit_attribute.write(&gfx);
}