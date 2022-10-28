#[derive(Copy, Clone, PartialEq, Hash)]
pub struct TextScale {
    pub px: u32,
}
impl TextScale {
    pub fn px(&self) -> f32 {
        self.px as f32
    }
}
impl From<u32> for TextScale {
    fn from(px: u32) -> Self {
        Self { px }
    }
}
