use std::collections::HashSet;

use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Added, Changed, Commands, Or, Query, RemovedComponents, Res, With};
use fontdue::layout::TextStyle;

use crate::text::cache::Cache;
use crate::text::difference::Difference;
use crate::text::extraction::Extraction;
use crate::text::font::MonoSpacedFont;
use crate::text::glyph::{Glyph, Key};
use crate::text::place::Placer;
use crate::text::scale::TextScale;
use crate::text::text::Text;
use crate::visibility::Visibility;
use crate::{Area, Color, Depth, Position, Section};

pub(crate) fn setup(mut cmd: Commands) {
    cmd.insert_resource(Extraction::new());
    cmd.insert_resource(MonoSpacedFont::jet_brains_mono(30u32));
}

pub(crate) fn manage_render_groups(
    mut text: Query<
        (
            Entity,
            &Text,
            &TextScale,
            &Position,
            &Depth,
            &Color,
            Option<&Area>,
            &mut Cache,
            &mut Difference,
            &Visibility,
        ),
        Or<(Changed<Visibility>, Added<Text>, Changed<TextScale>)>,
    >,
    removed: RemovedComponents<Text>,
    mut extraction: ResMut<Extraction>,
    font: Res<MonoSpacedFont>,
) {
    for (
        entity,
        text,
        scale,
        position,
        depth,
        color,
        maybe_area,
        mut cache,
        mut difference,
        visibility,
    ) in text.iter_mut()
    {
        if visibility.visible() {
            *difference = Difference::new();
            *cache = Cache::new(*position, *depth, *color);
            difference.position.replace(*position);
            difference.depth.replace(*depth);
            difference.color.replace(*color);
            if let Some(bounds) = maybe_area {
                let section = Section::new(*position, *bounds);
                cache.bound.replace(section);
                difference.bounds.replace(section);
            }
            let max = text.len();
            let unique_glyphs = text.len();
            let atlas_block = font.character_dimensions('a', scale.px());
            extraction.added_render_groups.insert(
                entity,
                (max, *position, *depth, *color, atlas_block, unique_glyphs),
            );
        } else {
            extraction.removed_render_groups.insert(entity);
        }
    }
    for entity in removed.iter() {
        extraction.removed_render_groups.insert(entity);
    }
}

pub(crate) fn letter_diff(
    mut text: Query<
        (&TextScale, &mut Placer, &mut Cache, &mut Difference),
        Or<(Changed<Placer>, Changed<Area>)>,
    >,
) {
    for (scale, mut placer, mut cache, mut difference) in text.iter_mut() {
        let mut retained_keys = HashSet::new();
        let old_keys = cache.keys.clone();
        for placed_glyph in placer.filtered_placement().iter() {
            let key = Key::new(placed_glyph.byte_offset as u32);
            let glyph_position = (placed_glyph.x, placed_glyph.y).into();
            let glyph_id = placed_glyph.key;
            let character = placed_glyph.parent;
            let glyph = Glyph::new(character, *scale, glyph_id);
            if cache.exists(key) {
                retained_keys.insert(key);
                if cache.glyph_position_different(key, glyph_position) {
                    difference.update.insert(key, glyph_position);
                    cache.glyph_positions.insert(key, glyph_position);
                }
                if cache.glyph_id_different(key, glyph_id) {
                    difference.glyph_add.insert(key, glyph);
                    cache.glyph_ids.insert(key, glyph_id);
                }
            } else {
                difference.add.insert(key, glyph_position);
                difference.glyph_add.insert(key, glyph);
                cache.add(key, glyph_id, glyph_position);
            }
        }
        let keys_to_remove = old_keys
            .difference(&retained_keys)
            .copied()
            .collect::<HashSet<Key>>();
        for key in keys_to_remove {
            difference.glyph_remove.insert(cache.get_glyph_id(key));
            difference.remove.insert(key);
            cache.remove(key);
        }
    }
}

pub(crate) fn bounds_diff(
    mut text: Query<(&Position, Option<&Area>, &mut Cache, &mut Difference), Changed<Area>>,
) {
    for (position, maybe_area, mut cache, mut difference) in text.iter_mut() {
        if let Some(area) = maybe_area {
            let section = Section::new(*position, *area);
            difference.bounds.replace(section);
            cache.bound.replace(section);
        } else if cache.bound.is_some() {
            difference.bounds = None;
            cache.bound.take();
        }
    }
}

pub(crate) fn depth_diff(
    mut text: Query<(&Depth, &mut Cache, &mut Difference), (Changed<Depth>, With<Text>)>,
) {
    for (depth, mut cache, mut difference) in text.iter_mut() {
        if *depth != cache.depth {
            difference.depth.replace(*depth);
        }
    }
}

pub(crate) fn position_diff(
    mut text: Query<(&Position, &mut Cache, &mut Difference), (Changed<Position>, With<Text>)>,
) {
    for (position, mut cache, mut difference) in text.iter_mut() {
        if *position != cache.position {
            difference.position.replace(*position);
        }
    }
}

pub(crate) fn color_diff(
    mut text: Query<(&Color, &mut Cache, &mut Difference), (Changed<Color>, With<Text>)>,
) {
    for (color, mut cache, mut difference) in text.iter_mut() {
        if *color != cache.color {
            difference.color.replace(*color);
        }
    }
}

pub(crate) fn place(
    mut dirty_text: Query<
        (&Text, &TextScale, &mut Placer, &Visibility),
        Or<(Changed<Text>, Changed<TextScale>, Changed<Visibility>)>,
    >,
    font: Res<MonoSpacedFont>,
) {
    for (text, scale, mut placer, visibility) in dirty_text.iter_mut() {
        if visibility.visible() {
            placer.place(text, scale, &font);
        }
    }
}

pub(crate) fn discard_out_of_bounds(
    mut text: Query<
        (&mut Placer, &Position, &Area, &mut Cache, &mut Difference),
        Or<(Changed<Placer>, Changed<Area>)>,
    >,
) {
    for (mut placer, position, area, mut cache, mut difference) in text.iter_mut() {
        let text_section = Section::new(*position, *area);
        placer.reset_filtered();
        let mut filter_queue = HashSet::new();
        for glyph in placer.unfiltered_placement().iter() {
            let key = Key::new(glyph.byte_offset as u32);
            let glyph_section = Section::new(
                (position.x + glyph.x, position.y + glyph.y),
                (glyph.width, glyph.height),
            );
            if !text_section.is_overlapping(glyph_section) {
                filter_queue.insert(key);
            }
        }
        placer.filter_placement(filter_queue);
    }
}

pub(crate) fn pull_differences(
    mut extraction: ResMut<Extraction>,
    mut differences: Query<(Entity, &mut Difference, &Visibility), Changed<Difference>>,
) {
    for (entity, mut difference, visibility) in differences.iter_mut() {
        if visibility.visible() {
            extraction.differences.insert(entity, difference.clone());
            difference.reset();
        }
    }
}
