use std::marker::PhantomData;

pub struct Attributes<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync> {
    pub buffer: wgpu::Buffer,
    _phantom: PhantomData<Attribute>,
}
