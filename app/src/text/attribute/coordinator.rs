pub struct Coordinator {
    pub current: u32,
    pub max: u32,
}
impl Coordinator {
    pub fn new(max: u32) -> Self {
        Self { current: 0, max }
    }
}
