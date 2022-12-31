use crate::text::rasterization::descriptor::GlyphDescriptor;
pub(crate) use crate::text::rasterization::descriptor::{place, Descriptor, DescriptorRequest};
use crate::text::rasterization::font::{font, Font};
pub(crate) use crate::text::rasterization::rasterize::{rasterize, Add, Glyph, GlyphHash};
use crate::text::rasterization::references::References;
use crate::text::InstanceCoordinator;
pub(crate) use buffer::{write, Buffer};
pub(crate) use references::{get_reference, resolve};
pub(crate) use remove::{remove, Remove};
use std::collections::{HashMap, HashSet};

mod buffer;
mod descriptor;
mod font;
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
    pub(crate) descriptors: Vec<GlyphDescriptor>,
    pub(crate) descriptor_order: HashMap<GlyphHash, usize>,
    pub(crate) descriptor_requests: Vec<DescriptorRequest>,
    pub(crate) references: References,
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
            descriptors: Vec::new(),
            descriptor_order: HashMap::new(),
            descriptor_requests: Vec::new(),
            references: HashMap::new(),
            font: font(),
            write: Vec::new(),
        }
    }
}
pub(crate) fn read_requests(rasterization: &mut Rasterization, coordinator: &InstanceCoordinator) {
    // use request.hash to send rasterize_request
}
pub(crate) fn integrate_placements(
    rasterization: &Rasterization,
    coordinator: &mut InstanceCoordinator,
) {
    // for each instance_request - put Some(placement) in request.placement using request.hash as key
}
