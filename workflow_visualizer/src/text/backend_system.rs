use std::collections::HashSet;
use std::num::NonZeroU32;

use bevy_ecs::prelude::{Entity, EventReader, Res, ResMut};
use bytemuck::{Pod, Zeroable};

use crate::coord::{DeviceContext, NumericalContext};
use crate::gfx::GfxSurface;
use crate::instance::index::{Index, Indexer};
use crate::instance::key::Key;
use crate::instance::GpuAttributeBuffer;
use crate::instance::NullBit;
use crate::instance::{offset, AttributeWrite, CpuAttributeBuffer};
use crate::text::atlas::{
    Atlas, AtlasAddQueue, AtlasBindGroup, AtlasBlock, AtlasDimension, AtlasFreeLocations,
    AtlasGlyphReference, AtlasGlyphReferences, AtlasGlyphs, AtlasLocation, AtlasPosition,
    AtlasTextureDimensions, AtlasWriteQueue, Bitmap,
};
use crate::text::coords::Coords;
use crate::text::difference::{Difference, TextBoundDifference};
use crate::text::extraction::Extraction;
use crate::text::glyph::{Glyph, GlyphId};
use crate::text::render_group::{
    DepthWrite, DrawSection, KeyedGlyphIds, PositionWrite, RenderGroup, RenderGroupBindGroup,
    RenderGroupTextBound, TextPlacement,
};
use crate::text::renderer::TextRenderer;
use crate::text::scale::{AlignedFonts, TextScaleAlignment};
use crate::uniform::Uniform;
use crate::visibility::VisibleSection;
use crate::window::WindowResize;
use crate::{Area, Color, Position, RawArea, RawPosition, ScaleFactor, Section, Viewport};

pub(crate) fn create_render_groups(
    extraction: Res<Extraction>,
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
        (max, position, visible_section, depth, atlas_block, unique_glyphs, text_scale_alignment),
    ) in extraction.added_render_groups.iter()
    {
        let position = position.to_device(scale_factor.factor);
        let text_placement = TextPlacement::new(position, *depth);
        let text_placement_uniform = Uniform::new(&gfx_surface.device, text_placement);
        let render_group_bind_group = RenderGroupBindGroup::new(
            &gfx_surface,
            &renderer.render_group_bind_group_layout,
            &text_placement_uniform,
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
        let null_cpu = CpuAttributeBuffer::<NullBit>::new(max.0);
        let coords_cpu = CpuAttributeBuffer::<Coords>::new(max.0);
        let glyph_position_cpu = CpuAttributeBuffer::<RawPosition>::new(max.0);
        let glyph_area_cpu = CpuAttributeBuffer::<RawArea>::new(max.0);
        let null_gpu = GpuAttributeBuffer::<NullBit>::new(&gfx_surface, max.0, "null bit buffer");
        let coords_gpu = GpuAttributeBuffer::<Coords>::new(&gfx_surface, max.0, "coords buffer");
        let glyph_position_gpu =
            GpuAttributeBuffer::<RawPosition>::new(&gfx_surface, max.0, "glyph position buffer");
        let glyph_area_gpu =
            GpuAttributeBuffer::<RawArea>::new(&gfx_surface, max.0, "glyph area buffer");
        let null_write = AttributeWrite::<NullBit>::new();
        let coords_write = AttributeWrite::<Coords>::new();
        let glyph_position_write = AttributeWrite::<RawPosition>::new();
        let glyph_area_write = AttributeWrite::<RawArea>::new();
        let position_write = PositionWrite::new();
        let depth_write = DepthWrite::new();
        let keyed_glyph_ids = KeyedGlyphIds::new();
        let draw_section = DrawSection::new();
        let glyph_color_write = AttributeWrite::<Color>::new();
        let glyph_color_cpu = CpuAttributeBuffer::<Color>::new(max.0);
        let glyph_color_gpu =
            GpuAttributeBuffer::<Color>::new(&gfx_surface, max.0, "glyph color buffer");
        let render_group_entity = renderer
            .container
            .spawn(RenderGroup::new(
                *max,
                position,
                *visible_section,
                *depth,
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
                keyed_glyph_ids,
                draw_section,
                atlas_texture_dimensions,
                atlas_dimension,
                atlas_free_locations,
                atlas_glyph_references,
                atlas_write_queue,
                atlas_add_queue,
                atlas_glyphs,
                text_placement,
                text_placement_uniform,
                glyph_color_write,
                glyph_color_cpu,
                glyph_color_gpu,
            ))
            .id();
        let old = renderer.render_groups.insert(*entity, render_group_entity);
        if let Some(old_render_group) = old {
            renderer.container.despawn(old_render_group);
        }
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
        let render_group = *renderer
            .render_groups
            .get(entity)
            .expect("no render group for entity");
        let mut draw_section_resize_needed = false;
        set_text_bound(
            &mut renderer,
            difference,
            render_group,
            &mut draw_section_resize_needed,
        );
        set_visible_section(
            &mut renderer,
            difference,
            render_group,
            &mut draw_section_resize_needed,
        );
        queue_position(
            &mut renderer,
            &scale_factor,
            difference,
            render_group,
            &mut draw_section_resize_needed,
        );
        queue_depth(&mut renderer, difference, render_group);
        queue_remove(&mut renderer, difference, render_group);
        queue_add(&mut renderer, difference, render_group);
        queue_glyph_color(&mut renderer, difference, render_group);
        grow_attributes(&mut renderer, &gfx_surface, render_group);
        update_glyph_positions(&mut renderer, difference, render_group);
        resolve_glyphs(&mut renderer, difference, render_group);
        resolve_atlas(&mut renderer, render_group);
        let adjusted_glyphs = grow_atlas(&mut renderer, &gfx_surface, render_group);
        rasterize_add_queue(&mut renderer, &font, render_group);
        update_adjusted_glyphs(&mut renderer, render_group, adjusted_glyphs);
        read_added_glyphs(&mut renderer, difference, render_group);
        write_attribute::<RawPosition>(&mut renderer, &gfx_surface, render_group);
        write_attribute::<RawArea>(&mut renderer, &gfx_surface, render_group);
        write_attribute::<NullBit>(&mut renderer, &gfx_surface, render_group);
        write_attribute::<Coords>(&mut renderer, &gfx_surface, render_group);
        write_attribute::<Color>(&mut renderer, &gfx_surface, render_group);
        write_text_placement(&mut renderer, &gfx_surface, render_group);
        write_atlas(&mut renderer, &gfx_surface, render_group);
        if draw_section_resize_needed {
            resolve_draw_section(&mut renderer, &viewport, &scale_factor, render_group)
        }
    }
}

