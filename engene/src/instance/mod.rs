mod attribute;
mod key;
use crate::instance::attribute::AttributeBuffer;
use crate::task::Container;
use bevy_ecs::prelude::Resource;
pub use key::EntityKey;
use std::hash::Hash;
use std::marker::PhantomData;
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct Index(pub(crate) usize);
#[derive(Resource)]
pub(crate) struct IndexHandler {}
impl IndexHandler {
    pub(crate) fn new() -> Self {
        Self {}
    }
    pub(crate) fn max(&self) -> usize {
        0
    }
    pub(crate) fn current(&self) -> usize {
        0
    }
}
pub trait AttributeHandler<Request>
where
    Self::Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
{
    type Attribute;
    fn extract(request: &Request) -> Self::Attribute;
}
pub struct Coordinator<Key: Eq + Hash + PartialEq + Copy + Clone, Request: 'static> {
    // open slot policy - nullable - shrink occasionally
    pub(crate) container: Container,
    _key: PhantomData<Key>,
    _request: PhantomData<Request>,
}
impl<Key: Eq + Hash + PartialEq + Copy + Clone, Request: 'static> Coordinator<Key, Request> {
    pub fn new() -> Self {
        Self {
            container: Container::new(),
            _key: PhantomData,
            _request: PhantomData,
        }
    }
    pub fn setup_attribute<AH: AttributeHandler<Request>>(&mut self, device: &wgpu::Device) {
        self.container
            .get_non_send_resource_mut::<Vec<fn(&Request) -> AH::Attribute>>()
            .expect("no attribute handler call vec attached")
            .push(AH::extract);
        self.container
            .insert_resource(attribute::gpu_buffer::<AH::Attribute>(device, self.max()));
    }
    pub fn gpu_buffer<AH: AttributeHandler<Request>>(&self) -> &wgpu::Buffer {
        &self
            .container
            .get_resource::<AttributeBuffer<AH::Attribute>>()
            .expect("no gpu buffer attached")
            .buffer
    }
    pub fn prepare(&mut self) {}
    pub fn process(&mut self) {}
    pub fn finish(&mut self) {}
    pub fn current(&self) -> usize {
        self.container
            .get_resource::<IndexHandler>()
            .expect("no index handler attached")
            .current()
    }
    pub fn max(&self) -> usize {
        self.container
            .get_resource::<IndexHandler>()
            .expect("no index handler attached")
            .max()
    }
}
