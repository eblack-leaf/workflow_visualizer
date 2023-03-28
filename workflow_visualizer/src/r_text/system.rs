use std::collections::HashSet;
use crate::r_text::atlas::{Atlas, AtlasAddQueue, AtlasBindGroup, AtlasBlock, AtlasDimension, AtlasFreeLocations, AtlasGlyphReferences, AtlasGlyphs, AtlasTextureDimensions, AtlasWriteQueue};
use crate::r_text::component::{
    Cache, Difference, FilteredPlacement, Placement, Placer, Text, TextGridPlacement,
    TextLetterDimensions, TextScale, TextScaleAlignment,
};
use crate::r_text::font::AlignedFonts;
use crate::r_text::render_group::{DrawSection, KeyedGlyphIds, LayerWrite, PositionWrite, RenderGroup, RenderGroupBindGroup, RenderGroupUniqueGlyphs, TextPlacement};
use crate::r_text::renderer::{Extraction, TextRenderer};
use crate::{Area, Color, DeviceContext, Indexer, InstanceAttributeManager, InterfaceContext, Layer, Position, ScaleFactor, Section, Uniform, Viewport, Visibility, VisibleSection};
use bevy_ecs::prelude::{Added, Changed, Entity, Or, Query, RemovedComponents, Res, ResMut};
use crate::gfx::GfxSurface;

