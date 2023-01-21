use std::collections::HashMap;

use bevy_ecs::prelude::{Added, Changed, Entity, Or, Query, RemovedComponents, ResMut, With};
use wgpu::{BindGroupEntry, Buffer, BufferUsages};

use crate::{Area, Canvas, Color, Depth, Position, Section, Visibility};
use crate::clean_text::atlas::Atlas;
use crate::clean_text::cache::Cache;
use crate::clean_text::coords::Coords;
use crate::clean_text::extraction::{Difference, Extraction};
use crate::clean_text::glyph::{Glyph, GlyphId, Key};
use crate::clean_text::index::{Index, Indexer};
use crate::clean_text::scale::Scale;
use crate::clean_text::text::Text;
use crate::uniform::Uniform;

pub(crate) struct RenderGroup {
    pub(crate) bounds: Option<Section>,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) position_uniform: Uniform<Position>,
    pub(crate) position_write: Option<Position>,
    pub(crate) depth_uniform: Uniform<Depth>,
    pub(crate) depth_write: Option<Depth>,
    pub(crate) color_uniform: Uniform<Color>,
    pub(crate) color_write: Option<Color>,
    pub(crate) indexer: Indexer<Key>,
    pub(crate) null_gpu: wgpu::Buffer,
    pub(crate) null_cpu: Vec<bool>,
    pub(crate) null_write: HashMap<Index, bool>,
    pub(crate) coords_gpu: wgpu::Buffer,
    pub(crate) coords_cpu: Vec<Coords>,
    pub(crate) coords_write: HashMap<Index, Coords>,
    pub(crate) glyph_position_cpu: Vec<Position>,
    pub(crate) glyph_position_gpu: wgpu::Buffer,
    pub(crate) glyph_position_write: HashMap<Index, Position>,
    pub(crate) keyed_glyph_ids: HashMap<Key, GlyphId>,
    pub(crate) atlas: Atlas,
}

