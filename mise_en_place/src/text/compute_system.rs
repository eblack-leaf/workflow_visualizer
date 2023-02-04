use std::collections::HashSet;

use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{
    Added, Changed, Commands, Or, Query, RemovedComponents, Res, With, Without,
};
use fontdue::layout::TextStyle;

use crate::color::Color;
use crate::coord::{Area, Depth, Position, ScaledSection, Section};
use crate::text::cache::Cache;
use crate::text::difference::Difference;
use crate::text::extraction::Extraction;
use crate::text::font::MonoSpacedFont;
use crate::text::glyph::{Glyph, Key};
use crate::text::place::Placer;
use crate::text::render_group::TextBound;
use crate::text::scale::{TextScale, TextScaleAlignment};
use crate::text::text::Text;
use crate::visibility::Visibility;
use crate::visibility::VisibleSection;
use crate::window::ScaleFactor;

pub(crate) fn setup(scale_factor: Res<ScaleFactor>, mut cmd: Commands) {
    cmd.insert_resource(Extraction::new());
    cmd.insert_resource(MonoSpacedFont::jet_brains_mono(
        TextScale::from_alignment(TextScaleAlignment::Medium, scale_factor.factor).scale,
    ));
}

pub(crate) fn calc_scale_from_alignment(
    text: Query<
        (Entity, &TextScaleAlignment),
        (Or<(Without<TextScale>, Changed<TextScaleAlignment>)>),
    >,
    scale_factor: Res<ScaleFactor>,
    mut cmd: Commands,
) {
    for (entity, text_scale_alignment) in text.iter() {
        cmd.entity(entity).insert(TextScale::from_alignment(
            *text_scale_alignment,
            scale_factor.factor,
        ));
    }
}

pub(crate) fn manage_render_groups(
    mut text: Query<
        (
            Entity,
            &Text,
            &TextScale,
            &Position,
            &VisibleSection,
            Option<&TextBound>,
            &Depth,
            &Color,
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
        visible_section,
        maybe_bound,
        depth,
        color,
        mut cache,
        mut difference,
        visibility,
    ) in text.iter_mut()
    {
        if visibility.visible() {
            *difference = Difference::new();
            *cache = Cache::new(*position, *depth, *color, *visible_section);
            difference.position.replace(*position);
            difference.depth.replace(*depth);
            difference.color.replace(*color);
            if let Some(bounds) = maybe_bound {
                let section = Section::new(*position, bounds.area);
                cache.bound.replace(section);
                difference.bounds.replace(section);
            }
            let max = text.len();
            let unique_glyphs = text.len();
            let atlas_block = font.character_dimensions('a', scale.px());
            extraction.added_render_groups.insert(
                entity,
                (
                    max,
                    *position,
                    *visible_section,
                    *depth,
                    *color,
                    atlas_block,
                    unique_glyphs,
                ),
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
        Or<(Changed<Placer>, Changed<TextBound>, Changed<VisibleSection>)>,
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

pub(crate) fn calc_area(
    text: Query<(Entity, &Text, &TextScale), (Or<(Changed<Text>, Changed<TextScale>)>)>,
    mut cmd: Commands,
    font: Res<MonoSpacedFont>,
) {
    for (entity, text, text_scale) in text.iter() {
        let dimensions = font.character_dimensions('A', text_scale.px());
        let mut max_width_letters = 0;
        let mut lines = 0;
        for line in text.string().lines() {
            max_width_letters = max_width_letters.max(line.len());
            lines += 1;
        }
        let line_height_advancement = 3f32;
        let area = Area::new(
            dimensions.width * max_width_letters.max(1) as f32,
            dimensions.height + line_height_advancement * lines.max(1) as f32,
        );
        cmd.entity(entity).insert(area);
    }
}

pub(crate) fn bounds_diff(
    mut text: Query<
        (&Position, Option<&TextBound>, &mut Cache, &mut Difference),
        Changed<TextBound>,
    >,
) {
    for (position, maybe_bound, mut cache, mut difference) in text.iter_mut() {
        if let Some(bound) = maybe_bound {
            let section = Section::new(*position, bound.area);
            if let Some(cached_bound) = cache.bound {
                if cached_bound != section {
                    difference.bounds.replace(section);
                    cache.bound.replace(section);
                }
            } else {
                difference.bounds.replace(section);
                cache.bound.replace(section);
            }
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
            cache.depth = *depth;
        }
    }
}

pub(crate) fn position_diff(
    mut text: Query<(&Position, &mut Cache, &mut Difference), (Changed<Position>, With<Text>)>,
) {
    for (position, mut cache, mut difference) in text.iter_mut() {
        if *position != cache.position {
            difference.position.replace(*position);
            cache.position = *position;
        }
    }
}

pub(crate) fn color_diff(
    mut text: Query<(&Color, &mut Cache, &mut Difference), (Changed<Color>, With<Text>)>,
) {
    for (color, mut cache, mut difference) in text.iter_mut() {
        if *color != cache.color {
            difference.color.replace(*color);
            cache.color = *color;
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

pub(crate) fn visible_area_diff(
    mut text: Query<
        (Entity, &VisibleSection, &mut Difference, &mut Cache),
        (Changed<VisibleSection>, With<Text>),
    >,
) {
    for (entity, visible_section, mut difference, mut cache) in text.iter_mut() {
        if cache.visible_section.section != visible_section.section {
            difference.visible_section.replace(*visible_section);
            cache.visible_section = *visible_section;
        }
    }
}

pub(crate) fn discard_out_of_bounds(
    mut text: Query<
        (
            &mut Placer,
            &Position,
            &Area,
            Option<&TextBound>,
            &VisibleSection,
        ),
        Or<(Changed<Placer>, Changed<VisibleSection>, Changed<TextBound>)>,
    >,
    scale_factor: Res<ScaleFactor>,
) {
    for (mut placer, position, area, bound, visible_section) in text.iter_mut()
    {
        let bound_area = match bound {
            Some(b) => {
                let scaled = b.area.to_scaled(scale_factor.factor);
                Area::new(scaled.width, scaled.height)
            }
            None => *area,
        };
        let text_section = visible_section.section.intersection(Section::new(
            *position,
            bound_area,
        ));
        placer.reset_filtered();
        if let Some(section) = text_section {
            let mut filter_queue = HashSet::new();
            for glyph in placer.unfiltered_placement().iter() {
                let key = Key::new(glyph.byte_offset as u32);
                let glyph_section = Section::new(
                    (section.position.x + glyph.x, section.position.y + glyph.y),
                    (glyph.width, glyph.height),
                );
                if !section.is_overlapping(glyph_section) {
                    filter_queue.insert(key);
                }
            }
            placer.filter_placement(filter_queue);
        }
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
