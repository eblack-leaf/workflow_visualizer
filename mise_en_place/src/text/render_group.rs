use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{
    Added, Changed, Component, Entity, Or, Query, RemovedComponents, ResMut, With,
};
use bytemuck::{Pod, Zeroable};
use wgpu::{BindGroupEntry, Buffer, BufferAddress, BufferUsages};

use crate::{Color, TextScaleAlignment};
use crate::coord::{Area, Depth, Position, ScaledArea, ScaledPosition, ScaledSection, Section};
use crate::gfx::GfxSurface;
use crate::text::atlas::Atlas;
use crate::text::cache::Cache;
use crate::text::coords::Coords;
use crate::text::difference::Difference;
use crate::text::extraction::Extraction;
use crate::text::font::MonoSpacedFont;
use crate::text::glyph::{Glyph, GlyphId, Key};
use crate::text::index::{Index, Indexer};
use crate::text::scale::{AlignedFonts, TextScale};
use crate::text::text::Text;
use crate::uniform::Uniform;
use crate::visibility::{Visibility, VisibleSection};

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
pub(crate) struct NullBit {
    bit: u32,
}

impl Default for NullBit {
    fn default() -> Self {
        NullBit::null()
    }
}

impl NullBit {
    pub(crate) const NOT_NULL: u32 = 0u32;
    pub(crate) const NULL: u32 = 1u32;
    fn new(bit: u32) -> Self {
        Self { bit }
    }
    pub(crate) fn not_null() -> NullBit {
        Self::new(Self::NOT_NULL)
    }
    pub(crate) fn null() -> Self {
        Self::new(Self::NULL)
    }
}

#[derive(Component, Copy, Clone)]
pub struct TextBound {
    pub area: Area,
}

impl TextBound {
    pub fn new<A: Into<Area>>(area: A) -> Self {
        Self { area: area.into() }
    }
}

pub(crate) struct RenderGroup {
    pub(crate) bound_section: Option<Section>,
    pub(crate) draw_section: Option<ScaledSection>,
    pub(crate) visible_section: VisibleSection,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) text_placement: TextPlacement,
    pub(crate) text_placement_uniform: Uniform<TextPlacement>,
    pub(crate) position_write: Option<ScaledPosition>,
    pub(crate) depth_write: Option<Depth>,
    pub(crate) color_uniform: Uniform<Color>,
    pub(crate) color_write: Option<Color>,
    pub(crate) indexer: Indexer<Key>,
    pub(crate) null_gpu: wgpu::Buffer,
    pub(crate) null_cpu: Vec<NullBit>,
    pub(crate) null_write: HashMap<Index, NullBit>,
    pub(crate) coords_gpu: wgpu::Buffer,
    pub(crate) coords_cpu: Vec<Coords>,
    pub(crate) coords_write: HashMap<Index, Coords>,
    pub(crate) glyph_position_cpu: Vec<Position>,
    pub(crate) glyph_position_gpu: wgpu::Buffer,
    pub(crate) glyph_position_write: HashMap<Index, Position>,
    pub(crate) glyph_area_cpu: Vec<Area>,
    pub(crate) glyph_area_gpu: wgpu::Buffer,
    pub(crate) glyph_area_write: HashMap<Index, Area>,
    pub(crate) keyed_glyph_ids: HashMap<Key, GlyphId>,
    pub(crate) atlas: Atlas,
    pub(crate) text_scale_alignment: TextScaleAlignment,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Default, PartialEq)]
pub(crate) struct TextPlacement {
    pub(crate) placement: [f32; 4],
}

impl TextPlacement {
    pub(crate) fn new(position: ScaledPosition, depth: Depth) -> Self {
        Self {
            placement: [position.x, position.y, depth.layer, 0.0],
        }
    }
}

