pub struct Attributes<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
> {
    pub buffer: Vec<Attribute>,
}

impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default>
    Attributes<Attribute>
{
    pub fn new(size: u32) -> Self {
        Self {
            buffer: {
                let mut vec = Vec::new();
                vec.resize(size as usize, Attribute::default());
                vec
            },
        }
    }
}
