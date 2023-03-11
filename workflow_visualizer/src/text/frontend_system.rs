use std::collections::HashSet;

use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{
    Added, Changed, Commands, Or, ParamSet, Query, RemovedComponents, Res, With, Without,
};

use crate::coord::InterfaceContext;
use crate::instance::key::Key;
use crate::text::atlas::AtlasBlock;
use crate::text::cache::Cache;
use crate::text::difference::{Difference, TextBoundDifference};
use crate::text::extraction::Extraction;
use crate::text::glyph::Glyph;
use crate::text::place::{Placer, WrapStyleComponent};
use crate::text::render_group::{RenderGroupMax, RenderGroupUniqueGlyphs, TextBound};
use crate::text::scale::{AlignedFonts, TextScale, TextScaleAlignment, TextScaleLetterDimensions};
use crate::text::text::{TextLineStructure, TextViewedContent};
use crate::visibility::Visibility;
use crate::visibility::VisibleSection;
use crate::{
    Area, DeviceContext, Layer, Position, ScaleFactor, Section, TextBuffer, TextContent,
    TextContentView, TextGridGuide,
};

pub(crate) fn setup(scale_factor: Res<ScaleFactor>, mut cmd: Commands) {
    cmd.insert_resource(Extraction::new());
    cmd.insert_resource(AlignedFonts::new(scale_factor.factor));
}

pub(crate) fn calc_scale_from_alignment(
    text: Query<(Entity, &TextScaleAlignment), Changed<TextScaleAlignment>>,
    scale_factor: Res<ScaleFactor>,
    fonts: Res<AlignedFonts>,
    mut cmd: Commands,
) {
    for (entity, text_scale_alignment) in text.iter() {
        let scale = TextScale::from_alignment(*text_scale_alignment, scale_factor.factor);
        let dimensions = fonts
            .fonts
            .get(text_scale_alignment)
            .unwrap()
            .character_dimensions('a', scale.px());
        let device_dimensions = Area::<DeviceContext>::new(dimensions.width, dimensions.height);
        cmd.entity(entity)
            .insert((scale, TextScaleLetterDimensions::new(device_dimensions)));
    }
}

pub(crate) fn calc_bound_from_guide(
    text: Query<
        (Entity, &TextGridGuide, &TextScaleAlignment),
        Or<(Without<TextBound>, Changed<TextGridGuide>)>,
    >,
    scale_factor: Res<ScaleFactor>,
    mut cmd: Commands,
    aligned_fonts: Res<AlignedFonts>,
) {
    for (entity, guide, alignment) in text.iter() {
        let font = aligned_fonts.fonts.get(alignment).expect("no aligned font");
        let character_dimensions = font.character_dimensions(
            'a',
            TextScale::from_alignment(*alignment, scale_factor.factor).px(),
        );
        let width = guide.horizontal_character_max as f32 * character_dimensions.width;
        let height = guide.line_max as f32 * character_dimensions.height;
        cmd.entity(entity).insert(TextBound::new((width, height)));
    }
}

pub(crate) fn update_content(
    mut text_query: Query<
        (
            &TextContent,
            &TextContentView,
            &mut Cache,
            &mut TextBuffer,
            &TextGridGuide,
            &mut TextViewedContent,
        ),
        Or<(Changed<TextContent>, Changed<TextContentView>)>,
    >,
) {
    for (content, content_view, mut cache, mut buffer, grid_guide, mut viewed_content) in
        text_query.iter_mut()
    {
        let new_viewed_content = content.view(content_view);
        if cache.viewed_content_string != new_viewed_content.0 {
            *buffer = TextBuffer::new(&new_viewed_content, content_view.initial_color, grid_guide);
            cache.viewed_content_string = new_viewed_content.0.clone();
            *viewed_content = new_viewed_content;
        }
    }
}

