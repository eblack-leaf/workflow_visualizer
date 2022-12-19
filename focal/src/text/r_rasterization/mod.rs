use crate::text::font::{font, Font};
pub(crate) use crate::text::r_rasterization::placement::{place, Placement, PlacementRequest};
pub(crate) use crate::text::r_rasterization::rasterize::{rasterize, Add, Glyph, GlyphHash};
pub(crate) use buffer::{write, Buffer};
use remove::Remove;
use std::collections::HashMap;
use swap::Swap;

mod buffer;
mod placement;
mod rasterize;
mod remove;
mod swap;

pub(crate) struct Rasterization {
    pub(crate) buffer: Buffer,
    pub(crate) adds: Vec<Add>,
    pub(crate) removes: Vec<Remove>,
    pub(crate) swaps: Vec<Swap>,
    pub(crate) glyphs: HashMap<GlyphHash, Glyph>,
    pub(crate) placements: Vec<Placement>,
    pub(crate) placement_order: HashMap<GlyphHash, usize>,
    pub(crate) placement_requests: Vec<PlacementRequest>,
    pub(crate) font: Font,
    pub(crate) write: Vec<u32>,
}
impl Rasterization {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            buffer: Buffer::new(device, 1024),
            adds: Vec::new(),
            removes: Vec::new(),
            swaps: Vec::new(),
            glyphs: HashMap::new(),
            placements: Vec::new(),
            placement_order: HashMap::new(),
            placement_requests: Vec::new(),
            font: font(),
            write: Vec::new(),
        }
    }
}
