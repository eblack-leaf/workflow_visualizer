use std::collections::HashSet;

use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{
    Added, Changed, Commands, Or, ParamSet, Query, RemovedComponents, Res, With, Without,
};

use crate::color::Color;
use crate::coord::{Area, Depth, DeviceView, Position, Section, UIView};
use crate::key::{Key, KeyFactory};
use crate::text::atlas::AtlasBlock;
use crate::text::cache::Cache;
use crate::text::difference::{Difference, TextBoundDifference};
use crate::text::extraction::Extraction;
use crate::text::glyph::Glyph;
use crate::text::place::Placer;
use crate::text::render_group::{RenderGroupMax, RenderGroupUniqueGlyphs, TextBound};
use crate::text::scale::{AlignedFonts, TextScale, TextScaleAlignment};
use crate::text::text::Text;
use crate::visibility::Visibility;
use crate::visibility::VisibleSection;
use crate::window::ScaleFactor;
use crate::{AreaAdjust, TextBoundGuide};

pub(crate) fn setup(scale_factor: Res<ScaleFactor>, mut cmd: Commands) {
    cmd.insert_resource(Extraction::new());
    cmd.insert_resource(AlignedFonts::new(scale_factor.factor));
}

pub(crate) fn intercept_area_adjust(
    attempted_area_adjust: Query<Entity, (With<Text>, With<AreaAdjust<UIView>>)>,
    mut cmd: Commands,
) {
    for entity in attempted_area_adjust.iter() {
        cmd.entity(entity).remove::<AreaAdjust<UIView>>();
    }
}

pub(crate) fn calc_scale_from_alignment(
    text: Query<
        (Entity, &TextScaleAlignment),
        Or<(Without<TextScale>, Changed<TextScaleAlignment>)>,
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

pub(crate) fn calc_bound_from_guide(
    text: Query<
        (Entity, &TextBoundGuide, &TextScaleAlignment),
        Or<(Without<TextBound>, Changed<TextBoundGuide>)>,
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

pub(crate) fn manage_render_groups(
    mut text: Query<
        (
            Entity,
            &Text,
            &TextScale,
            &Position<UIView>,
            &VisibleSection,
            Option<&TextBound>,
            &Depth,
            &mut Cache,
            &mut Difference,
            &Visibility,
            &TextScaleAlignment,
        ),
        Or<(Changed<Visibility>, Added<Text>, Changed<TextScale>)>,
    >,
    removed: RemovedComponents<Text>,
    mut extraction: ResMut<Extraction>,
    font: Res<AlignedFonts>,
) {
    for (
        entity,
        text,
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
            *cache = Cache::new(*position, *depth, *visible_section);
            difference.position.replace(*position);
            difference.depth.replace(*depth);
            if let Some(bounds) = maybe_bound {
                cache.bound.replace(*bounds);
                difference
                    .bounds
                    .replace(TextBoundDifference::Changed(*bounds));
            }
            let max = RenderGroupMax(text.length().max(1));
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

pub(crate) fn calc_area(text: Query<(Entity, &Placer), Changed<Placer>>, mut cmd: Commands) {
    for (entity, placer) in text.iter() {
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;
        for (_, glyph) in placer.unfiltered_placement().iter() {
            width = width.max(glyph.x + glyph.width as f32);
            height = height.max(glyph.y + glyph.height as f32);
        }
        if width != 0.0 && height != 0.0 {
            cmd.entity(entity)
                .insert(Area::<UIView>::new(width, height));
        }
    }
}

pub(crate) fn bounds_diff(
    mut text: ParamSet<(
        Query<(&TextBound, &mut Cache, &mut Difference), Changed<TextBound>>,
        Query<(&mut Cache, &mut Difference)>,
    )>,
    removed: RemovedComponents<TextBound>,
) {
    for (bound, mut cache, mut difference) in text.p0().iter_mut() {
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
    let mut query = text.p1();
    for entity in removed.iter() {
        let (mut cache, mut difference) = query.get_mut(entity).expect("no entity present");
        difference.bounds = Option::from(TextBoundDifference::Removed);
        cache.bound.take();
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
    mut text: Query<
        (&Position<UIView>, &mut Cache, &mut Difference),
        (Changed<Position<UIView>>, With<Text>),
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
    removed_bounds: RemovedComponents<TextBound>,
    mut text: ParamSet<(
        Query<
            (
                &Text,
                &TextScale,
                &mut Placer,
                &TextScaleAlignment,
                Option<&TextBound>,
            ),
            Or<(
                Changed<Text>,
                Changed<TextScale>,
                Changed<Visibility>,
                Changed<TextBound>,
            )>,
        >,
        Query<(&Text, &TextScale, &mut Placer, &TextScaleAlignment)>,
    )>,
) {
    for (text, scale, mut placer, text_scale_alignment, maybe_text_bound) in text.p0().iter_mut() {
        placer.place(
            text,
            scale,
            font.fonts
                .get(text_scale_alignment)
                .expect("no aligned font"),
            maybe_text_bound,
        );
    }
    let mut query = text.p1();
    for removed in removed_bounds.iter() {
        let (text, scale, mut placer, text_scale_alignment) =
            query.get_mut(removed).expect("no text for entity");
        placer.place(
            text,
            scale,
            font.fonts
                .get(text_scale_alignment)
                .expect("no aligned font"),
            None,
        );
    }
}

pub(crate) fn visible_area_diff(
    mut text: Query<
        (&VisibleSection, &mut Difference, &mut Cache),
        (Changed<VisibleSection>, With<Text>),
    >,
) {
    for (visible_section, mut difference, mut cache) in text.iter_mut() {
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
            &Position<UIView>,
            &Area<UIView>,
            Option<&TextBound>,
            &VisibleSection,
        ),
        Or<(
            Changed<Placer>,
            Changed<VisibleSection>,
            Changed<TextBound>,
            Changed<Position<UIView>>,
        )>,
    >,
    scale_factor: Res<ScaleFactor>,
) {
    for (mut placer, position, area, bound, visible_section) in text.iter_mut() {
        let bound_area = match bound {
            Some(b) => b.area.to_device(scale_factor.factor),
            None => area.to_device(scale_factor.factor),
        };
        let scaled_position = position.to_device(scale_factor.factor);
        let text_section = visible_section
            .section
            .to_scaled(scale_factor.factor)
            .intersection(Section::<DeviceView>::new(scaled_position, bound_area));
        placer.reset_filtered();
        if let Some(section) = text_section {
            let mut filter_queue = HashSet::new();
            for (key, glyph) in placer.unfiltered_placement().iter() {
                let glyph_section = Section::<DeviceView>::new(
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
