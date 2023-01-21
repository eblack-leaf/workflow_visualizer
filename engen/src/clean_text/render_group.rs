use bevy_ecs::prelude::{Added, Changed, Entity, Or, Query, RemovedComponents, ResMut};
use crate::{Canvas, Color, Depth, Position, Section, Visibility};
use crate::clean_text::cache::Cache;
use crate::clean_text::coords::Coords;
use crate::clean_text::extraction::{Difference, Extraction};
use crate::clean_text::glyph;
use crate::clean_text::glyph::Key;
use crate::clean_text::index::Indexer;
use crate::clean_text::scale::Scale;
use crate::clean_text::text::Text;
use crate::uniform::Uniform;

pub(crate) struct RenderGroup {
    pub(crate) bounds: Option<Section>,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) position_uniform: Uniform<Position>,
    pub(crate) depth_uniform: Uniform<Depth>,
    pub(crate) color_uniform: Uniform<Color>,
    pub(crate) indexer: Indexer<glyph::Key>,
    pub(crate) null_gpu: wgpu::Buffer,
    pub(crate) null_cpu: Vec<bool>,
    pub(crate) coords_gpu: wgpu::Buffer,
    pub(crate) coords_cpu: Vec<Coords>,
    pub(crate) glyph_position_cpu: Vec<Position>,
    pub(crate) glyph_position_gpu: wgpu::Buffer,
}
impl RenderGroup {
    pub(crate) fn new(canvas: &Canvas, bind_group_layout: &wgpu::BindGroupLayout, position: Position, depth: Depth, color: Color, max: u32) -> Self {
        Self {
            bounds: None,
            bind_group: canvas.device.create_bind_group(),
            position_uniform: Uniform::new(&canvas.device, position),
            depth_uniform: Uniform::new(&canvas.device, depth),
            color_uniform: Uniform::new(&canvas.device, color),
            indexer: Indexer::new(max),
            null_gpu: canvas.device.create_buffer(),
            null_cpu: Vec::new(),
            coords_gpu: canvas.device.create_buffer(),
            coords_cpu: Vec::new(),
            glyph_position_gpu: canvas.device.create_buffer(),
            glyph_position_cpu: Vec::new(),
        }
    }
    fn update_null(&mut self, key: Key, null_bit: bool) {
        // queue write of null
    }
    pub(crate) fn count(&self) -> u32 {
        0
    }
    pub(crate) fn add(&mut self, key: Key, glyph_position: Position) {
        // make index (next)
        // queue write of position
        self.update_null(key, false);
    }
    pub(crate) fn update_color(&mut self, key: Key, color: Color) {
        // queue color uniform write
    }
    pub(crate) fn update_coords(&mut self, key: Key, coords: Coords) {
        // queue write to coords buffer
    }
    pub(crate) fn update_position(&mut self, key: Key, position: Position) {
        // queue write to position uniform
    }
    pub(crate) fn update_depth(&mut self, key: Key, depth: Depth) {
        // queue write to depth uniform
    }
    pub(crate) fn remove(&mut self, key: Key) {
        // remove index
        // queue write to null to inactive
    }
    pub(crate) fn write(&mut self, canvas: &Canvas) {
        // write null
        // write coords
        // write uniforms
        // write atlas
    }
}
pub(crate) fn depth_change(text: Query<(Entity, &Depth, &mut Difference), (Changed<Depth>)>) {}
pub(crate) fn position_change(text: Query<(Entity, &Position, &mut Difference), (Changed<Position>)>) {

}
pub(crate) fn scale_change(text: Query<(Entity, &mut Cache), (Changed<Scale>)>) {
    // clear cache + compute side stuff
    // add to extraction as new entity
}
pub(crate) fn manage_render_groups(text: Query<(Entity), (Or<(Changed<Visibility>, Added<Text>)>)>,
                                   removed: RemovedComponents<Text>,
                                   mut extraction: ResMut<Extraction>) {
    // if visible / or added - extraction.added_render_groups.insert(entity, info)
    // if not visible / or removed - extraction.removed_render_groups.insert(entity)
}