fn queue_glyph_color(renderer: &mut TextRenderer, difference: &Difference, render_group: Entity) {
    for (change, color) in difference.glyph_color_change.iter() {
        let index = renderer
            .container
            .get::<Indexer<Key>>(render_group)
            .unwrap()
            .get_index(*change)
            .unwrap();
        renderer
            .container
            .get_mut::<AttributeWrite<Color>>(render_group)
            .unwrap()
            .write
            .insert(index, *color);
    }
}

fn set_text_bound(
    renderer: &mut TextRenderer,
    difference: &Difference,
    render_group: Entity,
    draw_section_resize_needed: &mut bool,
) {
    if let Some(diff) = difference.bounds {
        *draw_section_resize_needed = true;
        match diff {
            TextBoundDifference::Changed(bound) => {
                renderer
                    .container
                    .get_mut::<RenderGroupTextBound>(render_group)
                    .expect("no render group text bound")
                    .text_bound_area
                    .replace(bound.area);
            }
            TextBoundDifference::Removed => {
                renderer
                    .container
                    .get_mut::<RenderGroupTextBound>(render_group)
                    .expect("no render group text bound")
                    .text_bound_area
                    .take();
            }
        }
    }
}

fn set_visible_section(
    renderer: &mut TextRenderer,
    difference: &Difference,
    render_group: Entity,
    draw_section_resize_needed: &mut bool,
) {
    if let Some(visible_section) = difference.visible_section {
        *renderer
            .container
            .get_mut::<VisibleSection>(render_group)
            .expect("no render group for entity") = visible_section;
        *draw_section_resize_needed = true;
    }
}

