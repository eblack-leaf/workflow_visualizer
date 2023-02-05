use std::collections::{HashMap, HashSet};
use std::num::NonZeroU32;

use bevy_ecs::prelude::{Added, Commands, Entity, EventReader, Res, ResMut};

use crate::ecs_text::atlas::{
    Atlas, AtlasAddQueue, AtlasBindGroup, AtlasBlock, AtlasDimension, AtlasFreeLocations,
    AtlasGlyphReference, AtlasGlyphReferences, AtlasGlyphs, AtlasLocation, AtlasPosition,
    AtlasTextureDimensions, AtlasWriteQueue, Bitmap, GlyphArea,
};
use crate::ecs_text::coords::Coords;
use crate::ecs_text::cpu_buffer::CpuBuffer;
use crate::ecs_text::extraction::Extraction;
use crate::ecs_text::glyph::{Glyph, GlyphId, Key};
use crate::ecs_text::gpu_buffer::GpuBuffer;
use crate::ecs_text::index::{Index, Indexer};
use crate::ecs_text::null_bit::NullBit;
use crate::ecs_text::render_group::{
    ColorWrite, CoordsWrite, DepthWrite, DrawSection, GlyphAreaWrite, GlyphPositionWrite,
    KeyedGlyphIds, NullWrite, PositionWrite, RenderGroup, RenderGroupBindGroup, RenderGroupMax,
    RenderGroupTextBound, RenderGroupUniqueGlyphs, TextBound, TextPlacement,
};
use crate::ecs_text::renderer::TextRenderer;
use crate::ecs_text::scale::{AlignedFonts, TextScaleAlignment};
use crate::gfx::GfxSurface;
use crate::uniform::Uniform;
use crate::viewport::Viewport;
use crate::visibility::VisibleSection;
use crate::window::{Resize, ScaleFactor};
use crate::{Area, Color, Position, ScaledSection, Section};

pub(crate) fn create_render_groups(
    mut extraction: ResMut<Extraction>,
    mut renderer: ResMut<TextRenderer>,
    gfx_surface: Res<GfxSurface>,
    scale_factor: Res<ScaleFactor>,
) {
    for entity in extraction.removed_render_groups.iter() {
        let render_group_entity = *renderer
            .render_groups
            .get(entity)
            .expect("no render group entity");
        renderer.container.despawn(render_group_entity);
        renderer.render_groups.remove(entity);
    }
    for (
        entity,
        (
            max,
            position,
            visible_section,
            depth,
            color,
            atlas_block,
            unique_glyphs,
            text_scale_alignment,
        ),
    ) in extraction.added_render_groups.iter()
    {
        let position = position.to_scaled(scale_factor.factor);
        let text_placement = TextPlacement::new(position, *depth);
        let text_placement_uniform = Uniform::new(&gfx_surface.device, text_placement);
        let color_uniform = Uniform::new(&gfx_surface.device, *color);
        let render_group_bind_group = RenderGroupBindGroup::new(
            &gfx_surface,
            &renderer.render_group_bind_group_layout,
            &text_placement_uniform,
            &color_uniform,
        );
        let indexer = Indexer::<Key>::new(max.0);
        let atlas_dimension = AtlasDimension::from_unique_glyphs(*unique_glyphs);
        let atlas_texture_dimensions = AtlasTextureDimensions::new(*atlas_block, atlas_dimension);
        let atlas = Atlas::new(&gfx_surface, atlas_texture_dimensions);
        let atlas_bind_group =
            AtlasBindGroup::new(&gfx_surface, &renderer.atlas_bind_group_layout, &atlas);
        let atlas_free_locations = AtlasFreeLocations::new(atlas_dimension);
        let atlas_glyphs = AtlasGlyphs::new();
        let atlas_write_queue = AtlasWriteQueue::new();
        let atlas_add_queue = AtlasAddQueue::new();
        let atlas_glyph_references = AtlasGlyphReferences::new();
        let text_bound = RenderGroupTextBound::new();
        let null_cpu = CpuBuffer::<NullBit>::new(max.0);
        let coords_cpu = CpuBuffer::<Coords>::new(max.0);
        let glyph_position_cpu = CpuBuffer::<Position>::new(max.0);
        let glyph_area_cpu = CpuBuffer::<Area>::new(max.0);
        let null_gpu = GpuBuffer::<NullBit>::new(&gfx_surface, max.0, "null bit buffer");
        let coords_gpu = GpuBuffer::<Coords>::new(&gfx_surface, max.0, "coords buffer");
        let glyph_position_gpu =
            GpuBuffer::<Position>::new(&gfx_surface, max.0, "glyph position buffer");
        let glyph_area_gpu = GpuBuffer::<Area>::new(&gfx_surface, max.0, "glyph area buffer");
        let null_write = NullWrite::new();
        let coords_write = CoordsWrite::new();
        let glyph_position_write = GlyphPositionWrite::new();
        let glyph_area_write = GlyphAreaWrite::new();
        let position_write = PositionWrite::new();
        let depth_write = DepthWrite::new();
        let color_write = ColorWrite::new();
        let keyed_glyph_ids = KeyedGlyphIds::new();
        let draw_section = DrawSection::new();
        let render_group_entity = renderer
            .container
            .spawn(RenderGroup::new(
                *max,
                position,
                *visible_section,
                *depth,
                *color,
                *atlas_block,
                *unique_glyphs,
                *text_scale_alignment,
                render_group_bind_group,
                indexer,
                atlas,
                atlas_bind_group,
                text_bound,
                null_cpu,
                coords_cpu,
                glyph_position_cpu,
                glyph_area_cpu,
                null_gpu,
                coords_gpu,
                glyph_position_gpu,
                glyph_area_gpu,
                null_write,
                coords_write,
                glyph_position_write,
                glyph_area_write,
                position_write,
                depth_write,
                color_write,
                keyed_glyph_ids,
                draw_section,
                atlas_texture_dimensions,
                atlas_dimension,
                atlas_free_locations,
                atlas_glyph_references,
                atlas_write_queue,
                atlas_add_queue,
                atlas_glyphs,
            ))
            .id();
        renderer.render_groups.insert(*entity, render_group_entity);
    }
}

