use std::collections::HashMap;

use bevy_ecs::prelude::Component;

use crate::{
    Color, DeviceContext, Indexer, InstanceAttributeManager, Key, Layer, NullBit, Position,
    RawArea, RawPosition, Section, Uniform, VisibleSection,
};
use crate::gfx::GfxSurface;
use crate::text::atlas::{AtlasAddQueue, AtlasGlyphReferences, AtlasGlyphs, AtlasWriteQueue};
use crate::text::component::{GlyphId, TextScaleAlignment, TextValue};
use crate::texture_atlas::{TextureAtlas, TextureBindGroup, TextureCoordinates};

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Default, PartialEq, Component)]
pub(crate) struct TextPlacement {
    pub(crate) placement: [f32; 4],
}

impl TextPlacement {
    pub(crate) fn new(position: Position<DeviceContext>, layer: Layer) -> Self {
        Self {
            placement: [position.x, position.y, layer.z, 0.0],
        }
    }
}

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
pub(crate) struct DrawSection {
    pub(crate) section: Option<Section<DeviceContext>>,
}

impl DrawSection {
    pub(crate) fn new() -> Self {
        Self { section: None }
    }
}
pub(crate) struct PositionWrite {
    pub(crate) write: Option<Position<DeviceContext>>,
}

impl PositionWrite {
    pub(crate) fn new() -> Self {
        Self { write: None }
    }
}

pub(crate) struct LayerWrite {
    pub(crate) write: Option<Layer>,
}

impl LayerWrite {
    pub(crate) fn new() -> Self {
        Self { write: None }
    }
}

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

#[derive(Copy, Clone)]
pub(crate) struct RenderGroupUniqueGlyphs {
    pub(crate) unique_glyphs: u32,
}

impl RenderGroupUniqueGlyphs {
    pub(crate) fn from_text(text: &TextValue) -> Self {
        let length = text.0.len();
        Self {
            unique_glyphs: length.min(1024) as u32,
        }
    }
}
pub(crate) struct RenderGroup {
    pub(crate) position: Position<DeviceContext>,
    pub(crate) visible_section: VisibleSection,
    pub(crate) layer: Layer,
    pub(crate) position_write: PositionWrite,
    pub(crate) layer_write: LayerWrite,
    pub(crate) keyed_glyph_ids: KeyedGlyphIds,
    pub(crate) draw_section: DrawSection,
    pub(crate) text_placement: TextPlacement,
    pub(crate) text_placement_uniform: Uniform<TextPlacement>,
    pub(crate) unique_glyphs: RenderGroupUniqueGlyphs,
    pub(crate) text_scale_alignment: TextScaleAlignment,
    pub(crate) indexer: Indexer<Key>,
    pub(crate) glyph_positions: InstanceAttributeManager<RawPosition>,
    pub(crate) glyph_areas: InstanceAttributeManager<RawArea>,
    pub(crate) glyph_colors: InstanceAttributeManager<Color>,
    pub(crate) null_bits: InstanceAttributeManager<NullBit>,
    pub(crate) glyph_tex_coords: InstanceAttributeManager<TextureCoordinates>,
    pub(crate) render_group_bind_group: RenderGroupBindGroup,
    pub(crate) atlas: TextureAtlas,
    pub(crate) atlas_bind_group: TextureBindGroup,
    pub(crate) atlas_glyph_references: AtlasGlyphReferences,
    pub(crate) atlas_write_queue: AtlasWriteQueue,
    pub(crate) atlas_add_queue: AtlasAddQueue,
    pub(crate) atlas_glyphs: AtlasGlyphs,
}