fn resolve_draw_section(
    renderer: &mut TextRenderer,
    viewport: &Viewport,
    scale_factor: &ScaleFactor,
    render_group: Entity,
) {
    if let Some(bound) = renderer
        .container
        .get::<RenderGroupTextBound>(render_group)
        .unwrap()
        .text_bound_area
    {
        let position = *renderer
            .container
            .get::<Position<DeviceContext>>(render_group)
            .unwrap();
        let scaled_bound =
            Section::<DeviceContext>::new(position, bound.to_device(scale_factor.factor));
        let visible_section = renderer
            .container
            .get::<VisibleSection>(render_group)
            .unwrap();
        let visible_bound = visible_section
            .section
            .unwrap()
            .to_device(scale_factor.factor)
            .intersection(scaled_bound);
        if let Some(v_bound) = visible_bound {
            let viewport_dimensions = v_bound.intersection(viewport.as_section());
            if let Some(v_dim) = viewport_dimensions {
                let draw_bound = Section::<DeviceContext>::new(
                    v_dim.position - viewport.as_section().position,
                    v_dim.area,
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
    } else {
        renderer
            .container
            .get_mut::<DrawSection>(render_group)
            .unwrap()
            .section
            .take();
    }
}

fn queue_position(
    renderer: &mut TextRenderer,
    scale_factor: &ScaleFactor,
    difference: &Difference,
    render_group: Entity,
    draw_section_resize_needed: &mut bool,
) {
    if let Some(position) = difference.position {
        *draw_section_resize_needed = true;
        let scaled_position = position.to_device(scale_factor.factor);
        renderer
            .container
            .get_mut::<PositionWrite>(render_group)
            .unwrap()
            .write
            .replace(scaled_position);
    }
}

fn queue_depth(renderer: &mut TextRenderer, difference: &Difference, render_group: Entity) {
    if let Some(depth) = difference.depth {
        renderer
            .container
            .get_mut::<DepthWrite>(render_group)
            .unwrap()
            .write
            .replace(depth);
    }
}

fn queue_remove(renderer: &mut TextRenderer, difference: &Difference, render_group: Entity) {
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
            .get_mut::<AttributeWrite<NullBit>>(render_group)
            .unwrap()
            .write
            .insert(index, NullBit::null());
        let _ = renderer
            .container
            .get_mut::<Indexer<Key>>(render_group)
            .unwrap()
            .remove(*key);
    }
}

fn queue_add(renderer: &mut TextRenderer, difference: &Difference, render_group: Entity) {
    for (key, glyph_position) in difference.add.iter() {
        let index = renderer
            .container
            .get_mut::<Indexer<Key>>(render_group)
            .unwrap()
            .next(*key);
        renderer
            .container
            .get_mut::<AttributeWrite<RawPosition>>(render_group)
            .unwrap()
            .write
            .insert(index, glyph_position.as_raw());
        renderer
            .container
            .get_mut::<AttributeWrite<NullBit>>(render_group)
            .unwrap()
            .write
            .insert(index, NullBit::not_null());
    }
}

fn grow_attributes(renderer: &mut TextRenderer, gfx_surface: &GfxSurface, render_group: Entity) {
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
            .get_mut::<CpuAttributeBuffer<RawPosition>>(render_group)
            .unwrap()
            .buffer
            .resize(max as usize, RawPosition::default());
        *renderer
            .container
            .get_mut::<GpuAttributeBuffer<RawPosition>>(render_group)
            .unwrap() =
            GpuAttributeBuffer::<RawPosition>::new(&gfx_surface, max, "glyph position buffer");
        gfx_surface.queue.write_buffer(
            &renderer
                .container
                .get::<GpuAttributeBuffer<RawPosition>>(render_group)
                .unwrap()
                .buffer,
            0,
            bytemuck::cast_slice(
                &renderer
                    .container
                    .get::<CpuAttributeBuffer<RawPosition>>(render_group)
                    .unwrap()
                    .buffer,
            ),
        );
        renderer
            .container
            .get_mut::<CpuAttributeBuffer<RawArea>>(render_group)
            .unwrap()
            .buffer
            .resize(max as usize, RawArea::default());
        *renderer
            .container
            .get_mut::<GpuAttributeBuffer<RawArea>>(render_group)
            .unwrap() = GpuAttributeBuffer::<RawArea>::new(&gfx_surface, max, "glyph area buffer");
        gfx_surface.queue.write_buffer(
            &renderer
                .container
                .get::<GpuAttributeBuffer<RawArea>>(render_group)
                .unwrap()
                .buffer,
            0,
            bytemuck::cast_slice(
                &renderer
                    .container
                    .get::<CpuAttributeBuffer<RawArea>>(render_group)
                    .unwrap()
                    .buffer,
            ),
        );
        renderer
            .container
            .get_mut::<CpuAttributeBuffer<NullBit>>(render_group)
            .unwrap()
            .buffer
            .resize(max as usize, NullBit::default());
        *renderer
            .container
            .get_mut::<GpuAttributeBuffer<NullBit>>(render_group)
            .unwrap() = GpuAttributeBuffer::<NullBit>::new(&gfx_surface, max, "null bit buffer");
        gfx_surface.queue.write_buffer(
            &renderer
                .container
                .get::<GpuAttributeBuffer<NullBit>>(render_group)
                .unwrap()
                .buffer,
            0,
            bytemuck::cast_slice(
                &renderer
                    .container
                    .get::<CpuAttributeBuffer<NullBit>>(render_group)
                    .unwrap()
                    .buffer,
            ),
        );
        renderer
            .container
            .get_mut::<CpuAttributeBuffer<Coords>>(render_group)
            .unwrap()
            .buffer
            .resize(max as usize, Coords::default());
        *renderer
            .container
            .get_mut::<GpuAttributeBuffer<Coords>>(render_group)
            .unwrap() = GpuAttributeBuffer::<Coords>::new(&gfx_surface, max, "coords buffer");
        gfx_surface.queue.write_buffer(
            &renderer
                .container
                .get::<GpuAttributeBuffer<Coords>>(render_group)
                .unwrap()
                .buffer,
            0,
            bytemuck::cast_slice(
                &renderer
                    .container
                    .get::<CpuAttributeBuffer<Coords>>(render_group)
                    .unwrap()
                    .buffer,
            ),
        );
        renderer
            .container
            .get_mut::<CpuAttributeBuffer<Color>>(render_group)
            .unwrap()
            .buffer
            .resize(max as usize, Color::default());
        *renderer
            .container
            .get_mut::<GpuAttributeBuffer<Color>>(render_group)
            .unwrap() = GpuAttributeBuffer::<Color>::new(&gfx_surface, max, "color buffer");
        gfx_surface.queue.write_buffer(
            &renderer
                .container
                .get::<GpuAttributeBuffer<Color>>(render_group)
                .unwrap()
                .buffer,
            0,
            bytemuck::cast_slice(
                &renderer
                    .container
                    .get::<CpuAttributeBuffer<Color>>(render_group)
                    .unwrap()
                    .buffer,
            ),
        );
    }
}

fn update_glyph_positions(
    renderer: &mut TextRenderer,
    difference: &Difference,
    render_group: Entity,
) {
    for (key, glyph_position) in difference.update.iter() {
        let index = renderer
            .container
            .get::<Indexer<Key>>(render_group)
            .unwrap()
            .get_index(*key)
            .expect("no index for key");
        renderer
            .container
            .get_mut::<AttributeWrite<RawPosition>>(render_group)
            .unwrap()
            .write
            .insert(index, glyph_position.as_raw());
    }
}

fn resolve_glyphs(renderer: &mut TextRenderer, difference: &Difference, render_group: Entity) {
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
}

fn resolve_atlas(renderer: &mut TextRenderer, render_group: Entity) {
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
}

fn grow_atlas(
    renderer: &mut TextRenderer,
    gfx_surface: &GfxSurface,
    render_group: Entity,
) -> HashSet<GlyphId> {
    let mut adjusted_glyphs = HashSet::new();
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
        let atlas_bind_group =
            AtlasBindGroup::new(&gfx_surface, &renderer.atlas_bind_group_layout, &atlas);
        *renderer
            .container
            .get_mut::<AtlasBindGroup>(render_group)
            .unwrap() = atlas_bind_group;
        let mut free_locations = AtlasFreeLocations::new(new_dimension);
        let mut writes = Vec::<(
            GlyphId,
            AtlasLocation,
            Coords,
            Area<NumericalContext>,
            Bitmap,
        )>::new();
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
    }
    adjusted_glyphs
}

