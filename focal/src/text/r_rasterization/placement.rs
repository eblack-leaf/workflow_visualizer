use crate::text::r_rasterization::rasterize::{Glyph, GlyphHash};
use crate::text::r_rasterization::Rasterization;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub(crate) struct Placement {
    pub parts: [u32; 3],
}
impl Placement {
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
}
pub(crate) struct PlacementRequest {
    pub(crate) hash: GlyphHash,
    pub(crate) glyph: Glyph,
}
impl PlacementRequest {
    pub(crate) fn new(hash: GlyphHash, glyph: Glyph) -> Self {
        Self { hash, glyph }
    }
}
pub(crate) fn place(rasterization: &mut Rasterization) {
    for request in rasterization.placement_requests.iter() {
        if !rasterization.placement_order.contains_key(&request.hash) {
            let start: u32 = (rasterization.buffer.cpu.len() + rasterization.write.len()) as u32;
            let row_size: u32 = request.glyph.metrics.width as u32;
            let rows: u32 = (request.glyph.bitmap.len() / row_size as usize) as u32;
            let placement = Placement::new(start, row_size, rows);
            rasterization.write.extend(&request.glyph.bitmap);
        }
    }
}
