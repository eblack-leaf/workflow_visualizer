use bytemuck::{Pod, Zeroable};
use crate::coord::Section;

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Default)]
pub(crate) struct Coords {
    pub(crate) data: [f32; 4],
}

impl Coords {
    pub(crate) fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            data: [left, top, right, bottom],
        }
    }
    pub(crate) fn section(&self) -> Section {
        Section::new(
            (self.data[0], self.data[1]),
            (self.data[2] - self.data[0], self.data[3] - self.data[1]),
        )
    }
}
