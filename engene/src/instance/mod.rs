mod attribute;
mod index;
mod key;

pub(crate) use crate::instance::attribute::{CpuBuffer, GpuBuffer};
use crate::instance::index::Index;
use crate::task::{Stage, WorkloadId};
use crate::Task;
pub use attribute::AttributeExtractor;
use bevy_ecs::prelude::{Component, Res, ResMut, Resource};
pub(crate) use index::IndexHandler;
pub use key::EntityKey;
use std::collections::HashSet;
use std::hash::Hash;
use std::marker::PhantomData;
pub struct Request<
    Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static,
    RequestData: Send + Sync + 'static,
> {
    pub key: Key,
    pub data: RequestData,
}
#[derive(Resource)]
pub(crate) struct RequestHandler<
    Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static,
    RequestData: Send + Sync + 'static,
> {
    pub(crate) requests: Vec<Request<Key, RequestData>>,
}
impl<
        Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static,
        Request: Send + Sync + 'static,
    > RequestHandler<Key, Request>
{
    pub(crate) fn new() -> Self {
        Self {
            requests: Vec::new(),
        }
    }
}
#[derive(Resource)]
pub(crate) struct RemoveHandler<Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static> {
    pub(crate) removes: HashSet<Key>,
}
impl<Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static> RemoveHandler<Key> {
    pub(crate) fn new() -> Self {
        Self {
            removes: HashSet::new(),
        }
    }
}
#[derive(Resource)]
pub(crate) struct WriteRequests<Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static> {
    pub(crate) requests: HashSet<Key>,
}
impl<Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static> WriteRequests<Key> {
    pub(crate) fn new() -> Self {
        Self {
            requests: HashSet::new(),
        }
    }
}
#[derive(Resource)]
pub(crate) struct NullRequests {
    pub(crate) requests: HashSet<Index>,
}
impl NullRequests {
    pub(crate) fn new() -> Self {
        Self {
            requests: HashSet::new(),
        }
    }
}
#[derive(Resource)]
pub(crate) struct CacheCheckRequests<
    Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static,
> {
    pub(crate) requests: HashSet<Key>,
}
impl<Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static> CacheCheckRequests<Key> {
    pub(crate) fn new() -> Self {
        Self {
            requests: HashSet::new(),
        }
    }
}
// track what keys coordinator user uses to send removes / requests appropriately
#[derive(Component, Resource)]
pub(crate) struct CachedKeys<Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static> {
    pub(crate) used_keys: HashSet<Key>,
}
impl<Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static> CachedKeys<Key> {
    pub(crate) fn new() -> Self {
        Self {
            used_keys: HashSet::new(),
        }
    }
}
pub struct Coordinator<
    Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static,
    RequestData: 'static,
> {
    // open slot policy - nullable - shrink occasionally
    pub(crate) task: Task,
    _key: PhantomData<Key>,
    _request: PhantomData<RequestData>,
}
impl<
        Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static,
        RequestData: Send + Sync + 'static,
    > Coordinator<Key, RequestData>
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
                task.container
                    .insert_resource(IndexHandler::<Key>::new(max));
                task.container
                    .insert_resource(RequestHandler::<Key, RequestData>::new());
                task.container.insert_resource(RemoveHandler::<Key>::new());
                task.container.insert_resource(WriteRequests::<Key>::new());
                task.container.insert_resource(NullRequests::new());
                task.container
                    .insert_resource(CacheCheckRequests::<Key>::new());
                task
            },
            _key: PhantomData,
            _request: PhantomData,
        }
    }
    pub fn setup_attribute<Attribute: AttributeExtractor<RequestData>>(
        &mut self,
        device: &wgpu::Device,
    ) {
        self.task
            .main
            .schedule
            .add_system_to_stage(Stage::During, Self::process_attribute::<Attribute>);
        self.task
            .container
            .insert_resource(attribute::CpuBuffer::<Attribute>::new(self.max()));
        self.task
            .container
            .insert_resource(attribute::gpu_buffer::<Attribute>(device, self.max()));
    }
    fn start_processing(
        requests: Res<RequestHandler<Key, RequestData>>,
        removes: Res<RemoveHandler<Key>>,
        mut index_handler: ResMut<IndexHandler<Key>>,
        mut write_requests: ResMut<WriteRequests<Key>>,
        mut null_requests: ResMut<NullRequests>,
        mut cache_check_requests: ResMut<CacheCheckRequests<Key>>,
    ) {
        for key in removes.removes.iter() {
            let removed_index = index_handler.remove(key);
            if let Some(index) = removed_index {
                null_requests.requests.insert(index);
            }
        }
        for request in requests.requests.iter() {
            match index_handler.exists(request.key) {
                true => {
                    cache_check_requests.requests.insert(request.key);
                }
                false => {
                    let _index = index_handler.next(request.key);
                    write_requests.requests.insert(request.key);
                }
            }
        }
    }
    fn process_attribute<Attribute: AttributeExtractor<RequestData>>(
        requests: Res<RequestHandler<Key, RequestData>>,
        mut index_handler: ResMut<IndexHandler<Key>>,
        mut write_requests: ResMut<WriteRequests<Key>>,
        mut null_requests: ResMut<NullRequests>,
        mut cache_check_requests: ResMut<CacheCheckRequests<Key>>,
        cpu_buffer: ResMut<CpuBuffer<Attribute>>,
        gpu_buffer: ResMut<GpuBuffer<Attribute>>,
    ) {
        // iter cache checks
        // iter write requests to resolve writes
        // combine write ranges
        // if grown - write all cpu else - write write ranges
    }
    fn finish_processing() {
        // clear and reset
    }
    pub fn gpu_buffer<Attribute: AttributeExtractor<RequestData>>(&self) -> &wgpu::Buffer {
        &self
            .task
            .container
            .get_resource::<GpuBuffer<Attribute>>()
            .expect("no gpu buffer attached")
            .buffer
    }
    pub fn coordinate(&mut self) {
        self.task.exec(WorkloadId::Main);
    }
    pub fn count(&self) -> usize {
        self.task
            .container
            .get_resource::<IndexHandler<Key>>()
            .expect("no index handler attached")
            .count()
    }
    pub fn max(&self) -> usize {
        self.task
            .container
            .get_resource::<IndexHandler<Key>>()
            .expect("no index handler attached")
            .max()
    }
    pub fn has_instances(&self) -> bool {
        self.count() > 0
    }
}
