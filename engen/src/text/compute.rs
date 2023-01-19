use crate::canvas::Visibility;
use crate::text::cache::Cache;
use crate::text::changes::Changes;
use crate::text::component::{Key, Keys, Placer, Text, TextOffset};
use crate::text::font::Font;
use crate::text::instance::Attributes;
use crate::text::rasterization::{Alignment, Glyph};
use crate::text::Scale;
use crate::{Area, Color, Depth, Position, Section};
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Added, Changed, Commands, Or, Query, RemovedComponents, Res};
use std::collections::{HashMap, HashSet};

pub(crate) fn compute_setup(mut cmd: Commands) {
    cmd.insert_resource(Changes::new());
    cmd.insert_resource(Cache::new());
    cmd.insert_resource(Font::default());
}

pub(crate) fn update_attrs(
    text: Query<
        (Entity, &Text, &Position, &Color, &Depth),
        (Or<(Changed<Position>, Changed<Color>, Changed<Depth>)>),
    >,
) {
}

pub(crate) fn push_compute_changes(
    mut cache: ResMut<Cache>,
    mut changes: ResMut<Changes>,
    mut text: Query<
        (
            Entity,
            &Text,
            &mut Placer,
            &mut Keys,
            &Position,
            Option<&Area>,
            &Depth,
            &Color,
            &Scale,
            &Visibility,
        ),
        // if changed position or color or depth try to just write to changes.updates if present
        (Or<(Changed<Text>, Changed<Area>)>),
    >,
    font: Res<Font>,
) {
    for (
        entity,
        text,
        mut placer,
        mut keys,
        position,
        maybe_area,
        depth,
        color,
        scale,
        visibility,
    ) in text.iter_mut()
    {
        if visibility.visible {
            placer.placer.clear();
            placer.placer.append(
                font.font_slice(),
                &fontdue::layout::TextStyle::new(text.string.as_str(), scale.px(), Font::index()),
            );
            let mut retained_keys = HashSet::new();
            let mut added_keys = HashSet::new();
            if let Some(area) = maybe_area {
                cache.bounds.insert(entity, (*position, *area).into());
                changes.bounds.insert(entity, (*position, *area).into());
            } else {
                let old_bound = cache.bounds.remove(&entity);
                if old_bound.is_some() {
                    changes.removed_bounds.insert(entity);
                }
            }
            let mut glyphs = HashSet::new();
            let mut removed_glyphs = HashSet::new();
            let mut removes = HashSet::new();
            let mut adds = HashMap::new();
            for positioned_glyph in placer.placer.glyphs() {
                let hash = positioned_glyph.key;
                let glyph = Glyph::new(hash, positioned_glyph.parent, *scale);
                let key = Key::new(entity, TextOffset(positioned_glyph.byte_offset));
                let glyph_section = Section::new(
                    *position + (positioned_glyph.x, positioned_glyph.y).into(),
                    (positioned_glyph.width, positioned_glyph.height).into(),
                );
                if let Some(area) = maybe_area {
                    let text_section = Section::new(*position, *area);
                    if !(text_section.left() < glyph_section.right()
                        && text_section.right() > glyph_section.left()
                        && text_section.top() < glyph_section.bottom()
                        && text_section.bottom() > glyph_section.top())
                    {
                        if cache.attributes.contains_key(&key) {
                            removes.insert(key);
                        }
                        continue;
                    }
                }
                let current_attributes = Attributes::new(glyph_section.position, *depth, *color);
                if cache.attributes.contains_key(&key) {
                    retained_keys.insert(key);
                    let cached_glyph = cache.glyphs.get(&key).expect("no cached glyph for key");
                    if *cached_glyph != positioned_glyph.key {
                        glyphs.insert((key, glyph));
                    } else {
                        removed_glyphs.insert((key, hash));
                    }
                    let cached_attributes =
                        cache.attributes.get(&key).expect("no cached attributes");
                    // tolerance check each value to decide if should be replaced ond go to changes.updates
                    // also store in cache if changed
                } else {
                    added_keys.insert(key);
                    adds.insert(key, (glyph_section.area, current_attributes));
                    cache.attributes.insert(key, current_attributes);
                    glyphs.insert((key, glyph));
                    cache.glyphs.insert(key, positioned_glyph.key);
                }
            }
            if !glyphs.is_empty() {
                changes.glyphs.insert(entity, glyphs);
            }
            if !removed_glyphs.is_empty() {
                changes.removed_glyphs.insert(entity, removed_glyphs);
            }
            let keys_to_remove = keys
                .keys
                .difference(&retained_keys)
                .copied()
                .collect::<HashSet<Key>>();
            removes.extend(keys_to_remove);
            if !removes.is_empty() {
                changes.removes.insert(entity, removes);
            }
            if !adds.is_empty() {
                changes.adds.insert(entity, adds);
            }
            keys.keys.extend(added_keys);
        }
    }
}

pub(crate) fn visibility(
    text: Query<(Entity, &Text, &Visibility), (Changed<Visibility>)>,
    mut cache: ResMut<Cache>,
    mut changes: ResMut<Changes>,
) {
    for (entity, text, visibility) in text.iter() {
        if visibility.visible {
            cache.visible_entities.insert(entity);
        } else {
            cache.visible_entities.remove(&entity);
        }
        changes.visibility.insert(entity, *visibility);
    }
}

pub(crate) fn text_entity_changes(
    added: Query<(Entity, &Text, &Scale), (Added<Text>)>,
    removed: RemovedComponents<Text>,
    mut changes: ResMut<Changes>,
    font: Res<Font>,
) {
    for (entity, text, scale) in added.iter() {
        changes.added_text_entities.insert(
            entity,
            (
                text.string.len() as u32,
                Alignment::new(font.character_dimensions('a', scale.px())),
            ),
        );
    }
    for entity in removed.iter() {
        changes.removed_text_entities.insert(entity);
    }
}
