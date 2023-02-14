use std::collections::HashMap;

use bevy_ecs::prelude::{Bundle, Component};

use crate::{Area, Color, Depth, Position, Section};
use crate::coord::{Device, GpuArea, GpuPosition, View};
use crate::gfx::GfxSurface;
use crate::text::atlas::{
    Atlas, AtlasAddQueue, AtlasBindGroup, AtlasBlock, AtlasDimension, AtlasFreeLocations,
    AtlasGlyphReferences, AtlasGlyphs, AtlasTextureDimensions, AtlasWriteQueue,
};
use crate::text::coords::Coords;
use crate::text::cpu_buffer::CpuBuffer;
use crate::text::glyph::{GlyphId, Key};
use crate::text::gpu_buffer::GpuBuffer;
use crate::text::index::{Index, Indexer};
use crate::text::null_bit::NullBit;
use crate::text::scale::TextScaleAlignment;
use crate::text::text::Text;
use crate::uniform::Uniform;
use crate::visibility::VisibleSection;

#[derive(Component, Copy, Clone)]
pub(crate) struct RenderGroupMax(pub(crate) u32);

#[derive(Component, Copy, Clone)]
pub(crate) struct RenderGroupUniqueGlyphs {
    pub(crate) unique_glyphs: u32,
}

impl RenderGroupUniqueGlyphs {
    pub(crate) fn from_text(text: &Text) -> Self {
        let length = text.length();
        Self {
            unique_glyphs: length.min(1024),
        }
    }
}

#[derive(Component)]
pub(crate) struct AttributeWrite<Attribute> {
    pub(crate) write: HashMap<Index, Attribute>,
}

impl<Attribute> AttributeWrite<Attribute> {
    pub(crate) fn new() -> Self {
        Self {
            write: HashMap::new(),
        }
    }
}

#[derive(Bundle)]
pub(crate) struct RenderGroup {
    pub(crate) max: RenderGroupMax,
    pub(crate) position: Position<Device>,
    pub(crate) visible_section: VisibleSection,
    pub(crate) depth: Depth,
    pub(crate) atlas_block: AtlasBlock,
    pub(crate) unique_glyphs: RenderGroupUniqueGlyphs,
    pub(crate) text_scale_alignment: TextScaleAlignment,
    pub(crate) render_group_bind_group: RenderGroupBindGroup,
    pub(crate) indexer: Indexer<Key>,
    pub(crate) atlas: Atlas,
    pub(crate) atlas_bind_group: AtlasBindGroup,
    pub(crate) text_bound: RenderGroupTextBound,
    pub(crate) null_cpu: CpuBuffer<NullBit>,
    pub(crate) coords_cpu: CpuBuffer<Coords>,
    pub(crate) glyph_position_cpu: CpuBuffer<GpuPosition>,
    pub(crate) glyph_area_cpu: CpuBuffer<GpuArea>,
    pub(crate) null_gpu: GpuBuffer<NullBit>,
    pub(crate) coords_gpu: GpuBuffer<Coords>,
    pub(crate) glyph_position_gpu: GpuBuffer<GpuPosition>,
    pub(crate) glyph_area_gpu: GpuBuffer<GpuArea>,
    pub(crate) null_write: AttributeWrite<NullBit>,
    pub(crate) coords_write: AttributeWrite<Coords>,
    pub(crate) glyph_position_write: AttributeWrite<GpuPosition>,
    pub(crate) glyph_area_write: AttributeWrite<GpuArea>,
    pub(crate) position_write: PositionWrite,
    pub(crate) depth_write: DepthWrite,
    pub(crate) keyed_glyph_ids: KeyedGlyphIds,
    pub(crate) draw_section: DrawSection,
    pub(crate) atlas_texture_dimensions: AtlasTextureDimensions,
    pub(crate) atlas_dimension: AtlasDimension,
    pub(crate) atlas_free_locations: AtlasFreeLocations,
    pub(crate) atlas_glyph_references: AtlasGlyphReferences,
    pub(crate) atlas_write_queue: AtlasWriteQueue,
    pub(crate) atlas_add_queue: AtlasAddQueue,
    pub(crate) atlas_glyphs: AtlasGlyphs,
    pub(crate) text_placement: TextPlacement,
    pub(crate) text_placement_uniform: Uniform<TextPlacement>,
    pub(crate) glyph_color_write: AttributeWrite<Color>,
    pub(crate) glyph_color_cpu: CpuBuffer<Color>,
    pub(crate) glyph_color_gpu: GpuBuffer<Color>,
}

