use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{
    Added, Changed, Commands, Entity, EventReader, NonSend, NonSendMut, Or, Query,
    RemovedComponents, Res, ResMut,
};
use fontdue::layout::{GlyphPosition, LayoutSettings, TextStyle};
use tracing::trace;

use crate::gfx::GfxSurface;
use crate::instance::key::KeyFactory;
use crate::text::atlas::{
    AtlasAddQueue, AtlasGlyphReference, AtlasGlyphReferences, AtlasGlyphs, AtlasWriteQueue, Bitmap,
};
use crate::text::component::{
    Cache, Difference, FilteredPlacement, Glyph, GlyphId, Placement, Placer, TextGridLocation,
    TextGridPlacement, TextLetterDimensions, TextLineStructure, TextScale, TextValue,
    TextWrapStyle,
};
use crate::text::font::MonoSpacedFont;
use crate::text::render_group::{
    DrawSection, KeyedGlyphIds, LayerWrite, PositionWrite, RenderGroup, RenderGroupBindGroup,
    RenderGroupUniqueGlyphs,
};
use crate::text::renderer::{Extraction, TextRenderer};
use crate::texture_atlas::{
    AtlasBlock, AtlasDimension, AtlasLocation, AtlasPosition, TextureAtlas, TextureBindGroup,
    TextureCoordinates,
};
use crate::window::WindowResize;
use crate::{
    AlignedUniform, Area, Color, DeviceContext, Indexer, InstanceAttributeManager,
    InterfaceContext, Key, Layer, NullBit, NumericalContext, Position, ScaleFactor, Section,
    Uniform, Viewport, Visibility, VisibleSection,
};