pub(crate) fn render_group_differences(
    mut extraction: ResMut<Extraction>,
    mut renderer: ResMut<TextRenderer>,
    gfx_surface: Res<GfxSurface>,
    font: Res<AlignedFonts>,
    viewport: Res<Viewport>,
    scale_factor: Res<ScaleFactor>,
) {
    for (entity, difference) in extraction.differences.iter() {
        let render_group = *renderer
            .render_groups
            .get(entity)
            .expect("no render group for entity");
        let mut draw_section_resize_needed = false;
        if let Some(bound) = difference.bounds {
            renderer
                .container
                .get_mut::<RenderGroupTextBound>(render_group)
                .expect("no render group text bound")
                .text_bound_section
                .replace(bound);
            draw_section_resize_needed = true;
        }
        if let Some(visible_section) = difference.visible_section {
            *renderer
                .container
                .get_mut::<VisibleSection>(render_group)
                .expect("no render group for entity") = visible_section;
            draw_section_resize_needed = true;
        }
        if draw_section_resize_needed {
            if let Some(bound) = renderer
                .container
                .get::<RenderGroupTextBound>(render_group)
                .unwrap()
                .text_bound_section
            {
                let scaled_bound = bound.to_scaled(scale_factor.factor);
                let visible_section = renderer
                    .container
                    .get::<VisibleSection>(render_group)
                    .unwrap();
                let visible_bound = visible_section
                    .section
                    .to_scaled(scale_factor.factor)
                    .intersection(scaled_bound);
                if let Some(v_bound) = visible_bound {
                    let draw_bound = ScaledSection::new(
                        v_bound.position - viewport.as_section().position,
                        v_bound.area,
                    );
                    renderer
                        .container
                        .get_mut::<DrawSection>(render_group)
                        .unwrap()
                        .section
                        .replace(draw_bound);
                } else {
                    renderer
                        .container
                        .get_mut::<DrawSection>(render_group)
                        .unwrap()
                        .section
                        .take();
                }
            } else {
                renderer
                    .container
                    .get_mut::<DrawSection>(render_group)
                    .unwrap()
                    .section
                    .take();
            }
        }
        if let Some(position) = difference.position {
            renderer
                .container
                .get_mut::<PositionWrite>(render_group)
                .unwrap()
                .write
                .replace(position.to_scaled(scale_factor.factor));
        }
        if let Some(depth) = difference.depth {
            renderer
                .container
                .get_mut::<DepthWrite>(render_group)
                .unwrap()
                .write
                .replace(depth);
        }
        if let Some(color) = difference.color {
            renderer
                .container
                .get_mut::<ColorWrite>(render_group)
                .unwrap()
                .write
                .replace(color);
        }
        for key in difference.remove.iter() {
            renderer
                .container
                .get_mut::<KeyedGlyphIds>(render_group)
                .unwrap()
                .ids
                .remove(key);
            let index = renderer
                .container
                .get::<Indexer<Key>>(render_group)
                .unwrap()
                .get_index(*key)
                .expect("no index for key");
            renderer
                .container
                .get_mut::<NullWrite>(render_group)
                .unwrap()
                .write
                .insert(index, NullBit::null());
            let _ = renderer
                .container
                .get_mut::<Indexer<Key>>(render_group)
                .unwrap()
                .remove(*key);
        }
        for (key, glyph_position) in difference.add.iter() {
            let index = renderer
                .container
                .get_mut::<Indexer<Key>>(render_group)
                .unwrap()
                .next(*key);
            renderer
                .container
                .get_mut::<GlyphPositionWrite>(render_group)
                .unwrap()
                .write
                .insert(index, *glyph_position);
            renderer
                .container
                .get_mut::<NullWrite>(render_group)
                .unwrap()
                .write
                .insert(index, NullBit::not_null());
        }
        if renderer
            .container
            .get_mut::<Indexer<Key>>(render_group)
            .unwrap()
            .should_grow()
        {
            let max = renderer
                .container
                .get::<Indexer<Key>>(render_group)
                .unwrap()
                .max();
            renderer
                .container
                .get_mut::<CpuBuffer<Position>>(render_group)
                .unwrap()
                .buffer
                .resize(max as usize, Position::default());
            *renderer
                .container
                .get_mut::<GpuBuffer<Position>>(render_group)
                .unwrap() = GpuBuffer::<Position>::new(&gfx_surface, max, "glyph position buffer");
            gfx_surface.queue.write_buffer(
                &renderer
                    .container
                    .get::<GpuBuffer<Position>>(render_group)
                    .unwrap()
                    .buffer,
                0,
                bytemuck::cast_slice(
                    &renderer
                        .container
                        .get::<CpuBuffer<Position>>(render_group)
                        .unwrap()
                        .buffer,
                ),
            );
            renderer
                .container
                .get_mut::<CpuBuffer<GlyphArea>>(render_group)
                .unwrap()
                .buffer
                .resize(max as usize, GlyphArea::default());
            *renderer
                .container
                .get_mut::<GpuBuffer<GlyphArea>>(render_group)
                .unwrap() = GpuBuffer::<GlyphArea>::new(&gfx_surface, max, "glyph area buffer");
            gfx_surface.queue.write_buffer(
                &renderer
                    .container
                    .get::<GpuBuffer<GlyphArea>>(render_group)
                    .unwrap()
                    .buffer,
                0,
                bytemuck::cast_slice(
                    &renderer
                        .container
                        .get::<CpuBuffer<GlyphArea>>(render_group)
                        .unwrap()
                        .buffer,
                ),
            );
            renderer
                .container
                .get_mut::<CpuBuffer<NullBit>>(render_group)
                .unwrap()
                .buffer
                .resize(max as usize, NullBit::default());
            *renderer
                .container
                .get_mut::<GpuBuffer<NullBit>>(render_group)
                .unwrap() = GpuBuffer::<NullBit>::new(&gfx_surface, max, "null bit buffer");
            gfx_surface.queue.write_buffer(
                &renderer
                    .container
                    .get::<GpuBuffer<NullBit>>(render_group)
                    .unwrap()
                    .buffer,
                0,
                bytemuck::cast_slice(
                    &renderer
                        .container
                        .get::<CpuBuffer<NullBit>>(render_group)
                        .unwrap()
                        .buffer,
                ),
            );
            renderer
                .container
                .get_mut::<CpuBuffer<Coords>>(render_group)
                .unwrap()
                .buffer
                .resize(max as usize, Coords::default());
            *renderer
                .container
                .get_mut::<GpuBuffer<Coords>>(render_group)
                .unwrap() = GpuBuffer::<Coords>::new(&gfx_surface, max, "coords buffer");
            gfx_surface.queue.write_buffer(
                &renderer
                    .container
                    .get::<GpuBuffer<Coords>>(render_group)
                    .unwrap()
                    .buffer,
                0,
                bytemuck::cast_slice(
                    &renderer
                        .container
                        .get::<CpuBuffer<Coords>>(render_group)
                        .unwrap()
                        .buffer,
                ),
            );
        }
        for (key, glyph_position) in difference.update.iter() {
            let index = renderer
                .container
                .get::<Indexer<Key>>(render_group)
                .unwrap()
                .get_index(*key)
                .expect("no index for key");
            renderer
                .container
                .get_mut::<GlyphPositionWrite>(render_group)
                .unwrap()
                .write
                .insert(index, *glyph_position);
        }
        for glyph_id in difference.glyph_remove.iter() {
            renderer
                .container
                .get_mut::<AtlasGlyphReferences>(render_group)
                .unwrap()
                .references
                .get_mut(glyph_id)
                .unwrap()
                .decrement();
        }
        for (key, glyph) in difference.glyph_add.iter() {
            renderer
                .container
                .get_mut::<KeyedGlyphIds>(render_group)
                .unwrap()
                .ids
                .insert(*key, glyph.id);
            renderer
                .container
                .get_mut::<AtlasAddQueue>(render_group)
                .unwrap()
                .queue
                .insert(glyph.clone());
        }
        // prepare atlas
        let mut add_retained_glyphs = HashSet::new();
        for glyph in renderer
            .container
            .get::<AtlasAddQueue>(render_group)
            .unwrap()
            .queue
            .iter()
        {
            if renderer
                .container
                .get::<AtlasGlyphs>(render_group)
                .unwrap()
                .glyphs
                .contains_key(&glyph.id)
            {
                add_retained_glyphs.insert(glyph.clone());
            }
        }
        for glyph in add_retained_glyphs {
            renderer
                .container
                .get_mut::<AtlasGlyphReferences>(render_group)
                .unwrap()
                .references
                .get_mut(&glyph.id)
                .unwrap()
                .increment();
            renderer
                .container
                .get_mut::<AtlasAddQueue>(render_group)
                .unwrap()
                .queue
                .remove(&glyph);
        }
        let mut orphaned_glyphs = HashSet::new();
        for (glyph_id, reference) in renderer
            .container
            .get::<AtlasGlyphReferences>(render_group)
            .unwrap()
            .references
            .iter()
        {
            if reference.count == 0 {
                orphaned_glyphs.insert(*glyph_id);
            }
        }
        // retain filter
        // ...
        // free
        for glyph_id in orphaned_glyphs {
            let (_, _, location, _) = renderer
                .container
                .get_mut::<AtlasGlyphs>(render_group)
                .unwrap()
                .glyphs
                .remove(&glyph_id)
                .unwrap();
            renderer
                .container
                .get_mut::<AtlasFreeLocations>(render_group)
                .unwrap()
                .free
                .insert(location);
            renderer
                .container
                .get_mut::<AtlasGlyphReferences>(render_group)
                .unwrap()
                .references
                .remove(&glyph_id);
        }
        let num_new_glyphs = renderer
            .container
            .get::<AtlasAddQueue>(render_group)
            .unwrap()
            .queue
            .len() as u32;
        if num_new_glyphs != 0
            && num_new_glyphs
                > renderer
                    .container
                    .get::<AtlasFreeLocations>(render_group)
                    .unwrap()
                    .free
                    .len() as u32
        {
            // grow
            let current_dimension = renderer
                .container
                .get::<AtlasDimension>(render_group)
                .unwrap()
                .dimension;
            let current_total = current_dimension.pow(2);
            let mut incremental_dimension_add = 1;
            let mut next_size_up_total = (current_dimension + incremental_dimension_add).pow(2);
            while next_size_up_total - current_total < num_new_glyphs {
                incremental_dimension_add += 1;
                next_size_up_total = (current_dimension + incremental_dimension_add).pow(2);
            }
            let new_dimension = AtlasDimension::new(current_dimension + incremental_dimension_add);
            let texture_dimensions = AtlasTextureDimensions::new(
                *renderer.container.get::<AtlasBlock>(render_group).unwrap(),
                new_dimension,
            );
            let atlas = Atlas::new(&gfx_surface, texture_dimensions);
            let mut free_locations = AtlasFreeLocations::new(new_dimension);
            let mut writes = Vec::<(GlyphId, AtlasLocation, Coords, Area, Bitmap)>::new();
            let mut adjusted_glyphs = HashSet::new();
            for (glyph_id, (_, glyph_area, atlas_location, bitmap)) in renderer
                .container
                .get::<AtlasGlyphs>(render_group)
                .unwrap()
                .glyphs
                .iter()
            {
                let position = AtlasPosition::new(
                    *atlas_location,
                    *renderer.container.get::<AtlasBlock>(render_group).unwrap(),
                );
                let glyph_section = Section::new(position.position, *glyph_area);
                let coords = Coords::from_section(glyph_section, texture_dimensions);
                writes.push((
                    *glyph_id,
                    *atlas_location,
                    coords,
                    *glyph_area,
                    bitmap.clone(),
                ));
                free_locations.free.remove(atlas_location);
                adjusted_glyphs.insert(*glyph_id);
            }
            for write in writes {
                renderer
                    .container
                    .get_mut::<AtlasGlyphs>(render_group)
                    .unwrap()
                    .glyphs
                    .get_mut(&write.0)
                    .unwrap()
                    .0 = write.2;
                renderer
                    .container
                    .get_mut::<AtlasWriteQueue>(render_group)
                    .unwrap()
                    .queue
                    .insert(write.1, (write.2, write.3, write.4));
            }
            renderer.container.entity_mut(render_group).insert((
                atlas,
                texture_dimensions,
                free_locations,
                new_dimension,
            ));
            let add_queue = renderer
                .container
                .get_mut::<AtlasAddQueue>(render_group)
                .unwrap()
                .queue
                .drain()
                .collect::<HashSet<Glyph>>();
            for add in add_queue {
                renderer
                    .container
                    .get_mut::<AtlasGlyphReferences>(render_group)
                    .unwrap()
                    .references
                    .insert(add.id, AtlasGlyphReference::new());
                renderer
                    .container
                    .get_mut::<AtlasGlyphReferences>(render_group)
                    .unwrap()
                    .references
                    .get_mut(&add.id)
                    .unwrap()
                    .increment();
                let rasterization = font
                    .fonts
                    .get(
                        renderer
                            .container
                            .get::<TextScaleAlignment>(render_group)
                            .unwrap(),
                    )
                    .unwrap()
                    .font()
                    .rasterize(add.character, add.scale.px());
                let glyph_area = (rasterization.0.width, rasterization.0.height).into();
                let location = renderer
                    .container
                    .get_mut::<AtlasFreeLocations>(render_group)
                    .unwrap()
                    .next();
                let position = AtlasPosition::new(
                    location,
                    *renderer.container.get::<AtlasBlock>(render_group).unwrap(),
                );
                let glyph_section = Section::new(position.position, glyph_area);
                let coords = Coords::from_section(
                    glyph_section,
                    *renderer
                        .container
                        .get::<AtlasTextureDimensions>(render_group)
                        .unwrap(),
                );
                renderer
                    .container
                    .get_mut::<AtlasWriteQueue>(render_group)
                    .unwrap()
                    .queue
                    .insert(location, (coords, glyph_area, rasterization.1.clone()));
                renderer
                    .container
                    .get_mut::<AtlasGlyphs>(render_group)
                    .unwrap()
                    .glyphs
                    .insert(add.id, (coords, glyph_area, location, rasterization.1));
            }
            let mut glyph_info_writes = HashSet::<(Key, GlyphId)>::new();
            if !adjusted_glyphs.is_empty() {
                let atlas_bind_group = AtlasBindGroup::new(
                    &gfx_surface,
                    &renderer.atlas_bind_group_layout,
                    renderer.container.get::<Atlas>(render_group).unwrap(),
                );
                *renderer
                    .container
                    .get_mut::<AtlasBindGroup>(render_group)
                    .unwrap() = atlas_bind_group;
            }

            for adj_glyph in adjusted_glyphs {
                for (key, glyph_id) in renderer
                    .container
                    .get::<KeyedGlyphIds>(render_group)
                    .unwrap()
                    .ids
                    .iter()
                {
                    if adj_glyph == *glyph_id {
                        glyph_info_writes.insert((*key, *glyph_id));
                    }
                }
            }
            for (key, glyph_id) in glyph_info_writes {
                let (coords, area, _, _) = renderer
                    .container
                    .get::<AtlasGlyphs>(render_group)
                    .unwrap()
                    .glyphs
                    .get(&glyph_id)
                    .unwrap()
                    .clone();
                let index = renderer
                    .container
                    .get::<Indexer<Key>>(render_group)
                    .unwrap()
                    .get_index(key)
                    .unwrap();
                renderer
                    .container
                    .get_mut::<GlyphAreaWrite>(render_group)
                    .unwrap()
                    .write
                    .insert(index, area);
                renderer
                    .container
                    .get_mut::<CoordsWrite>(render_group)
                    .unwrap()
                    .write
                    .insert(index, coords);
            }
        }
        for (key, glyph) in difference.glyph_add.iter() {
            let (coords, area, _, _) = renderer
                .container
                .get::<AtlasGlyphs>(render_group)
                .unwrap()
                .glyphs
                .get(&glyph.id)
                .unwrap()
                .clone();
            let index = renderer
                .container
                .get::<Indexer<Key>>(render_group)
                .unwrap()
                .get_index(*key)
                .unwrap();
            renderer
                .container
                .get_mut::<GlyphAreaWrite>(render_group)
                .unwrap()
                .write
                .insert(index, area);
            renderer
                .container
                .get_mut::<CoordsWrite>(render_group)
                .unwrap()
                .write
                .insert(index, coords);
        }
        let glyph_positions = renderer
            .container
            .get_mut::<GlyphPositionWrite>(render_group)
            .unwrap()
            .write
            .drain()
            .collect::<Vec<(Index, Position)>>();
        for (index, position) in glyph_positions {
            renderer
                .container
                .get_mut::<CpuBuffer<Position>>(render_group)
                .unwrap()
                .buffer
                .insert(index.value as usize, position);
            let offset = offset::<Position>(&index);
            gfx_surface.queue.write_buffer(
                &renderer
                    .container
                    .get::<GpuBuffer<Position>>(render_group)
                    .unwrap()
                    .buffer,
                offset,
                bytemuck::cast_slice(&[position]),
            );
        }
        let glyph_areas = renderer
            .container
            .get_mut::<GlyphAreaWrite>(render_group)
            .unwrap()
            .write
            .drain()
            .collect::<Vec<(Index, Area)>>();
        for (index, area) in glyph_areas {
            renderer
                .container
                .get_mut::<CpuBuffer<Area>>(render_group)
                .unwrap()
                .buffer
                .insert(index.value as usize, area);
            let offset = offset::<Area>(&index);
            gfx_surface.queue.write_buffer(
                &renderer
                    .container
                    .get::<GpuBuffer<Area>>(render_group)
                    .unwrap()
                    .buffer,
                offset,
                bytemuck::cast_slice(&[area]),
            );
        }
        let nulls = renderer
            .container
            .get_mut::<NullWrite>(render_group)
            .unwrap()
            .write
            .drain()
            .collect::<Vec<(Index, NullBit)>>();
        for (index, null) in nulls {
            renderer
                .container
                .get_mut::<CpuBuffer<NullBit>>(render_group)
                .unwrap()
                .buffer
                .insert(index.value as usize, null);
            let offset = offset::<NullBit>(&index);
            gfx_surface.queue.write_buffer(
                &renderer
                    .container
                    .get::<GpuBuffer<NullBit>>(render_group)
                    .unwrap()
                    .buffer,
                offset,
                bytemuck::cast_slice(&[null]),
            );
        }
        let coords = renderer
            .container
            .get_mut::<CoordsWrite>(render_group)
            .unwrap()
            .write
            .drain()
            .collect::<Vec<(Index, Coords)>>();
        for (index, coords) in coords {
            renderer
                .container
                .get_mut::<CpuBuffer<Coords>>(render_group)
                .unwrap()
                .buffer
                .insert(index.value as usize, coords);
            let offset = offset::<Coords>(&index);
            gfx_surface.queue.write_buffer(
                &renderer
                    .container
                    .get::<GpuBuffer<Coords>>(render_group)
                    .unwrap()
                    .buffer,
                offset,
                bytemuck::cast_slice(&[coords]),
            );
        }
        let mut dirty = false;
        if let Some(position) = renderer
            .container
            .get_mut::<PositionWrite>(render_group)
            .unwrap()
            .write
            .take()
        {
            renderer
                .container
                .get_mut::<TextPlacement>(render_group)
                .unwrap()
                .placement[0] = position.x;
            renderer
                .container
                .get_mut::<TextPlacement>(render_group)
                .unwrap()
                .placement[1] = position.y;
            dirty = true;
        }
        if let Some(depth) = renderer
            .container
            .get_mut::<DepthWrite>(render_group)
            .unwrap()
            .write
            .take()
        {
            renderer
                .container
                .get_mut::<TextPlacement>(render_group)
                .unwrap()
                .placement[2] = depth.layer;
            dirty = true;
        }
        if dirty {
            let text_placement = *renderer
                .container
                .get::<TextPlacement>(render_group)
                .unwrap();
            renderer
                .container
                .get_mut::<Uniform<TextPlacement>>(render_group)
                .unwrap()
                .update(&gfx_surface.queue, text_placement);
        }
        if let Some(color) = renderer
            .container
            .get_mut::<ColorWrite>(render_group)
            .unwrap()
            .write
            .take()
        {
            renderer
                .container
                .get_mut::<Uniform<Color>>(render_group)
                .unwrap()
                .update(&gfx_surface.queue, color);
        }
        for (location, (coords, glyph_area, bitmap)) in renderer
            .container
            .get::<AtlasWriteQueue>(render_group)
            .unwrap()
            .queue
            .iter()
        {
            let atlas = renderer.container.get::<Atlas>(render_group).unwrap();
            let atlas_block = renderer.container.get::<AtlasBlock>(render_group).unwrap();
            let position = AtlasPosition::new(*location, *atlas_block).position;
            let image_copy_texture = wgpu::ImageCopyTexture {
                texture: &atlas.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: position.x as u32,
                    y: position.y as u32,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            };
            let image_data_layout = wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(glyph_area.width as u32),
                rows_per_image: NonZeroU32::new(glyph_area.height as u32),
            };
            let extent = wgpu::Extent3d {
                width: glyph_area.width as u32,
                height: glyph_area.height as u32,
                depth_or_array_layers: 1,
            };
            gfx_surface.queue.write_texture(
                image_copy_texture,
                bitmap.as_slice(),
                image_data_layout,
                extent,
            );
        }
        renderer
            .container
            .get_mut::<GlyphPositionWrite>(render_group)
            .unwrap()
            .write
            .clear();
        renderer
            .container
            .get_mut::<NullWrite>(render_group)
            .unwrap()
            .write
            .clear();
        renderer
            .container
            .get_mut::<CoordsWrite>(render_group)
            .unwrap()
            .write
            .clear();
        renderer
            .container
            .get_mut::<GlyphAreaWrite>(render_group)
            .unwrap()
            .write
            .clear();
    }
}