fn rasterize_add_queue(renderer: &mut TextRenderer, font: &AlignedFonts, render_group: Entity) {
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
        // TODO since subpixel , combine them here to save space
        let glyph_area: Area<NumericalContext> =
            (rasterization.0.width, rasterization.0.height).into();
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
}

fn update_adjusted_glyphs(
    renderer: &mut TextRenderer,
    render_group: Entity,
    adjusted_glyphs: HashSet<GlyphId>,
) {
    let mut glyph_info_writes = HashSet::<(Key, GlyphId)>::new();
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
            .get_mut::<AttributeWrite<RawArea>>(render_group)
            .unwrap()
            .write
            .insert(index, area.as_raw());
        renderer
            .container
            .get_mut::<AttributeWrite<Coords>>(render_group)
            .unwrap()
            .write
            .insert(index, coords);
    }
}

fn read_added_glyphs(renderer: &mut TextRenderer, difference: &Difference, render_group: Entity) {
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
            .get_mut::<AttributeWrite<RawArea>>(render_group)
            .unwrap()
            .write
            .insert(index, area.as_raw());
        renderer
            .container
            .get_mut::<AttributeWrite<Coords>>(render_group)
            .unwrap()
            .write
            .insert(index, coords);
    }
}

fn write_attribute<Attribute: Send + Sync + Default + Clone + Pod + Zeroable + 'static>(
    renderer: &mut TextRenderer,
    gfx_surface: &GfxSurface,
    render_group: Entity,
) {
    let attributes = renderer
        .container
        .get_mut::<AttributeWrite<Attribute>>(render_group)
        .unwrap()
        .write
        .drain()
        .collect::<Vec<(Index, Attribute)>>();
    let mut write_range: (Option<Index>, Option<Index>) = (None, None);
    for (index, attr) in attributes {
        *renderer
            .container
            .get_mut::<CpuAttributeBuffer<Attribute>>(render_group)
            .unwrap()
            .buffer
            .get_mut(index.value as usize)
            .unwrap() = attr;
        if let Some(start) = write_range.0.as_mut() {
            if index.value < start.value {
                *start = index;
            }
        } else {
            write_range.0.replace(index);
        }
        if let Some(end) = write_range.1.as_mut() {
            if index.value > end.value {
                *end = index;
            }
        } else {
            write_range.1.replace(index);
        }
    }
    if let Some(start) = write_range.0 {
        let end = write_range.1.take().unwrap();
        let cpu_range = &renderer
            .container
            .get::<CpuAttributeBuffer<Attribute>>(render_group)
            .unwrap()
            .buffer[start.value as usize..end.value as usize + 1];
        let offset = offset::<Attribute>(&start);
        gfx_surface.queue.write_buffer(
            &renderer
                .container
                .get::<GpuAttributeBuffer<Attribute>>(render_group)
                .unwrap()
                .buffer,
            offset,
            bytemuck::cast_slice(cpu_range),
        );
    }
}

