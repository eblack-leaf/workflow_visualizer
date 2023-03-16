use std::collections::HashSet;

use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Added, Changed, Commands, Or, Query, RemovedComponents, Res};

use crate::icon::cache::{Cache, DifferenceHolder};
use crate::icon::mesh::ColorInvert;
use crate::icon::{IconKey, IconSecondaryColor, IconSize};
use crate::visibility::Visibility;
use crate::{Area, Color, InterfaceContext, Layer, Position, ScaleFactor};

pub(crate) fn initialization(
    icons: Query<
        (
            Entity,
            &IconSecondaryColor,
            &IconKey,
            &Position<InterfaceContext>,
            &Area<InterfaceContext>,
            &Layer,
            &Color,
            &ColorInvert,
            &Visibility,
        ),
        Or<(Added<IconSecondaryColor>, Changed<Visibility>)>,
    >,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
    mut removed_icons: RemovedComponents<IconSecondaryColor>,
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
    icons: Query<(Entity, &ColorInvert, &Visibility), Changed<ColorInvert>>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, color_invert, visibility) in icons.iter() {
        if visibility.visible() {
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
}

pub(crate) fn position_cache_check(
    icons: Query<
        (Entity, &Position<InterfaceContext>, &Visibility),
        Changed<Position<InterfaceContext>>,
    >,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, position, visibility) in icons.iter() {
        if visibility.visible() {
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
}

pub(crate) fn area_cache_check(
    icons: Query<(Entity, &Area<InterfaceContext>, &Visibility), Changed<Area<InterfaceContext>>>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, area, visibility) in icons.iter() {
        if visibility.visible() {
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
}

pub(crate) fn depth_cache_check(
    icons: Query<(Entity, &Layer, &Visibility), Changed<Layer>>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, depth, visibility) in icons.iter() {
        if visibility.visible() {
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
}

pub(crate) fn color_cache_check(
    icons: Query<(Entity, &Color, &Visibility), Changed<Color>>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, color, visibility) in icons.iter() {
        if visibility.visible() {
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
}

pub(crate) fn secondary_color_cache_check(
    icons: Query<(Entity, &IconSecondaryColor, &Visibility), Changed<IconSecondaryColor>>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, color, visibility) in icons.iter() {
        if visibility.visible() {
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
}

pub(crate) fn icon_key_cache_check(
    icons: Query<
        (
            Entity,
            &IconSecondaryColor,
            &IconKey,
            &Position<InterfaceContext>,
            &Area<InterfaceContext>,
            &Layer,
            &Color,
            &ColorInvert,
            &Visibility,
        ),
        Changed<IconKey>,
    >,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, icon, icon_key, position, area, depth, color, color_invert, visibility) in
        icons.iter()
    {
        if visibility.visible() {
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
}

pub(crate) fn frontend_setup(mut cmd: Commands) {
    cmd.insert_resource(Cache::new());
    cmd.insert_resource(DifferenceHolder::new());
}

pub(crate) fn calc_area(
    scale_factor: Res<ScaleFactor>,
    mut icons: Query<(&IconSize, &mut Area<InterfaceContext>), Changed<IconSize>>,
) {
    for (size, mut area) in icons.iter_mut() {
        match size {
            IconSize::Small | IconSize::Medium | IconSize::Large => {
                let area_guide = match size {
                    IconSize::Small => 12.0,
                    IconSize::Medium => 15.0,
                    IconSize::Large => 18.0,
                    _ => 0.0,
                };
                let scaled = area_guide * scale_factor.factor;
                *area = Area::<InterfaceContext>::new(scaled as f32, scaled as f32);
            }
            IconSize::Custom((w, h)) => {
                *area = Area::<InterfaceContext>::new(*w, *h);
            }
        }
    }
}