pub(crate) fn setup(scale_factor: Res<ScaleFactor>, mut cmd: Commands) {
    cmd.insert_resource(Extraction::new());
    cmd.insert_resource(MonoSpacedFont::jet_brains_mono(
        MonoSpacedFont::DEFAULT_OPT_SCALE,
    ));
}
pub(crate) fn place(
    mut text_query: Query<
        (
            &mut Placer,
            &mut Placement,
            &TextValue,
            &Area<InterfaceContext>,
            &TextWrapStyle,
            &TextScale,
        ),
        Or<(
            Changed<Area<InterfaceContext>>,
            Changed<TextValue>,
            Changed<TextScale>,
        )>,
    >,
    fonts: Res<MonoSpacedFont>,
    scale_factor: Res<ScaleFactor>,
) {
    for (mut placer, mut placement, text, area, wrap_style, text_scale) in text_query.iter_mut() {
        let area = area.to_device(scale_factor.factor());
        placer.0.reset(&LayoutSettings {
            max_width: Some(area.width),
            max_height: Some(area.height),
            wrap_style: wrap_style.0,
            ..LayoutSettings::default()
        });
        placer.0.append(
            fonts.font_slice(),
            &TextStyle::new(text.0.as_str(), text_scale.px(), MonoSpacedFont::index()),
        );
        let mut key_factory = KeyFactory::new();
        placement.0 = placer
            .0
            .glyphs()
            .iter()
            .map(|g| (key_factory.generate(), *g))
            .collect::<Vec<(Key, GlyphPosition<()>)>>();
        for (key, glyph_position) in placement.0.iter_mut() {
            let base = if text_scale.0 > MonoSpacedFont::FACTOR_BASE_SCALE
                && text_scale.0 <= MonoSpacedFont::FACTOR_BASE_SCALE * 2
            {
                8.5f32
            } else if text_scale.0 > MonoSpacedFont::FACTOR_BASE_SCALE * 2 {
                11.5f32 * (text_scale.0 as f32 / 120f32).min(1.0)
            } else {
                0f32
            };
            if text_scale.0 > MonoSpacedFont::FACTOR_BASE_SCALE {
                let factor = text_scale.0 as f32 / MonoSpacedFont::FACTOR_BASE_SCALE as f32;
                glyph_position.y -= base * factor;
            }
        }
    }
}
pub(crate) fn letter_differential(
    mut text_query: Query<
        (
            &FilteredPlacement,
            &Visibility,
            &mut Cache,
            &mut Difference,
            &TextScale,
            &Color,
        ),
        Changed<FilteredPlacement>,
    >,
) {
    for (filtered_placement, visibility, mut cache, mut difference, text_scale, color) in
        text_query.iter_mut()
    {
        if visibility.visible() {
            let mut retained_keys = HashSet::new();
            let old_keys = cache.keys.clone();
            let mut keys_to_remove = HashSet::new();
            for (key, placed_glyph) in filtered_placement.0.iter() {
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
                let glyph = Glyph::new(character, *text_scale, glyph_id);
                if cache.exists(*key) {
                    retained_keys.insert(*key);
                    if cache.glyph_position_different(*key, glyph_position) {
                        difference.updated.insert(*key, glyph_position);
                        cache.glyph_position.insert(*key, glyph_position);
                    }
                    if cache.glyph_id_different(*key, glyph_id) {
                        difference.glyph_add.insert(*key, glyph);
                        cache.glyphs.insert(*key, glyph_id);
                    }
                } else {
                    difference.glyph_color_update.insert(*key, *color);
                    cache.glyph_color.insert(*key, *color);
                    difference.added.insert(*key, glyph_position);
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
pub(crate) fn filter(
    mut text_query: Query<
        (
            &Placement,
            &mut FilteredPlacement,
            &mut TextGridPlacement,
            &mut TextLineStructure,
            &VisibleSection,
            &Position<InterfaceContext>,
            &Area<InterfaceContext>,
            &TextLetterDimensions,
        ),
        Or<(Changed<TextValue>, Changed<VisibleSection>)>,
    >,
    scale_factor: Res<ScaleFactor>,
) {
    for (
        placement,
        mut filtered_placement,
        mut grid_placement,
        mut text_line_structure,
        visible_section,
        pos,
        area,
        text_letter_dimensions,
    ) in text_query.iter_mut()
    {
        if let Some(v_sec) = visible_section.section {
            filtered_placement.0 = placement.0.clone();
            grid_placement.0 = HashMap::new();
            let mut filter_queue = HashSet::new();
            let position = pos.to_device(scale_factor.factor());
            let v_sec = v_sec.to_device(scale_factor.factor());
            for (key, glyph_pos) in placement.0.iter() {
                if glyph_pos.parent.is_ascii_control() {
                    filter_queue.insert(*key);
                    continue;
                }
                let glyph_section = Section::<DeviceContext>::from((
                    (position.x + glyph_pos.x, position.y + glyph_pos.y),
                    (glyph_pos.width, glyph_pos.height),
                ));
                let grid_location = TextGridLocation::from_position(
                    glyph_section.position - position,
                    *text_letter_dimensions,
                );
                grid_placement.0.insert(grid_location, *key);
                if !v_sec.is_overlapping(glyph_section) {
                    filter_queue.insert(*key);
                }
            }
            filtered_placement
                .0
                .retain(|(key, _)| !filter_queue.contains(key));
            *text_line_structure = TextLineStructure::from_grid_placement(
                &grid_placement,
                area,
                text_letter_dimensions,
                scale_factor.factor(),
            );
        }
    }
}
pub(crate) fn scale_change(
    mut text_query: Query<(&mut TextLetterDimensions, &mut TextScale), Changed<TextScale>>,
    scale_factor: Res<ScaleFactor>,
    fonts: Res<MonoSpacedFont>,
) {
    for (mut text_letter_dimensions, mut text_scale) in text_query.iter_mut() {
        text_scale.0 = (text_scale.0 as f32 * scale_factor.factor()) as u32;
        let letter_dimensions = fonts.character_dimensions(text_scale.px());
        let letter_dimensions =
            Area::<DeviceContext>::from((letter_dimensions.width, letter_dimensions.height));
        *text_letter_dimensions = TextLetterDimensions(letter_dimensions);
    }
}
pub(crate) fn manage(
    mut text_query: Query<
        (
            Entity,
            &Visibility,
            &TextValue,
            &Position<InterfaceContext>,
            &VisibleSection,
            &Layer,
            &TextScale,
            &mut Cache,
            &mut Difference,
        ),
        Or<(Changed<Visibility>, Added<TextValue>, Changed<TextScale>)>,
    >,
    mut removed: RemovedComponents<TextValue>,
    mut extraction: ResMut<Extraction>,
    fonts: Res<MonoSpacedFont>,
) {
    for (
        entity,
        visibility,
        text,
        pos,
        visible_section,
        layer,
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
                    *text_scale,
                    AtlasBlock::new(fonts.character_dimensions(text_scale.px())),
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
        if let Some(entity_section) = visible_section.section {
            if let Some(cached_section) = cache.visible_section.section {
                if cached_section != entity_section {
                    difference.visible_section.replace(*visible_section);
                    cache.visible_section = *visible_section;
                }
            } else {
                cache.visible_section.section.replace(entity_section);
            }
        }
    }
}
pub(crate) fn color_diff(
    mut text_query: Query<(&Color, &mut Cache, &mut Difference), Changed<Color>>,
) {
    for (color, mut cache, mut difference) in text_query.iter_mut() {
        for key in cache.keys.clone().iter() {
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
    #[cfg(not(target_family = "wasm"))] mut renderer: ResMut<TextRenderer>,
    #[cfg(target_family = "wasm")] mut renderer: NonSendMut<TextRenderer>,
    #[cfg(not(target_family = "wasm"))] gfx_surface: Res<GfxSurface>,
    #[cfg(target_family = "wasm")] gfx_surface: NonSend<GfxSurface>,
    scale_factor: Res<ScaleFactor>,
) {
    for entity in extraction.removed.iter() {
        renderer.render_groups.remove(entity);
    }
    for (entity, (max, pos, visible_section, layer, unique_glyphs, text_scale, atlas_block)) in
        extraction.added.iter()
    {
        let position = pos.to_device(scale_factor.factor());
        let text_placement = AlignedUniform::new(
            &gfx_surface.device,
            Some([position.x, position.y, layer.z, 0.0]),
        );
        let render_group_bind_group = RenderGroupBindGroup::new(
            &gfx_surface,
            &renderer.render_group_bind_group_layout,
            &text_placement,
        );
        let atlas_dimension = AtlasDimension::from_unique_glyphs(unique_glyphs.unique_glyphs);
        let atlas = TextureAtlas::new(
            &gfx_surface,
            *atlas_block,
            atlas_dimension,
            wgpu::TextureFormat::R8Unorm,
        );
        let atlas_bind_group = TextureBindGroup::new(
            &gfx_surface,
            &renderer.atlas_bind_group_layout,
            atlas.view(),
        );
        renderer.render_groups.insert(
            *entity,
            RenderGroup {
                position,
                visible_section: *visible_section,
                layer: *layer,
                position_write: PositionWrite::new(),
                layer_write: LayerWrite::new(),
                keyed_glyph_ids: KeyedGlyphIds::new(),
                draw_section: DrawSection::new(),
                text_placement,
                unique_glyphs: *unique_glyphs,
                text_scale: *text_scale,
                indexer: Indexer::new(*max),
                glyph_positions: InstanceAttributeManager::new(&gfx_surface, *max),
                glyph_areas: InstanceAttributeManager::new(&gfx_surface, *max),
                glyph_colors: InstanceAttributeManager::new(&gfx_surface, *max),
                null_bits: InstanceAttributeManager::new(&gfx_surface, *max),
                glyph_tex_coords: InstanceAttributeManager::new(&gfx_surface, *max),
                render_group_bind_group,
                atlas,
                atlas_bind_group,
                atlas_glyph_references: AtlasGlyphReferences::new(),
                atlas_write_queue: AtlasWriteQueue::new(),
                atlas_add_queue: AtlasAddQueue::new(),
                atlas_glyphs: AtlasGlyphs::new(),
            },
        );
    }
}
pub(crate) fn render_group_differences(
    mut extraction: ResMut<Extraction>,
    #[cfg(not(target_family = "wasm"))] mut renderer: ResMut<TextRenderer>,
    #[cfg(target_family = "wasm")] mut renderer: NonSendMut<TextRenderer>,
    #[cfg(not(target_family = "wasm"))] gfx_surface: Res<GfxSurface>,
    #[cfg(target_family = "wasm")] gfx_surface: NonSend<GfxSurface>,
    font: Res<MonoSpacedFont>,
    #[cfg(not(target_family = "wasm"))] viewport: Res<Viewport>,
    #[cfg(target_family = "wasm")] viewport: NonSend<Viewport>,
    scale_factor: Res<ScaleFactor>,
) {
    for (entity, difference) in extraction.differences.iter() {
        trace!("processing differences for {:?}", entity);
        let mut render_group = renderer.render_groups.remove(entity).unwrap();
        let mut draw_section_resize_needed = false;
        if let Some(v_sec) = difference.visible_section {
            render_group.visible_section = v_sec;
            draw_section_resize_needed = true;
        }
        if let Some(position) = difference.position {
            render_group
                .position_write
                .write
                .replace(position.to_device(scale_factor.factor()));
            draw_section_resize_needed = true;
        }
        if let Some(layer) = difference.layer {
            render_group.layer_write.write.replace(layer);
        }
        for key in difference.remove.iter() {
            render_group.keyed_glyph_ids.ids.remove(key);
            let index = render_group.indexer.get_index(*key).unwrap();
            render_group.null_bits.queue_write(index, NullBit::null());
            render_group.indexer.remove(*key);
        }
        for (key, glyph_position) in difference.added.iter() {
            let index = render_group.indexer.next(*key);
            render_group
                .glyph_positions
                .queue_write(index, glyph_position.as_raw());
            render_group
                .null_bits
                .queue_write(index, NullBit::not_null());
        }
        for (key, color) in difference.glyph_color_update.iter() {
            let index = render_group.indexer.get_index(*key).unwrap();
            render_group.glyph_colors.queue_write(index, *color);
        }
        if render_group.indexer.should_grow() {
            let max = render_group.indexer.max();
            render_group.glyph_positions.grow(&gfx_surface, max);
            render_group.glyph_areas.grow(&gfx_surface, max);
            render_group.glyph_colors.grow(&gfx_surface, max);
            render_group.glyph_tex_coords.grow(&gfx_surface, max);
            render_group.null_bits.grow(&gfx_surface, max);
        }
        for (key, glyph_position) in difference.updated.iter() {
            let index = render_group.indexer.get_index(*key).unwrap();
            render_group
                .glyph_positions
                .queue_write(index, glyph_position.as_raw());
        }
        for (key, glyph) in difference.glyph_add.iter() {
            render_group.keyed_glyph_ids.ids.insert(*key, glyph.id);
            render_group.atlas_add_queue.queue.insert(glyph.clone());
        }
        let mut add_retained_glyphs = HashSet::new();
        for glyph in render_group.atlas_add_queue.queue.iter() {
            if render_group.atlas_glyphs.glyphs.contains_key(&glyph.id) {
                add_retained_glyphs.insert(glyph.clone());
            }
        }
        for glyph in add_retained_glyphs {
            render_group
                .atlas_glyph_references
                .references
                .get_mut(&glyph.id)
                .unwrap()
                .increment();
            render_group.atlas_add_queue.queue.remove(&glyph);
        }
        let mut orphaned_glyphs = HashSet::new();
        for (glyph_id, reference) in render_group.atlas_glyph_references.references.iter() {
            if reference.count == 0 {
                orphaned_glyphs.insert(*glyph_id);
            }
        }
        // retain filter
        // ...
        // free
        for glyph_id in orphaned_glyphs {
            let (_, _, location, _) = render_group.atlas_glyphs.glyphs.remove(&glyph_id).unwrap();
            render_group.atlas.free_locations.free.insert(location);
            render_group
                .atlas_glyph_references
                .references
                .remove(&glyph_id);
        }
        let adjusted_glyphs = {
            let mut adjusted_glyphs = HashSet::new();
            let num_new_glyphs = render_group.atlas_add_queue.queue.len() as u32;
            if num_new_glyphs != 0
                && num_new_glyphs > render_group.atlas.free_locations.free.len() as u32
            {
                let current_dimension = render_group.atlas.dimension.dimension;
                let current_total = current_dimension.pow(2);
                let mut incremental_dimension_add = 1;
                let mut next_size_up_total = (current_dimension + incremental_dimension_add).pow(2);
                while next_size_up_total - current_total < num_new_glyphs {
                    incremental_dimension_add += 1;
                    next_size_up_total = (current_dimension + incremental_dimension_add).pow(2);
                }
                let new_dimension =
                    AtlasDimension::new(current_dimension + incremental_dimension_add);
                render_group.atlas = TextureAtlas::new(
                    &gfx_surface,
                    render_group.atlas.block,
                    new_dimension,
                    wgpu::TextureFormat::R8Unorm,
                );
                render_group.atlas_bind_group = TextureBindGroup::new(
                    &gfx_surface,
                    &renderer.atlas_bind_group_layout,
                    render_group.atlas.view(),
                );
                let mut writes = Vec::<(
                    GlyphId,
                    AtlasLocation,
                    TextureCoordinates,
                    Area<NumericalContext>,
                    Bitmap,
                )>::new();
                for (glyph_id, (_, glyph_area, atlas_location, bitmap)) in
                    render_group.atlas_glyphs.glyphs.iter()
                {
                    let position = AtlasPosition::new(*atlas_location, render_group.atlas.block);
                    let glyph_section = Section::new(position.position, *glyph_area);
                    let coords = TextureCoordinates::from_section(
                        glyph_section,
                        render_group.atlas.texture_dimensions,
                    );
                    writes.push((
                        *glyph_id,
                        *atlas_location,
                        coords,
                        *glyph_area,
                        bitmap.clone(),
                    ));
                    render_group
                        .atlas
                        .free_locations
                        .free
                        .remove(atlas_location);
                    adjusted_glyphs.insert(*glyph_id);
                }
                for write in writes {
                    render_group
                        .atlas_glyphs
                        .glyphs
                        .get_mut(&write.0)
                        .unwrap()
                        .0 = write.2;
                    render_group
                        .atlas_write_queue
                        .queue
                        .insert(write.1, (write.2, write.3, write.4));
                }
            }
            adjusted_glyphs
        };
        let add_queue = render_group
            .atlas_add_queue
            .queue
            .drain()
            .collect::<HashSet<Glyph>>();
        for add in add_queue {
            render_group
                .atlas_glyph_references
                .references
                .insert(add.id, AtlasGlyphReference::new());
            render_group
                .atlas_glyph_references
                .references
                .get_mut(&add.id)
                .unwrap()
                .increment();
            let rasterization = font.font().rasterize(add.character, add.scale.px());
            // TODO since subpixel , combine them here to save space
            let glyph_area: Area<NumericalContext> =
                (rasterization.0.width, rasterization.0.height).into();
            let location = render_group.atlas.free_locations.next();
            let position = AtlasPosition::new(location, render_group.atlas.block);
            let glyph_section = Section::new(position.position, glyph_area);
            let coords = TextureCoordinates::from_section(
                glyph_section,
                render_group.atlas.texture_dimensions,
            );
            render_group
                .atlas_write_queue
                .queue
                .insert(location, (coords, glyph_area, rasterization.1.clone()));
            render_group
                .atlas_glyphs
                .glyphs
                .insert(add.id, (coords, glyph_area, location, rasterization.1));
        }
        let mut glyph_info_writes = HashSet::<(Key, GlyphId)>::new();
        for adj_glyph in adjusted_glyphs {
            for (key, glyph_id) in render_group.keyed_glyph_ids.ids.iter() {
                if adj_glyph == *glyph_id {
                    glyph_info_writes.insert((*key, *glyph_id));
                }
            }
        }
        for (key, glyph_id) in glyph_info_writes {
            let (coords, area, _, _) = render_group
                .atlas_glyphs
                .glyphs
                .get(&glyph_id)
                .unwrap()
                .clone();
            let index = render_group.indexer.get_index(key).unwrap();
            render_group.glyph_areas.queue_write(index, area.as_raw());
            render_group.glyph_tex_coords.queue_write(index, coords);
        }
        for (key, glyph) in difference.glyph_add.iter() {
            let (coords, area, _, _) = render_group
                .atlas_glyphs
                .glyphs
                .get(&glyph.id)
                .unwrap()
                .clone();
            let index = render_group.indexer.get_index(*key).unwrap();
            render_group.glyph_areas.queue_write(index, area.as_raw());
            render_group.glyph_tex_coords.queue_write(index, coords);
        }
        render_group.glyph_positions.write(&gfx_surface);
        render_group.glyph_areas.write(&gfx_surface);
        render_group.glyph_colors.write(&gfx_surface);
        render_group.glyph_tex_coords.write(&gfx_surface);
        render_group.null_bits.write(&gfx_surface);
        let mut dirty = false;
        if let Some(position) = render_group.position_write.write.take() {
            render_group.position = position;
            render_group.text_placement.set_aspect(0, position.x);
            render_group.text_placement.set_aspect(1, position.y);
            dirty = true;
        }
        if let Some(layer) = render_group.layer_write.write.take() {
            render_group.text_placement.set_aspect(2, layer.z);
            dirty = true;
        }
        if dirty {
            render_group.text_placement.update(&gfx_surface.queue);
        }
        for (location, (_, glyph_area, bitmap)) in render_group.atlas_write_queue.queue.iter() {
            render_group
                .atlas
                .write::<u8>(*location, bitmap.as_slice(), *glyph_area, &gfx_surface);
        }
        if draw_section_resize_needed {
            if let Some(v_sec) = render_group.visible_section.section {
                let v_sec = v_sec.to_device(scale_factor.factor());
                let draw_bound = Section::<DeviceContext>::new(
                    v_sec.position - viewport.as_section().position,
                    v_sec.area,
                );
                render_group.draw_section.section.replace(draw_bound);
            }
        }
        renderer.render_groups.insert(*entity, render_group);
    }
    *extraction = Extraction::new();
}
pub(crate) fn resolve_draw_section_on_resize(
    #[cfg(not(target_family = "wasm"))] mut renderer: ResMut<TextRenderer>,
    #[cfg(target_family = "wasm")] mut renderer: NonSendMut<TextRenderer>,
    mut event_reader: EventReader<WindowResize>,
    #[cfg(not(target_family = "wasm"))] viewport: Res<Viewport>,
    #[cfg(target_family = "wasm")] viewport: NonSend<Viewport>,
    scale_factor: Res<ScaleFactor>,
) {
    for _event in event_reader.iter() {
        for (_, render_group) in renderer.render_groups.iter_mut() {
            if let Some(v_sec) = render_group.visible_section.section {
                let v_sec = v_sec.to_device(scale_factor.factor());
                let draw_bound = Section::<DeviceContext>::new(
                    v_sec.position - viewport.as_section().position,
                    v_sec.area,
                );
                render_group.draw_section.section.replace(draw_bound);
            }
        }
    }
}