impl RenderGroup {
    pub(crate) fn new(
        gfx_surface: &GfxSurface,
        bind_group_layout_descriptor: &wgpu::BindGroupLayoutDescriptor,
        max: u32,
        position: ScaledPosition,
        visible_section: VisibleSection,
        depth: Depth,
        color: Color,
        atlas_block: Area,
        unique_glyphs: u32,
        text_scale_alignment: TextScaleAlignment,
    ) -> Self {
        let text_placement = TextPlacement::new(position, depth);
        let text_placement_uniform = Uniform::new(&gfx_surface.device, text_placement);
        let color_uniform = Uniform::new(&gfx_surface.device, color);
        let atlas = Atlas::new(gfx_surface, atlas_block, unique_glyphs);
        let bind_group_layout = gfx_surface.device.create_bind_group_layout(bind_group_layout_descriptor);
        Self {
            bound_section: None,
            draw_section: None,
            visible_section,
            bind_group: gfx_surface
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("render group bind group"),
                    layout: &bind_group_layout,
                    entries: &[
                        BindGroupEntry {
                            binding: 0,
                            resource: text_placement_uniform.buffer.as_entire_binding(),
                        },
                        BindGroupEntry {
                            binding: 1,
                            resource: color_uniform.buffer.as_entire_binding(),
                        },
                        BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::TextureView(&atlas.texture_view),
                        },
                    ],
                }),
            text_placement,
            text_placement_uniform,
            position_write: None,
            depth_write: None,
            color_uniform,
            color_write: None,
            indexer: Indexer::new(max),
            null_gpu: Self::gpu_buffer::<NullBit>(gfx_surface, max),
            null_cpu: Self::cpu_buffer(max),
            null_write: HashMap::new(),
            coords_gpu: Self::gpu_buffer::<Coords>(gfx_surface, max),
            coords_cpu: Self::cpu_buffer(max),
            glyph_position_gpu: Self::gpu_buffer::<Position>(gfx_surface, max),
            glyph_position_cpu: Self::cpu_buffer(max),
            coords_write: HashMap::new(),
            glyph_position_write: HashMap::new(),
            glyph_area_cpu: Self::cpu_buffer(max),
            glyph_area_gpu: Self::gpu_buffer::<Area>(gfx_surface, max),
            glyph_area_write: HashMap::new(),
            keyed_glyph_ids: HashMap::new(),
            atlas,
            text_scale_alignment,
            bind_group_layout,
        }
    }
    pub(crate) fn grow(&mut self, gfx_surface: &GfxSurface) {
        if self.indexer.should_grow() {
            // grow glyph positions
            self.glyph_position_cpu.resize(self.indexer.max as usize, Position::default());
            self.glyph_position_gpu = Self::gpu_buffer::<Position>(gfx_surface, self.indexer.max);
            gfx_surface.queue.write_buffer(&self.glyph_position_gpu, 0, bytemuck::cast_slice(&self.glyph_position_cpu));
            // grow glyph areas
            self.glyph_area_cpu.resize(self.indexer.max as usize, Area::default());
            self.glyph_area_gpu = Self::gpu_buffer::<Area>(gfx_surface, self.indexer.max);
            gfx_surface.queue.write_buffer(&self.glyph_area_gpu, 0, bytemuck::cast_slice(&self.glyph_area_cpu));
            // grow null
            self.null_cpu.resize(self.indexer.max as usize, NullBit::default());
            self.null_gpu = Self::gpu_buffer::<NullBit>(gfx_surface, self.indexer.max);
            gfx_surface.queue.write_buffer(&self.null_gpu, 0, bytemuck::cast_slice(&self.null_cpu));
            // grow coords
            self.coords_cpu.resize(self.indexer.max as usize, Coords::default());
            self.coords_gpu = Self::gpu_buffer::<Coords>(gfx_surface, self.indexer.max);
            gfx_surface.queue.write_buffer(&self.coords_gpu, 0, bytemuck::cast_slice(&self.coords_cpu));
        }
    }
    pub(crate) fn adjust_draw_section(
        &mut self,
        viewport_section: ScaledSection,
        scale_factor: f64,
    ) {
        // dont only check if have bounds but by visible section + bounds if there
        // but since filtering visible letters this is fine
        if let Some(bounds) = self.bound_section {
            let scaled_bounds = bounds.to_scaled(scale_factor);
            let bounded_visible_bounds =
                scaled_bounds.intersection(self.visible_section.section.to_scaled(scale_factor)); // scale here
            if let Some(section) = bounded_visible_bounds {
                let viewport_bounded_visible_bounds = section.intersection(viewport_section);
                if let Some(d_section) = viewport_bounded_visible_bounds {
                    self.draw_section.replace(ScaledSection::new(
                        d_section.position - viewport_section.position,
                        d_section.area,
                    ));
                } else {
                    self.draw_section.take();
                }
            } else {
                self.draw_section.take();
            }
        } else {
            self.draw_section.take();
        }
    }
    pub(crate) fn count(&self) -> u32 {
        self.indexer.count()
    }
    pub(crate) fn add_glyph(&mut self, key: Key, glyph: Glyph) {
        self.keyed_glyph_ids.insert(key, glyph.id);
        self.atlas.add_glyph(glyph);
    }
    pub(crate) fn prepare_atlas(&mut self, gfx_surface: &GfxSurface, fonts: &AlignedFonts) {
        self.atlas.free();
        let adjusted = self.atlas.grow(gfx_surface);
        self.atlas.process_queued_adds(fonts.fonts
            .get(&self.text_scale_alignment)
            .expect("no aligned font"));
        let mut glyph_info_writes = HashSet::<(Key, GlyphId)>::new();
        if let Some(adj) = adjusted {
            self.bind_group = gfx_surface
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("render group bind group"),
                    layout: &self.bind_group_layout,
                    entries: &[
                        BindGroupEntry {
                            binding: 0,
                            resource: self.text_placement_uniform.buffer.as_entire_binding(),
                        },
                        BindGroupEntry {
                            binding: 1,
                            resource: self.color_uniform.buffer.as_entire_binding(),
                        },
                        BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::TextureView(&self.atlas.texture_view),
                        },
                    ],
                });
            for id in adj {
                for (key, glyph_id) in self.keyed_glyph_ids.iter() {
                    if id == *glyph_id { glyph_info_writes.insert((*key, *glyph_id)); }
                }
            }
            for (key, glyph_id) in glyph_info_writes {
                let (coords, area) = self.read_glyph_info(key);
                self.queue_glyph_info(key, coords, area);
            }
        }
    }
    pub(crate) fn remove_glyph(&mut self, glyph_id: GlyphId) {
        self.atlas.remove_glyph(glyph_id);
    }
    pub(crate) fn read_glyph_info(&self, key: Key) -> (Coords, Area) {
        let glyph_id = self.get_glyph_id(key);
        self.atlas.read_glyph_info(glyph_id)
    }
    pub(crate) fn add(&mut self, key: Key, glyph_position: Position) {
        let _index = self.indexer.next(key);
        self.queue_glyph_position(key, glyph_position);
        self.queue_null(key, NullBit::not_null());
    }
    pub(crate) fn remove(&mut self, key: Key) {
        self.keyed_glyph_ids.remove(&key);
        self.queue_null(key, NullBit::null());
        let _old_index = self.indexer.remove(key);
    }
    pub(crate) fn write(&mut self, gfx_surface: &GfxSurface) {
        self.write_glyph_positions(gfx_surface);
        self.write_glyph_area(gfx_surface);
        self.write_null(gfx_surface);
        self.write_coords(gfx_surface);
        self.write_text_placement(gfx_surface);
        self.write_color(gfx_surface);
        self.atlas.write(gfx_surface);
        self.reset_writes();
    }
    pub(crate) fn queue_glyph_position(&mut self, key: Key, glyph_position: Position) {
        let index = self.get_index(key);
        self.glyph_position_write.insert(index, glyph_position);
    }
    pub(crate) fn queue_color(&mut self, color: Color) {
        self.color_write.replace(color);
    }
    pub(crate) fn queue_glyph_info(&mut self, key: Key, coords: Coords, glyph_area: Area) {
        let index = self.get_index(key);
        self.glyph_area_write.insert(index, glyph_area);
        self.coords_write.insert(index, coords);
    }
    pub(crate) fn queue_position(&mut self, position: ScaledPosition) {
        self.position_write.replace(position);
    }
    pub(crate) fn queue_depth(&mut self, depth: Depth) {
        self.depth_write.replace(depth);
    }
    fn queue_null(&mut self, key: Key, null_bit: NullBit) {
        let index = self.get_index(key);
        self.null_write.insert(index, null_bit);
    }
    fn gpu_buffer<T>(gfx_surface: &GfxSurface, max: u32) -> Buffer {
        gfx_surface.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("render group gpu buffer"),
            size: (std::mem::size_of::<T>() * max as usize) as wgpu::BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }
    fn cpu_buffer<T: Default + Clone>(max: u32) -> Vec<T> {
        let mut vec = Vec::new();
        vec.resize(max as usize, T::default());
        vec
    }
    fn get_index(&self, key: Key) -> Index {
        self.indexer.get_index(key).expect("no index for key")
    }
    fn write_color(&mut self, gfx_surface: &GfxSurface) {
        if let Some(color) = self.color_write.take() {
            self.color_uniform.update(&gfx_surface.queue, color);
        }
    }
    fn write_text_placement(&mut self, gfx_surface: &GfxSurface) {
        let mut dirty = false;
        if let Some(position) = self.position_write.take() {
            self.text_placement.placement[0] = position.x;
            self.text_placement.placement[1] = position.y;
            dirty = true;
        }
        if let Some(depth) = self.depth_write.take() {
            self.text_placement.placement[2] = depth.layer;
            dirty = true;
        }
        if dirty {
            self.text_placement_uniform
                .update(&gfx_surface.queue, self.text_placement);
        }
    }
    fn write_coords(&mut self, gfx_surface: &GfxSurface) {
        for (index, coords) in self.coords_write.iter() {
            self.coords_cpu.insert(index.value as usize, *coords);
            let offset = Self::offset::<Coords>(index);
            gfx_surface.queue.write_buffer(
                &self.coords_gpu,
                offset,
                bytemuck::cast_slice(&[*coords]),
            );
        }
    }
    fn write_null(&mut self, gfx_surface: &GfxSurface) {
        for (index, null) in self.null_write.iter() {
            self.null_cpu.insert(index.value as usize, *null);
            let offset = Self::offset::<NullBit>(index);
            gfx_surface
                .queue
                .write_buffer(&self.null_gpu, offset, bytemuck::cast_slice(&[*null]));
        }
    }
    fn write_glyph_positions(&mut self, gfx_surface: &GfxSurface) {
        for (index, glyph_position) in self.glyph_position_write.iter() {
            self.glyph_position_cpu
                .insert(index.value as usize, *glyph_position);
            let offset = Self::offset::<Position>(index);
            gfx_surface.queue.write_buffer(
                &self.glyph_position_gpu,
                offset,
                bytemuck::cast_slice(&[*glyph_position]),
            );
        }
    }
    fn write_glyph_area(&mut self, gfx_surface: &GfxSurface) {
        for (index, glyph_area) in self.glyph_area_write.iter() {
            self.glyph_area_cpu
                .insert(index.value as usize, *glyph_area);
            let offset = Self::offset::<Position>(index);
            gfx_surface.queue.write_buffer(
                &self.glyph_area_gpu,
                offset,
                bytemuck::cast_slice(&[*glyph_area]),
            );
        }
    }
    fn offset<T>(index: &Index) -> BufferAddress {
        (std::mem::size_of::<T>() * index.value as usize) as wgpu::BufferAddress
    }
    fn get_glyph_id(&self, key: Key) -> GlyphId {
        *self.keyed_glyph_ids.get(&key).expect("no glyph id for key")
    }
    fn reset_writes(&mut self) {
        self.glyph_position_write.clear();
        self.null_write.clear();
        self.coords_write.clear();
    }
}