fn offset<T>(index: &Index) -> wgpu::BufferAddress {
    (std::mem::size_of::<T>() * index.value as usize) as wgpu::BufferAddress
}

pub(crate) fn resize_receiver(
    mut renderer: ResMut<TextRenderer>,
    mut event_reader: EventReader<Resize>,
    viewport: Res<Viewport>,
) {
    for event in event_reader.iter() {
        let render_groups = renderer.render_groups.clone();
        for (_, render_group) in render_groups {
            if let Some(bound) = renderer
                .container
                .get::<RenderGroupTextBound>(render_group)
                .unwrap()
                .text_bound_section
            {
                let scaled_bound = bound.to_scaled(event.scale_factor);
                let visible_section = *renderer
                    .container
                    .get::<VisibleSection>(render_group)
                    .unwrap();
                let visible_bound = visible_section
                    .section
                    .to_scaled(event.scale_factor)
                    .intersection(scaled_bound);
                if let Some(v_bound) = visible_bound {
                    let draw_bound = ScaledSection::new(
                        v_bound.position - viewport.as_section().position,
                        v_bound.area,
                    );
                    renderer
                        .container
                        .get_mut::<DrawSection>(render_group)
                        .unwrap()
                        .section
                        .replace(draw_bound);
                } else {
                    renderer
                        .container
                        .get_mut::<DrawSection>(render_group)
                        .unwrap()
                        .section
                        .take();
                }
            } else {
                renderer
                    .container
                    .get_mut::<DrawSection>(render_group)
                    .unwrap()
                    .section
                    .take();
            }
        }
    }
}

pub(crate) fn reset_extraction(mut extraction: ResMut<Extraction>) {
    *extraction = Extraction::new();
}
