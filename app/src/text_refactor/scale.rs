#[derive(Copy, Clone, PartialEq, Hash)]
pub struct Scale {
    pub px: u32,
}

impl Scale {
    pub fn px(&self) -> f32 {
        self.px as f32
    }
}

impl From<u32> for Scale {
    fn from(px: u32) -> Self {
        Self { px }
    }
}
