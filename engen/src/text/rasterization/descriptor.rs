use crate::text::rasterization::rasterize::{Glyph, GlyphHash};
use crate::text::rasterization::Rasterization;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default, PartialEq)]
pub struct Descriptor {
    pub parts: [u32; 3],
}
impl Descriptor {
    pub fn new(start: u32, row_size: u32, rows: u32) -> Self {
        Self {
            parts: [start, row_size, rows],
        }
    }
    pub fn start(&self) -> u32 {
        self.parts[0]
    }
    pub fn row_size(&self) -> u32 {
        self.parts[1]
    }
    pub fn rows(&self) -> u32 {
        self.parts[2]
    }
    pub fn size(&self) -> u32 {
        self.row_size() * self.rows()
    }
    pub fn end(&self) -> u32 {
        self.start() + self.size()
    }
}
pub(crate) struct DescriptorRequest {
    pub(crate) hash: GlyphHash,
    pub(crate) glyph: Glyph,
}
impl DescriptorRequest {
    pub(crate) fn new(hash: GlyphHash, glyph: Glyph) -> Self {
        Self { hash, glyph }
    }
}
pub(crate) struct GlyphDescriptor {
    pub(crate) hash: GlyphHash,
    pub(crate) descriptor: Descriptor,
}
impl GlyphDescriptor {
    pub(crate) fn new(hash: GlyphHash, descriptor: Descriptor) -> Self {
        Self { hash, descriptor }
    }
}
pub(crate) fn place(rasterization: &mut Rasterization) {
    for request in rasterization.descriptor_requests.iter() {
        if !rasterization.descriptor_order.contains_key(&request.hash) {
            let start: u32 = (rasterization.buffer.cpu.len() + rasterization.write.len()) as u32;
            let row_size: u32 = request.glyph.metrics.width as u32;
            let rows: u32 = (request.glyph.bitmap.len() / row_size as usize) as u32;
            let descriptor = Descriptor::new(start, row_size, rows);
            rasterization
                .descriptors
                .push(GlyphDescriptor::new(request.hash, descriptor));
            rasterization
                .descriptor_order
                .insert(request.hash, rasterization.descriptors.len() - 1);
            rasterization.write.extend(&request.glyph.bitmap);
        }
    }
    rasterization.descriptor_requests.clear();
}
