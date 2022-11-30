pub struct Attributes<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync> {
    pub buffer: Vec<Attribute>,
}
