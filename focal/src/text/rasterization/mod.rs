use crate::instance_coordinator::InstanceCoordinator;
use crate::text::font::{font, Font};
use crate::text::rasterization::placement::GlyphPlacement;
pub(crate) use crate::text::rasterization::placement::{place, Placement, PlacementRequest};
pub(crate) use crate::text::rasterization::rasterize::{rasterize, Add, Glyph, GlyphHash};
use crate::text::rasterization::references::PlacementReferences;
use crate::text::TextInstanceCoordinator;
pub(crate) use buffer::{write, Buffer};
pub(crate) use references::{decrement_reference, get_reference, increment_reference, resolve};
pub(crate) use remove::{remove, Remove};
use std::collections::{HashMap, HashSet};

mod buffer;
mod placement;
mod rasterize;
mod references;
mod remove;

pub(crate) struct Rasterization {
    pub(crate) buffer: Buffer,
    pub(crate) adds: Vec<Add>,
    pub(crate) removes: Vec<Remove>,
    pub(crate) swapped_glyphs: HashSet<GlyphHash>,
    pub(crate) glyphs: HashMap<GlyphHash, Glyph>,
    pub(crate) retain_glyphs: HashSet<GlyphHash>,
    pub(crate) placements: Vec<GlyphPlacement>,
    pub(crate) placement_order: HashMap<GlyphHash, usize>,
    pub(crate) placement_requests: Vec<PlacementRequest>,
    pub(crate) placement_references: PlacementReferences,
    pub(crate) font: Font,
    pub(crate) write: Vec<u32>,
}
impl Rasterization {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            buffer: Buffer::new(device, 1024),
            adds: Vec::new(),
            removes: Vec::new(),
            swapped_glyphs: HashSet::new(),
            glyphs: HashMap::new(),
            retain_glyphs: HashSet::new(),
            placements: Vec::new(),
            placement_order: HashMap::new(),
            placement_requests: Vec::new(),
            placement_references: HashMap::new(),
            font: font(),
            write: Vec::new(),
        }
    }
}
pub(crate) fn read_requests(
    rasterization: &mut Rasterization,
    coordinator: &TextInstanceCoordinator,
) {
    // use request.hash to send rasterize_request
}
pub(crate) fn integrate_placements(
    rasterization: &Rasterization,
    coordinator: &mut TextInstanceCoordinator,
) {
    // for each instance_request - put Some(placement) in request.placement using request.hash as key
}
