#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default, PartialEq)]
pub struct PlacementDescriptor {
    pub parts: [f32; 4],
}

impl PlacementDescriptor {
    pub fn new(start: usize, glyph_width: usize, glyph_height: usize) -> Self {
        let row_size = glyph_width;
        let rows = glyph_width * glyph_height / row_size;
        Self {
            parts: [start as u32, row_size as u32, rows as u32],
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
