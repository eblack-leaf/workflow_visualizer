mod attribute;
mod key;
use crate::instance::attribute::AttributeBuffer;
use crate::task::{Stage, WorkloadId};
use crate::Task;
use bevy_ecs::prelude::Resource;
pub use key::EntityKey;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct Index {
    pub(crate) value: usize,
}
impl Index {
    pub(crate) fn new(value: usize) -> Self {
        Self { value }
    }
}
impl From<usize> for Index {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}
#[derive(Resource)]
pub(crate) struct IndexHandler {}
impl IndexHandler {
    pub(crate) fn new(max: usize) -> Self {
        Self {}
    }
    pub(crate) fn max(&self) -> usize {
        0
    }
    pub(crate) fn count(&self) -> usize {
        0
    }
    pub(crate) fn current_index(&self) -> Option<Index> {
        None
    }
}
pub trait AttributeHandler<Request>
where
    Self: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
{
    fn extract(request: &Request) -> Self;
}
pub struct Coordinator<Key: Eq + Hash + PartialEq + Copy + Clone + 'static, Request: 'static> {
    // open slot policy - nullable - shrink occasionally
    pub(crate) task: Task,
    _key: PhantomData<Key>,
    _request: PhantomData<Request>,
}
impl<Key: Eq + Hash + PartialEq + Copy + Clone + 'static, Request: 'static>
    Coordinator<Key, Request>
{
    pub fn new(max: usize) -> Self {
        Self {
            task: {
                let mut task = Task::new();
                task.main
                    .schedule
                    .add_system_to_stage(Stage::Before, Self::start_processing);
                task.main
                    .schedule
                    .add_system_to_stage(Stage::After, Self::finish_processing);
                task.container.insert_resource(IndexHandler::new(max));
                task
            },
            _key: PhantomData,
            _request: PhantomData,
        }
    }
    pub fn setup_attribute<AH: AttributeHandler<Request>>(&mut self, device: &wgpu::Device) {
        self.task
            .main
            .schedule
            .add_system_to_stage(Stage::During, Self::process_attribute::<AH>);
        self.task
            .container
            .insert_resource(attribute::CpuBuffer::<AH>::new(self.max()));
        self.task
            .container
            .insert_resource(attribute::gpu_buffer::<AH>(device, self.max()));
    }
    fn start_processing() {}
    fn process_attribute<AH: AttributeHandler<Request>>(/* add all data as system query */) {}
    fn finish_processing() {}
    pub fn gpu_buffer<AH: AttributeHandler<Request>>(&self) -> &wgpu::Buffer {
        &self
            .task
            .container
            .get_resource::<AttributeBuffer<AH>>()
            .expect("no gpu buffer attached")
            .buffer
    }
    pub fn coordinate(&mut self) {
        self.task.exec(WorkloadId::Main);
    }
    pub fn count(&self) -> usize {
        self.task
            .container
            .get_resource::<IndexHandler>()
            .expect("no index handler attached")
            .count()
    }
    pub fn max(&self) -> usize {
        self.task
            .container
            .get_resource::<IndexHandler>()
            .expect("no index handler attached")
            .max()
    }
    pub fn has_instances(&self) -> bool {
        self.count() > 0
    }
}