pub(crate) fn place(
    mut text_query: Query<
        (&mut Placer, &mut Placement, &Text),
        Or<(Changed<Area<InterfaceContext>>, Changed<Text>)>,
    >,
) {
}
pub(crate) fn filter(
    mut text_query: Query<
        (&Placement, &mut FilteredPlacement, &mut TextGridPlacement, &VisibleSection, &Position<InterfaceContext>, &TextLetterDimensions),
        Or<(
            Changed<Text>,
            Changed<VisibleSection>,
        )>,
    >,
) {
    for (placement, mut filtered_placement, mut grid_placement, visible_section) in text_query.iter_mut(){
        if let Some(v_sec) = visible_section.section {
            filtered_placement.0 = placement.0.clone();
            let mut filter_queue = HashSet::new();
            for (key, glyph_pos) in placement.0.iter() {
                let glyph_section = Section::<InterfaceContext>::from(((glyph_pos.x, glyph_pos.y), (glyph_pos.width, glyph_pos.height)));
                let grid_location = ();
                grid_placement.0.insert(grid_location, *key);
                if !v_sec.is_overlapping(glyph_section) {
                    filter_queue.insert(*key);
                }
            }
            filtered_placement.0.retain(|(key, _)| !filter_queue.contains(key));
        }
    }
}
pub(crate) fn scale_change(
    mut text_query: Query<
        (
            &mut TextScale,
            &mut TextLetterDimensions,
            &TextScaleAlignment,
        ),
        Changed<TextScaleAlignment>,
    >,
    scale_factor: Res<ScaleFactor>,
    fonts: Res<AlignedFonts>,
) {
    for (mut text_scale, mut text_letter_dimensions, text_scale_alignment) in text_query.iter_mut() {
        *text_scale = TextScale::from_alignment(*text_scale_alignment, scale_factor.factor);
        let letter_dimensions = fonts.fonts.get(text_scale_alignment).unwrap().character_dimensions('a', text_scale.px());
        let letter_dimensions = Area::<InterfaceContext>::from(
            (letter_dimensions.width, letter_dimensions.height)
        );
        *text_letter_dimensions = TextLetterDimensions(letter_dimensions);
    }
}
pub(crate) fn manage(
    mut text_query: Query<
        (
            Entity,
            &Visibility,
            &Text,
            &Position<InterfaceContext>,
            &VisibleSection,
            &Layer,
            &TextScaleAlignment,
            &TextScale,
            &mut Cache,
            &mut Difference,
        ),
        Or<(Changed<Visibility>, Added<Text>, Changed<TextScale>)>,
    >,
    mut removed: RemovedComponents<Text>,
    mut extraction: ResMut<Extraction>,
    fonts: Res<AlignedFonts>,
) {
    for (
        entity,
        visibility,
        text,
        pos,
        visible_section,
        layer,
        text_scale_alignment,
        text_scale,
        mut cache,
        mut difference,
    ) in text_query.iter_mut()
    {
        if visibility.visible() {
            *cache = Cache::new();
            *difference = Difference::new();
            difference.position.replace(*pos);
            difference.layer.replace(*layer);
            difference.visible_section.replace(*visible_section);
            extraction.added.insert(
                entity,
                (
                    text.0.len() as u32,
                    *pos,
                    *visible_section,
                    *layer,
                    RenderGroupUniqueGlyphs::from_text(text),
                    *text_scale_alignment,
                    AtlasBlock::new(fonts.fonts.get(text_scale_alignment).unwrap(), text_scale),
                ),
            );
        } else {
            extraction.removed.insert(entity);
        }
    }
    for entity in removed.iter() {
        extraction.removed.insert(entity);
    }
}
pub(crate) fn letter_cache_check() {}
pub(crate) fn position_diff(
    mut text_query: Query<
        (&Position<InterfaceContext>, &mut Cache, &mut Difference),
        Changed<Position<InterfaceContext>>,
    >,
) {
    for (pos, mut cache, mut difference) in text_query.iter_mut() {
        if *pos != cache.position {
            difference.position.replace(*pos);
            cache.position = *pos;
        }
    }
}
pub(crate) fn visible_section_diff(
    mut text_query: Query<(&VisibleSection, &mut Cache, &mut Difference), Changed<VisibleSection>>,
) {
    for (visible_section, mut cache, mut difference) in text_query.iter_mut() {
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
pub(crate) fn color_diff(
    mut text_query: Query<(&Color, &mut Cache, &mut Difference), Changed<Color>>,
) {
    for (color, mut cache, mut difference) in text_query.iter_mut() {
        for key in cache.keys.iter() {
            difference.glyph_color_update.insert(*key, *color);
            cache.glyph_color.insert(*key, *color);
        }
    }
}
pub(crate) fn layer_diff(
    mut text_query: Query<(&Layer, &mut Cache, &mut Difference), Changed<Layer>>,
) {
    for (layer, mut cache, mut difference) in text_query.iter_mut() {
        if *layer != cache.layer {
            difference.layer.replace(*layer);
            cache.layer = *layer;
        }
    }
}
pub(crate) fn pull_differences(
    mut text_query: Query<(Entity, &mut Difference, &Visibility), Changed<Difference>>,
    mut extraction: ResMut<Extraction>,
) {
    for (entity, mut difference, visibility) in text_query.iter_mut() {
        if visibility.visible() {
            extraction.differences.insert(entity, difference.clone());
            *difference = Difference::new();
        }
    }
}
pub(crate) fn create_render_groups(
    extraction: Res<Extraction>,
    mut renderer: ResMut<TextRenderer>,
    gfx_surface: Res<GfxSurface>,
    scale_factor: Res<ScaleFactor>,
) {
    for entity in extraction.removed.iter() {
        renderer.render_groups.remove(entity);
    }
    for (entity, (max, pos, visible_section, layer, unique_glyphs, text_scale_alignment, atlas_block)) in extraction.added.iter() {
        let position = pos.to_device(scale_factor.factor);
        let text_placement = TextPlacement::new(position, *layer);
        let text_placement_uniform = Uniform::new(&gfx_surface.device, text_placement);
        let render_group_bind_group = RenderGroupBindGroup::new(&gfx_surface, &renderer.render_group_bind_group_layout, &text_placement_uniform);
        let atlas_dimension = AtlasDimension::from_unique_glyphs(unique_glyphs.unique_glyphs);
        let atlas_texture_dimensions = AtlasTextureDimensions::new(*atlas_block, atlas_dimension);
        let atlas = Atlas::new(&gfx_surface, atlas_texture_dimensions);
        let atlas_bind_group = AtlasBindGroup::new(&gfx_surface, &renderer.atlas_bind_group_layout, &atlas);
        renderer.render_groups.insert(*entity, RenderGroup{
            position,
            visible_section: *visible_section,
            layer: *layer,
            position_write: PositionWrite::new(),
            layer_write: LayerWrite::new(),
            keyed_glyph_ids: KeyedGlyphIds::new(),
            draw_section: DrawSection::new(),
            text_placement,
            text_placement_uniform,
            unique_glyphs: *unique_glyphs,
            text_scale_alignment: *text_scale_alignment,
            indexer: Indexer::new(*max),
            glyph_positions: InstanceAttributeManager::new(&gfx_surface, *max),
            glyph_areas: InstanceAttributeManager::new(&gfx_surface, *max),
            glyph_colors: InstanceAttributeManager::new(&gfx_surface, *max),
            null_bits: InstanceAttributeManager::new(&gfx_surface, *max),
            glyph_tex_coords: InstanceAttributeManager::new(&gfx_surface, *max),
            render_group_bind_group,
            atlas,
            atlas_bind_group,
            atlas_texture_dimensions,
            atlas_dimension,
            atlas_free_locations: AtlasFreeLocations::new(atlas_dimension),
            atlas_glyph_references: AtlasGlyphReferences::new(),
            atlas_write_queue: AtlasWriteQueue::new(),
            atlas_add_queue: AtlasAddQueue::new(),
            atlas_glyphs: AtlasGlyphs::new(),
            atlas_block: *atlas_block
        });
    }
}
pub(crate) fn render_group_differences(
    extraction: Res<Extraction>,
    mut renderer: ResMut<TextRenderer>,
    gfx_surface: Res<GfxSurface>,
    font: Res<AlignedFonts>,
    viewport: Res<Viewport>,
    scale_factor: Res<ScaleFactor>,
) {
    for (entity, difference) in extraction.differences.iter() {

    }
}
pub(crate) fn resolve_draw_section_on_resize() {}