impl RenderGroup {
    pub(crate) fn new(
        max: RenderGroupMax,
        position: Position<Device>,
        visible_section: VisibleSection,
        depth: Depth,
        atlas_block: AtlasBlock,
        unique_glyphs: RenderGroupUniqueGlyphs,
        text_scale_alignment: TextScaleAlignment,
        render_group_bind_group: RenderGroupBindGroup,
        indexer: Indexer<Key>,
        atlas: Atlas,
        atlas_bind_group: AtlasBindGroup,
        text_bound: RenderGroupTextBound,
        null_cpu: CpuBuffer<NullBit>,
        coords_cpu: CpuBuffer<Coords>,
        glyph_position_cpu: CpuBuffer<GpuPosition>,
        glyph_area_cpu: CpuBuffer<GpuArea>,
        null_gpu: GpuBuffer<NullBit>,
        coords_gpu: GpuBuffer<Coords>,
        glyph_position_gpu: GpuBuffer<GpuPosition>,
        glyph_area_gpu: GpuBuffer<GpuArea>,
        null_write: AttributeWrite<NullBit>,
        coords_write: AttributeWrite<Coords>,
        glyph_position_write: AttributeWrite<GpuPosition>,
        glyph_area_write: AttributeWrite<GpuArea>,
        position_write: PositionWrite,
        depth_write: DepthWrite,
        keyed_glyph_ids: KeyedGlyphIds,
        draw_section: DrawSection,
        atlas_texture_dimensions: AtlasTextureDimensions,
        atlas_dimension: AtlasDimension,
        atlas_free_locations: AtlasFreeLocations,
        atlas_glyph_references: AtlasGlyphReferences,
        atlas_write_queue: AtlasWriteQueue,
        atlas_add_queue: AtlasAddQueue,
        atlas_glyphs: AtlasGlyphs,
        text_placement: TextPlacement,
        text_placement_uniform: Uniform<TextPlacement>,
        glyph_color_write: AttributeWrite<Color>,
        glyph_color_cpu: CpuBuffer<Color>,
        glyph_color_gpu: GpuBuffer<Color>,
    ) -> Self {
        Self {
            max,
            position,
            visible_section,
            depth,
            atlas_block,
            unique_glyphs,
            text_scale_alignment,
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
        }
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Default, PartialEq, Component)]
pub(crate) struct TextPlacement {
    pub(crate) placement: [f32; 4],
}

impl TextPlacement {
    pub(crate) fn new(position: Position<Device>, depth: Depth) -> Self {
        Self {
            placement: [position.x, position.y, depth.layer, 0.0],
        }
    }
}

#[derive(Component)]
pub(crate) struct RenderGroupBindGroup {
    pub(crate) bind_group: wgpu::BindGroup,
}

impl RenderGroupBindGroup {
    pub(crate) fn new(
        gfx_surface: &GfxSurface,
        layout: &wgpu::BindGroupLayout,
        text_placement_uniform: &Uniform<TextPlacement>,
    ) -> Self {
        Self {
            bind_group: gfx_surface
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("render group bind group"),
                    layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: text_placement_uniform.buffer.as_entire_binding(),
                    }],
                }),
        }
    }
}

#[derive(Component, Copy, Clone)]
pub struct TextBoundGuide {
    pub horizontal_character_max: u32,
    pub line_max: u32,
}

impl TextBoundGuide {
    pub fn new(horizontal_character_max: u32, line_max: u32) -> Self {
        Self {
            horizontal_character_max,
            line_max,
        }
    }
}

#[derive(Component, Copy, Clone)]
pub(crate) struct TextBound {
    pub area: Area<View>,
}

impl TextBound {
    pub(crate) fn new<A: Into<Area<View>>>(area: A) -> Self {
        Self { area: area.into() }
    }
}

#[derive(Component)]
pub(crate) struct PositionWrite {
    pub(crate) write: Option<Position<Device>>,
}

impl PositionWrite {
    pub(crate) fn new() -> Self {
        Self { write: None }
    }
}

#[derive(Component)]
pub(crate) struct DepthWrite {
    pub(crate) write: Option<Depth>,
}

impl DepthWrite {
    pub(crate) fn new() -> Self {
        Self { write: None }
    }
}

#[derive(Component)]
pub(crate) struct KeyedGlyphIds {
    pub(crate) ids: HashMap<Key, GlyphId>,
}

impl KeyedGlyphIds {
    pub(crate) fn new() -> Self {
        Self {
            ids: HashMap::new(),
        }
    }
}

#[derive(Component)]
pub(crate) struct RenderGroupTextBound {
    pub(crate) text_bound_area: Option<Area<View>>,
}

impl RenderGroupTextBound {
    pub(crate) fn new() -> Self {
        Self {
            text_bound_area: None,
        }
    }
}

#[derive(Component)]
pub(crate) struct DrawSection {
    pub(crate) section: Option<Section<Device>>,
}

impl DrawSection {
    pub(crate) fn new() -> Self {
        Self { section: None }
    }
}