pub(crate) fn manage_render_groups(
    mut text: Query<
        (
            Entity,
            &TextBuffer,
            &TextViewedContent,
            &TextScale,
            &Position<InterfaceContext>,
            &VisibleSection,
            &TextBound,
            &Layer,
            &mut Cache,
            &mut Difference,
            &Visibility,
            &TextScaleAlignment,
        ),
        Or<(Changed<Visibility>, Added<TextBuffer>, Changed<TextScale>)>,
    >,
    mut removed: RemovedComponents<TextBuffer>,
    mut extraction: ResMut<Extraction>,
    font: Res<AlignedFonts>,
) {
    for (
        entity,
        text,
        viewed_content,
        scale,
        position,
        visible_section,
        maybe_bound,
        depth,
        mut cache,
        mut difference,
        visibility,
        text_scale_alignment,
    ) in text.iter_mut()
    {
        if visibility.visible() {
            *difference = Difference::new();
            *cache = Cache::new(
                *position,
                *depth,
                *visible_section,
                viewed_content.0.clone(),
            );
            difference.position.replace(*position);
            difference.depth.replace(*depth);
            cache.bound.replace(*maybe_bound);
            difference
                .bounds
                .replace(TextBoundDifference::Changed(*maybe_bound));
            let max = RenderGroupMax(text.num_letters().max(1));
            let unique_glyphs = RenderGroupUniqueGlyphs::from_text(text);
            extraction.added_render_groups.insert(
                entity,
                (
                    max,
                    *position,
                    *visible_section,
                    *depth,
                    AtlasBlock::new(
                        font.fonts
                            .get(text_scale_alignment)
                            .expect("no aligned font for"),
                        scale,
                    ),
                    unique_glyphs,
                    *text_scale_alignment,
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
        (
            &TextScale,
            &Placer,
            &mut Cache,
            &mut Difference,
            &Visibility,
        ),
        Or<(Changed<Placer>, Changed<TextBound>, Changed<VisibleSection>)>,
    >,
) {
    for (scale, placer, mut cache, mut difference, visibility) in text.iter_mut() {
        if visibility.visible() {
            let mut retained_keys = HashSet::new();
            let old_keys = cache.keys.clone();
            let mut keys_to_remove = HashSet::new();
            for (key, placed_glyph) in placer.filtered_placement().iter() {
                if placed_glyph.parent.is_ascii_control()
                    || placed_glyph.parent.is_ascii_whitespace()
                {
                    if cache.exists(*key) {
                        keys_to_remove.insert(*key);
                    }
                    continue;
                }
                let glyph_position = (placed_glyph.x, placed_glyph.y).into();
                let glyph_id = placed_glyph.key;
                let character = placed_glyph.parent;
                let glyph = Glyph::new(character, *scale, glyph_id);
                if cache.exists(*key) {
                    retained_keys.insert(*key);
                    if cache.glyph_position_different(*key, glyph_position) {
                        difference.update.insert(*key, glyph_position);
                        cache.glyph_positions.insert(*key, glyph_position);
                    }
                    if cache.glyph_id_different(*key, glyph_id) {
                        difference.glyph_add.insert(*key, glyph);
                        cache.glyph_ids.insert(*key, glyph_id);
                    }
                    if cache.glyph_color_different(*key, placed_glyph.user_data.color) {
                        difference
                            .glyph_color_change
                            .insert(*key, placed_glyph.user_data.color);
                        cache
                            .glyph_colors
                            .insert(*key, placed_glyph.user_data.color);
                    }
                } else {
                    difference
                        .glyph_color_change
                        .insert(*key, placed_glyph.user_data.color);
                    cache
                        .glyph_colors
                        .insert(*key, placed_glyph.user_data.color);
                    difference.add.insert(*key, glyph_position);
                    difference.glyph_add.insert(*key, glyph);
                    cache.add(*key, glyph_id, glyph_position);
                }
            }
            keys_to_remove.extend(
                old_keys
                    .difference(&retained_keys)
                    .copied()
                    .collect::<HashSet<Key>>(),
            );
            for key in keys_to_remove {
                difference.glyph_remove.insert(cache.get_glyph_id(key));
                difference.remove.insert(key);
                cache.remove(key);
            }
        }
    }
}

pub(crate) fn calc_area(
    text: Query<(Entity, &Placer), Changed<Placer>>,
    mut cmd: Commands,
    scale_factor: Res<ScaleFactor>,
) {
    for (entity, placer) in text.iter() {
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;
        for (_, glyph) in placer.unfiltered_placement().iter() {
            width = width.max(glyph.x + glyph.width as f32);
            height = height.max(glyph.y + glyph.height as f32);
        }
        if width != 0.0 && height != 0.0 {
            cmd.entity(entity)
                .insert(Area::<DeviceContext>::new(width, height).to_ui(scale_factor.factor));
        }
    }
}

pub(crate) fn bounds_diff(
    mut text: Query<(&TextBound, &mut Cache, &mut Difference), Changed<TextBound>>,
) {
    for (bound, mut cache, mut difference) in text.iter_mut() {
        if let Some(cached_bound) = cache.bound {
            if cached_bound.area != bound.area {
                difference
                    .bounds
                    .replace(TextBoundDifference::Changed(*bound));
                cache.bound.replace(*bound);
            }
        } else {
            difference
                .bounds
                .replace(TextBoundDifference::Changed(*bound));
            cache.bound.replace(*bound);
        }
    }
}

pub(crate) fn depth_diff(
    mut text: Query<(&Layer, &mut Cache, &mut Difference), (Changed<Layer>, With<TextBuffer>)>,
) {
    for (depth, mut cache, mut difference) in text.iter_mut() {
        if *depth != cache.layer {
            difference.depth.replace(*depth);
            cache.layer = *depth;
        }
    }
}

pub(crate) fn position_diff(
    mut text: Query<
        (&Position<InterfaceContext>, &mut Cache, &mut Difference),
        (Changed<Position<InterfaceContext>>, With<TextBuffer>),
    >,
) {
    for (position, mut cache, mut difference) in text.iter_mut() {
        if *position != cache.position {
            difference.position.replace(*position);
            cache.position = *position;
        }
    }
}

pub(crate) fn place(
    font: Res<AlignedFonts>,
    mut text: Query<
        (
            &TextBuffer,
            &TextScale,
            &mut Placer,
            &TextScaleAlignment,
            &WrapStyleComponent,
            &TextBound,
        ),
        Or<(
            Changed<TextBuffer>,
            Changed<TextScale>,
            Changed<Visibility>,
            Changed<TextBound>,
        )>,
    >,
) {
    for (text, scale, mut placer, text_scale_alignment, wrap_style, text_bound) in text.iter_mut() {
        placer.place(
            text,
            scale,
            font.fonts
                .get(text_scale_alignment)
                .expect("no aligned font"),
            wrap_style,
            text_bound,
        );
    }
}

pub(crate) fn visible_area_diff(
    mut text: Query<
        (&VisibleSection, &mut Difference, &mut Cache),
        (Changed<VisibleSection>, With<TextBuffer>),
    >,
) {
    for (visible_section, mut difference, mut cache) in text.iter_mut() {
        if let Some(cached_section) = cache.visible_section.section {
            if let Some(entity_section) = visible_section.section {
                if cached_section != entity_section {
                    difference.visible_section.replace(*visible_section);
                    cache.visible_section = *visible_section;
                }
            }
        }
    }
}

pub(crate) fn discard_out_of_bounds(
    mut text: Query<
        (
            &mut Placer,
            &Position<InterfaceContext>,
            &TextBound,
            &VisibleSection,
        ),
        Or<(
            Changed<Placer>,
            Changed<VisibleSection>,
            Changed<TextBound>,
            Changed<Position<InterfaceContext>>,
        )>,
    >,
    scale_factor: Res<ScaleFactor>,
) {
    for (mut placer, position, bound, visible_section) in text.iter_mut() {
        let bound_area = bound.area.to_device(scale_factor.factor);
        let scaled_position = position.to_device(scale_factor.factor);
        if let Some(visible_section) = visible_section.section {
            let text_section = visible_section
                .to_device(scale_factor.factor)
                .intersection(Section::<DeviceContext>::new(scaled_position, bound_area));
            placer.reset_filtered();
            if let Some(section) = text_section {
                let mut filter_queue = HashSet::new();
                for (key, glyph) in placer.unfiltered_placement().iter() {
                    let glyph_section = Section::<DeviceContext>::new(
                        (scaled_position.x + glyph.x, scaled_position.y + glyph.y),
                        (glyph.width, glyph.height),
                    );
                    if !section.is_overlapping(glyph_section) {
                        filter_queue.insert(*key);
                    }
                }
                placer.filter_placement(filter_queue);
            }
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

pub(crate) fn calc_line_structure(
    updated: Query<(Entity, &TextBuffer), Changed<TextBuffer>>,
    mut cmd: Commands,
) {
    for (entity, text_buffer) in updated.iter() {
        let mut max_y = 0;
        for (loc, _) in text_buffer.letters.iter() {
            if loc.y > max_y {
                max_y = loc.y;
            }
        }
        let mut letter_counts = Vec::new();
        letter_counts.resize(max_y as usize + 1, 0);
        for (loc, _) in text_buffer.letters.iter() {
            if let Some(current_max) = letter_counts.get_mut(loc.y as usize) {
                if loc.x > *current_max {
                    *current_max = loc.x;
                }
            }
        }
        if letter_counts.is_empty() {
            letter_counts.push(0);
        }
        cmd.entity(entity)
            .insert(TextLineStructure::new(letter_counts));
    }
}
