use std::collections::HashSet;

use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Added, Changed, Commands, Or, Query, RemovedComponents, Res};

use crate::{
    Area, Color, Depth, Icon, IconKey, IconSize, Position, ScaleFactor, UIView, Visibility,
};
use crate::icon::cache::{Cache, DifferenceHolder};
use crate::icon::interface::IconAreaGuide;
use crate::icon::mesh::ColorInvert;

pub(crate) fn initialization(
    icons: Query<
        (
            Entity,
            &Icon,
            &IconKey,
            &Position<UIView>,
            &Area<UIView>,
            &Depth,
            &Color,
            &ColorInvert,
            &Visibility,
        ),
        Or<(Added<Icon>, Changed<Visibility>)>,
    >,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
    removed_icons: RemovedComponents<Icon>,
) {
    let mut removals = HashSet::new();
    for entity in removed_icons.iter() {
        removals.insert(entity);
    }
    for (entity, icon, icon_key, position, area, depth, color, color_invert, visibility) in
    icons.iter()
    {
        if visibility.visible() {
            difference_holder
                .differences
                .as_mut()
                .unwrap()
                .icon_adds
                .insert(
                    entity,
                    (
                        *icon_key,
                        *position,
                        *area,
                        *depth,
                        *color,
                        icon.secondary_color,
                        *color_invert,
                    ),
                );
            cache.icon_key.insert(entity, *icon_key);
            cache.position.insert(entity, *position);
            cache.area.insert(entity, *area);
            cache.depth.insert(entity, *depth);
            cache.color.insert(entity, *color);
            cache.secondary_color.insert(entity, icon.secondary_color);
            cache.color_invert.insert(entity, *color_invert);
        } else {
            removals.insert(entity);
        }
    }
    for removed in removals {
        difference_holder
            .differences
            .as_mut()
            .unwrap()
            .icon_removes
            .insert(removed);
        let _icon_key = cache.icon_key.remove(&removed);
    }
}

pub(crate) fn color_invert_cache_check(
    icons: Query<(Entity, &ColorInvert), Changed<ColorInvert>>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, color_invert) in icons.iter() {
        let cached_value = cache.color_invert.get(&entity);
        if let Some(val) = cached_value {
            if color_invert.signal != val.signal {
                difference_holder
                    .differences
                    .as_mut()
                    .unwrap()
                    .color_invert
                    .insert(entity, *color_invert);
                cache.color_invert.insert(entity, *color_invert);
            }
        }
    }
}

pub(crate) fn position_cache_check(
    icons: Query<(Entity, &Position<UIView>), Changed<Position<UIView>>>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, position) in icons.iter() {
        let cached_value = cache.position.get(&entity);
        if let Some(val) = cached_value {
            if position != val {
                difference_holder
                    .differences
                    .as_mut()
                    .unwrap()
                    .position
                    .insert(entity, *position);
                cache.position.insert(entity, *position);
            }
        }
    }
}

pub(crate) fn area_cache_check(
    icons: Query<(Entity, &Area<UIView>), Changed<Area<UIView>>>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, area) in icons.iter() {
        let cached_value = cache.area.get(&entity);
        if let Some(val) = cached_value {
            if area != val {
                difference_holder
                    .differences
                    .as_mut()
                    .unwrap()
                    .area
                    .insert(entity, *area);
                cache.area.insert(entity, *area);
            }
        }
    }
}

pub(crate) fn depth_cache_check(
    icons: Query<(Entity, &Depth), Changed<Depth>>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, depth) in icons.iter() {
        let cached_value = cache.depth.get(&entity);
        if let Some(val) = cached_value {
            if depth != val {
                difference_holder
                    .differences
                    .as_mut()
                    .unwrap()
                    .depth
                    .insert(entity, *depth);
                cache.depth.insert(entity, *depth);
            }
        }
    }
}

pub(crate) fn color_cache_check(
    icons: Query<(Entity, &Color), Changed<Color>>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, color) in icons.iter() {
        let cached_value = cache.color.get(&entity);
        if let Some(val) = cached_value {
            if color != val {
                difference_holder
                    .differences
                    .as_mut()
                    .unwrap()
                    .color
                    .insert(entity, *color);
                cache.color.insert(entity, *color);
            }
        }
    }
}

pub(crate) fn secondary_color_cache_check(
    icons: Query<(Entity, &Icon), Changed<Icon>>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, color) in icons.iter() {
        let cached_value = cache.secondary_color.get(&entity);
        if let Some(val) = cached_value {
            if color.secondary_color != *val {
                difference_holder
                    .differences
                    .as_mut()
                    .unwrap()
                    .secondary_color
                    .insert(entity, color.secondary_color);
                cache.color.insert(entity, color.secondary_color);
            }
        }
    }
}

pub(crate) fn icon_key_cache_check(
    icons: Query<
        (
            Entity,
            &Icon,
            &IconKey,
            &Position<UIView>,
            &Area<UIView>,
            &Depth,
            &Color,
            &ColorInvert,
        ),
        Changed<IconKey>,
    >,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, icon, icon_key, position, area, depth, color, color_invert) in icons.iter() {
        let cached_value = cache.icon_key.get(&entity);
        if let Some(val) = cached_value {
            if icon_key != val {
                difference_holder
                    .differences
                    .as_mut()
                    .unwrap()
                    .icon_removes
                    .insert(entity);
                difference_holder
                    .differences
                    .as_mut()
                    .unwrap()
                    .icon_adds
                    .insert(
                        entity,
                        (
                            *icon_key,
                            *position,
                            *area,
                            *depth,
                            *color,
                            icon.secondary_color,
                            *color_invert,
                        ),
                    );
                cache.icon_key.insert(entity, *icon_key);
            }
        }
    }
}

pub(crate) fn frontend_setup(mut cmd: Commands) {
    cmd.insert_resource(Cache::new());
    cmd.insert_resource(DifferenceHolder::new());
    cmd.insert_resource(IconAreaGuide::default());
}

pub(crate) fn calc_area(
    icon_area_guide: Res<IconAreaGuide>,
    scale_factor: Res<ScaleFactor>,
    icons: Query<(Entity, &IconSize), Changed<IconSize>>,
    mut cmd: Commands,
) {
    for (entity, size) in icons.iter() {
        match size {
            IconSize::Small | IconSize::Medium | IconSize::Large => {
                let area_guide = *icon_area_guide.guide.get(size).unwrap();
                let scaled = area_guide as f64 * scale_factor.factor;
                cmd.entity(entity)
                    .insert(Area::<UIView>::new(scaled as f32, scaled as f32));
            }
            IconSize::Custom((w, h)) => {
                cmd.entity(entity).insert(Area::<UIView>::new(*w as f32, *h as f32));
            }
        }
    }
}
