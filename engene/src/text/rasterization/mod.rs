use crate::text::{GlyphOffset, TextBufferCoordinator};
use bevy_ecs::prelude::Resource;
pub(crate) use binding::Binding;
use std::collections::{HashMap, HashSet};

mod binding;
mod glyph;
mod placement;
mod request;
mod resolve;
mod write;

use crate::instance::EntityKey;
use crate::text::font::Font;
use crate::text::rasterization::glyph::{GlyphReference, RasterizedGlyph};
use crate::text::rasterization::request::Request;
use crate::text::GlyphHash;
use crate::Canvas;
pub(crate) use placement::PlacementDescriptor;

pub(crate) fn bytes(num: usize) -> usize {
    num * std::mem::size_of::<u32>()
}
pub(crate) struct RasterizationHandler {
    pub(crate) binding: Binding,
    pub(crate) requests: Vec<Request>,
    pub(crate) placement_descriptors: HashMap<GlyphHash, PlacementDescriptor>,
    pub(crate) references: HashMap<GlyphHash, GlyphReference>,
    pub(crate) keyed_glyphs: HashMap<EntityKey<GlyphOffset>, GlyphHash>,
    pub(crate) font: Font,
    pub(crate) cached_rasterized_glyphs: HashMap<GlyphHash, RasterizedGlyph>,
    pub(crate) write_requests: HashSet<GlyphHash>,
    pub(crate) retain_glyphs: HashSet<GlyphHash>,
}
impl RasterizationHandler {
    pub(crate) fn new(device: &wgpu::Device) -> Self {
        Self {
            binding: Binding::new(device, 1024),
            requests: Vec::new(),
            placement_descriptors: HashMap::new(),
            references: HashMap::new(),
            keyed_glyphs: HashMap::new(),
            font: Font::default(),
            cached_rasterized_glyphs: HashMap::new(),
            write_requests: HashSet::new(),
            retain_glyphs: HashSet::new(),
        }
    }
    pub(crate) fn read_requests(&mut self, coordinator: &TextBufferCoordinator) {
        for key in coordinator.remove_handler.removes.iter() {
            if let Some(glyph) = self.keyed_glyphs.get(key).copied() {
                self.keyed_glyphs.remove(key);
                self.decrement_reference(&glyph);
            }
        }
        for (key, request) in coordinator.request_handler.requests.iter() {
            let old = self.keyed_glyphs.insert(*key, request.data.glyph_hash);
            if let Some(glyph_hash) = old {
                if request.data.glyph_hash == glyph_hash {
                    continue;
                }
                self.decrement_reference(&glyph_hash);
            }
            self.requests.push(Request::new(
                request.data.glyph_hash,
                request.data.character,
                request.data.scale,
            ));
        }
    }

    fn decrement_reference(&mut self, glyph_hash: &GlyphHash) {
        self.references
            .get_mut(&glyph_hash)
            .expect("no reference count for glyph")
            .decrement()
    }
    pub(crate) fn prepare(&mut self, canvas: &Canvas) {
        request::request(self);
        resolve::resolve(self);
        write::write(self, canvas);
    }
    pub(crate) fn integrate_requests(&self, coordinator: &mut TextBufferCoordinator) {
        for (_key, mut request) in coordinator.request_handler.requests.iter_mut() {
            request.data.placement_descriptor.replace(
                *self
                    .placement_descriptors
                    .get(&request.data.glyph_hash)
                    .expect("no placement descriptor for requested glyph hash"),
            );
        }
    }
}
