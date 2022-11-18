#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RasterizationDescriptor {
    pub parts: [u32; 3],
}

impl RasterizationDescriptor {
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
}