fn write_text_placement(
    renderer: &mut TextRenderer,
    gfx_surface: &GfxSurface,
    render_group: Entity,
) {
    let mut dirty = false;
    if let Some(position) = renderer
        .container
        .get_mut::<PositionWrite>(render_group)
        .unwrap()
        .write
        .take()
    {
        *renderer
            .container
            .get_mut::<Position<DeviceContext>>(render_group)
            .unwrap() = position;
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
    if let Some(layer) = renderer
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
            .placement[2] = layer.z;
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
}

fn write_atlas(renderer: &mut TextRenderer, gfx_surface: &GfxSurface, render_group: Entity) {
    for (location, (_, glyph_area, bitmap)) in renderer
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
        let extent_w = glyph_area.width as u32;
        let extent_h = glyph_area.height as u32;
        let image_data_layout = wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: NonZeroU32::new(extent_w),
            rows_per_image: NonZeroU32::new(extent_h),
        };
        let extent = wgpu::Extent3d {
            width: extent_w,
            height: extent_h,
            depth_or_array_layers: 1,
        };
        gfx_surface.queue.write_texture(
            image_copy_texture,
            bitmap.as_slice(),
            image_data_layout,
            extent,
        );
    }
}

pub(crate) fn resize_receiver(
    mut renderer: ResMut<TextRenderer>,
    mut event_reader: EventReader<WindowResize>,
    viewport: Res<Viewport>,
    scale_factor: Res<ScaleFactor>,
) {
    for _event in event_reader.iter() {
        let render_groups = renderer.render_groups.clone();
        for (_, render_group) in render_groups {
            resolve_draw_section(&mut renderer, &viewport, &scale_factor, render_group);
        }
    }
}

pub(crate) fn reset_extraction(mut extraction: ResMut<Extraction>) {
    *extraction = Extraction::new();
}