impl RenderGroup {
    pub(crate) fn new(canvas: &Canvas, bind_group_layout: &wgpu::BindGroupLayout, max: u32, position: Position, depth: Depth, color: Color, unique_glyphs: u32) -> Self {
        let position_uniform = Uniform::new(&canvas.device, position);
        let depth_uniform = Uniform::new(&canvas.device, depth);
        let color_uniform = Uniform::new(&canvas.device, color);
        let atlas = Atlas::new(canvas, unique_glyphs);
        Self {
            bounds: None,
            bind_group: canvas.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("render group bind group"),
                layout: bind_group_layout,
                entries: &[
                    // position
                    BindGroupEntry {
                        binding: 0,
                        resource: position_uniform.buffer.as_entire_binding(),
                    },
                    // depth
                    BindGroupEntry {
                        binding: 1,
                        resource: depth_uniform.buffer.as_entire_binding(),
                    },
                    // color
                    BindGroupEntry {
                        binding: 2,
                        resource: color_uniform.buffer.as_entire_binding(),
                    },
                    // texture
                    BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::TextureView(&atlas.texture_view),
                    }
                ],
            }),
            position_uniform,
            position_write: None,
            depth_uniform,
            depth_write: None,
            color_uniform,
            color_write: None,
            indexer: Indexer::new(max),
            null_gpu: Self::gpu_buffer::<bool>(canvas, max),
            null_cpu: Self::cpu_buffer(max),
            null_write: HashMap::new(),
            coords_gpu: Self::gpu_buffer::<Coords>(canvas, max),
            coords_cpu: Self::cpu_buffer(max),
            glyph_position_gpu: Self::gpu_buffer::<Position>(canvas, max),
            glyph_position_cpu: Self::cpu_buffer(max),
            coords_write: HashMap::new(),
            glyph_position_write: HashMap::new(),
            keyed_glyph_ids: HashMap::new(),
            atlas,
        }
    }
    pub(crate) fn count(&self) -> u32 {
        self.indexer.count()
    }
    pub(crate) fn add_glyph(&mut self, key: Key, glyph: Glyph) {
        if let Some(glyph_id) = self.keyed_glyph_ids.get(&key) {
            if glyph_id == glyph.id {
                return;
            }
        }
        self.keyed_glyph_ids.insert(key, glyph.id);
        self.atlas.add_glyph(key, glyph);
    }
    pub(crate) fn remove_glyph(&mut self, glyph_id: GlyphId) {
        self.atlas.remove_glyph(glyph_id);
    }
    pub(crate) fn read_glyph_coords(&self, key: Key) -> Coords {
        let glyph_id = self.get_glyph_id(key);
        self.atlas.read_glyph_coords(glyph_id)
    }
    pub(crate) fn add(&mut self, key: Key, glyph_position: Position) {
        let _index = self.indexer.next(key);
        self.queue_glyph_position(key, glyph_position);
        self.queue_null(key, false);
    }
    pub(crate) fn remove(&mut self, key: Key) {
        self.keyed_glyph_ids.remove(&key);
        self.queue_null(key, true);
        let _old_index = self.indexer.remove(key);
    }
    pub(crate) fn write(&mut self, canvas: &Canvas) {
        self.write_glyph_positions(canvas);
        self.write_null(canvas);
        self.write_coords(canvas);
        self.write_position(canvas);
        self.write_depth(canvas);
        self.write_color(canvas);
        self.atlas.write(canvas);
        self.reset_writes();
    }
    pub(crate) fn queue_glyph_position(&mut self, key: Key, glyph_position: Position) {
        let index = self.get_index(key);
        self.glyph_position_write.insert(index, glyph_position);
    }
    pub(crate) fn queue_color(&mut self, color: Color) {
        self.color_write.replace(color);
    }
    pub(crate) fn queue_coords(&mut self, key: Key, coords: Coords) {
        let index = self.get_index(key);
        self.coords_write.insert(index, coords);
    }
    pub(crate) fn queue_position(&mut self, position: Position) {
        self.position_write.replace(position);
    }
    pub(crate) fn queue_depth(&mut self, depth: Depth) {
        self.depth_write.replace(depth);
    }
    fn queue_null(&mut self, key: Key, null_bit: bool) {
        let index = self.get_index(key);
        self.null_write.insert(index, null_bit);
    }
    fn gpu_buffer<T>(canvas: &Canvas, max: u32) -> Buffer {
        canvas.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("render group gpu buffer"),
            size: (std::mem::size_of::<T>() * max) as wgpu::BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }
    fn cpu_buffer<T: Default>(max: u32) -> Vec<T> {
        let mut vec = Vec::new();
        vec.resize(max as usize, T::default());
        vec
    }
    fn get_index(&self, key: Key) -> Index {
        self.indexer.get_index(key).expect("no index for key")
    }
    fn write_color(&mut self, canvas: &Canvas) {
        if let Some(color) = self.color_write.take() {
            self.color_uniform.update(&canvas.queue, color);
        }
    }
    fn write_depth(&mut self, canvas: &Canvas) {
        if let Some(depth) = self.depth_write.take() {
            self.depth_uniform.update(&canvas.queue, depth);
        }
    }
    fn write_position(&mut self, canvas: &Canvas) {
        if let Some(position) = self.position_write.take() {
            self.position_uniform.update(&canvas.queue, position);
        }
    }
    fn write_coords(&mut self, canvas: &Canvas) {
        for (index, coords) in self.coords_write.iter() {
            self.coords_cpu.insert(index.value as usize, *coords);
            let offset = (std::mem::size_of::<Position>() * index.value) as wgpu::BufferAddress;
            canvas.queue.write_buffer(&self.coords_gpu, offset, bytemuck::cast_slice(&[*coords]));
        }
    }
    fn write_null(&mut self, canvas: &Canvas) {
        for (index, null) in self.null_write.iter() {
            self.null_cpu.insert(index.value as usize, *null);
            let offset = (std::mem::size_of::<bool>() * index.value) as wgpu::BufferAddress;
            canvas.queue.write_buffer(&self.null_gpu, offset, bytemuck::cast_slice(&[*null]));
        }
    }
    fn write_glyph_positions(&mut self, canvas: &Canvas) {
        for (index, glyph_position) in self.glyph_position_write.iter() {
            self.glyph_position_cpu.insert(index.value as usize, *glyph_position);
            let offset = (std::mem::size_of::<Position>() * index.value) as wgpu::BufferAddress;
            canvas.queue.write_buffer(&self.glyph_position_gpu, offset, bytemuck::cast_slice(&[*glyph_position]));
        }
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

pub(crate) fn depth_diff(mut text: Query<(Entity, &Depth, &mut Cache, &mut Difference), (Changed<Depth>, With<Text>)>) {
    for (entity, depth, mut cache, mut difference) in text.iter_mut() {
        if *depth != cache.depth {
            difference.depth.replace(*depth);
        }
    }
}

pub(crate) fn position_diff(mut text: Query<(Entity, &Position, &mut Cache, &mut Difference), (Changed<Position>, With<Text>)>) {
    for (entity, position, mut cache, mut difference) in text.iter_mut() {
        if *position != cache.position {
            difference.position.replace(*position);
        }
    }
}

pub(crate) fn color_diff(mut text: Query<(Entity, &Color, &mut Cache, &mut Difference), (Changed<Color>, With<Text>)>) {
    for (entity, color, mut cache, mut difference) in text.iter_mut() {
        if *color != cache.color {
            difference.color.replace(*color);
        }
    }
}

pub(crate) fn manage_render_groups(text: Query<(Entity, &Position, &Depth, &Color, Option<&Area>, &mut Cache, &mut Difference), (Or<(Changed<Visibility>, Added<Text>, Changed<Scale>)>)>,
                                   removed: RemovedComponents<Text>,
                                   mut extraction: ResMut<Extraction>) {
    // clear cache + write initial to diff
    // if visible / or added - extraction.added_render_groups.insert(entity, info)
    // if not visible / or removed - extraction.removed_render_groups.insert(entity)
}