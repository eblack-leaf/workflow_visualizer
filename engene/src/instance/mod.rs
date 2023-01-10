mod attribute;
mod index;
mod key;

pub(crate) use crate::instance::attribute::{CpuBuffer, GpuBuffer};
use crate::instance::index::Index;
use crate::task::Container;
use crate::Canvas;
pub use attribute::AttributeHandler;
use bevy_ecs::prelude::{Component, Resource};
pub(crate) use index::IndexHandler;
use iter_tools::Itertools;
pub use key::EntityKey;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::marker::PhantomData;
pub struct Request<RequestData: Send + Sync + 'static> {
    pub data: RequestData,
}
pub(crate) struct RequestHandler<
    Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static,
    RequestData: Send + Sync + 'static,
> {
    pub(crate) requests: HashMap<Key, Request<RequestData>>,
}
impl<
        Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static,
        Request: Send + Sync + 'static,
    > RequestHandler<Key, Request>
{
    pub(crate) fn new() -> Self {
        Self {
            requests: HashMap::new(),
        }
    }
}
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
pub(crate) struct Growth {
    pub(crate) new_max: Option<usize>,
    pub(crate) growth_factor: usize,
}
impl Growth {
    pub(crate) fn new(growth_factor: usize) -> Self {
        Self {
            new_max: None,
            growth_factor,
        }
    }
}
pub struct BufferCoordinator<
    Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static,
    RequestData: Send + Sync + 'static,
> {
    // open slot policy - nullable - shrink occasionally
    pub(crate) container: Container,
    pub(crate) index_handler: IndexHandler<Key>,
    pub(crate) request_handler: RequestHandler<Key, RequestData>,
    pub(crate) remove_handler: RemoveHandler<Key>,
    pub(crate) write_requests: WriteRequests<Key>,
    pub(crate) null_requests: NullRequests,
    pub(crate) cache_check_requests: CacheCheckRequests<Key>,
    pub(crate) growth: Growth,
    _key: PhantomData<Key>,
    _request: PhantomData<RequestData>,
}
impl<
        Key: Eq + Hash + PartialEq + Copy + Clone + Send + Sync + 'static,
        RequestData: Send + Sync + 'static,
    > BufferCoordinator<Key, RequestData>
{
    pub fn new(max: usize) -> Self {
        Self {
            container: Container::new(),
            index_handler: IndexHandler::<Key>::new(max),
            request_handler: RequestHandler::<Key, RequestData>::new(),
            remove_handler: RemoveHandler::<Key>::new(),
            write_requests: WriteRequests::<Key>::new(),
            null_requests: NullRequests::new(),
            cache_check_requests: CacheCheckRequests::<Key>::new(),
            growth: Growth::new(5),
            _key: PhantomData,
            _request: PhantomData,
        }
    }
    pub fn setup_attribute<Attribute: AttributeHandler<RequestData>>(
        &mut self,
        device: &wgpu::Device,
    ) {
        self.container
            .insert_resource(CpuBuffer::<Attribute>::new(self.max()));
        self.container
            .insert_resource(GpuBuffer::<Attribute>::new(device, self.max()));
    }
    pub fn start(&mut self) {
        for key in self.remove_handler.removes.iter() {
            let removed_index = self.index_handler.remove(key);
            if let Some(index) = removed_index {
                self.null_requests.requests.insert(index);
            }
        }
        for (key, _request) in self.request_handler.requests.iter() {
            match self.index_handler.exists(*key) {
                true => {
                    self.cache_check_requests.requests.insert(*key);
                }
                false => {
                    let _index = self.index_handler.next(*key);
                    self.write_requests.requests.insert(*key);
                }
            }
        }
        if self.index_handler.should_grow() {
            self.index_handler.grow(self.growth.growth_factor);
            self.growth.new_max.replace(self.index_handler.max());
        }
    }
    fn cpu_buffer<Attribute: AttributeHandler<RequestData>>(&self) -> &CpuBuffer<Attribute> {
        self.container
            .get_resource::<CpuBuffer<Attribute>>()
            .expect("no cpu buffer attached")
    }
    pub fn prepare<Attribute: AttributeHandler<RequestData>>(&mut self, canvas: &Canvas) {
        for index in self.null_requests.requests.iter() {
            self.container
                .get_resource_mut::<CpuBuffer<Attribute>>()
                .expect("")
                .buffer
                .insert(index.value, Attribute::null())
        }
        // iter cache checks
        for key in self.cache_check_requests.requests.iter() {
            let cached_value = *self
                .container
                .get_resource::<CpuBuffer<Attribute>>()
                .expect("")
                .buffer
                .get(
                    self.index_handler
                        .get_index(*key)
                        .expect("no index for key")
                        .value,
                )
                .expect("no cached value");
            let requested_value = Attribute::extract(
                &self
                    .request_handler
                    .requests
                    .get(key)
                    .expect("no requested value")
                    .data,
            );
            if cached_value != requested_value {
                self.write_requests.requests.insert(*key);
            }
        }
        if let Some(new_max) = self.growth.new_max {
            // grown so remake buffer and write all cpu write requests then gpu all cpu
            self.container
                .insert_resource(GpuBuffer::<Attribute>::new(&canvas.device, new_max));
            for key in self.write_requests.requests.iter() {
                // write to cpu
                let index = self
                    .index_handler
                    .get_index(*key)
                    .expect("no index for key");
                let attribute = Attribute::extract(
                    &self
                        .request_handler
                        .requests
                        .get(key)
                        .expect("no request data")
                        .data,
                );
                self.container
                    .get_resource_mut::<CpuBuffer<Attribute>>()
                    .expect("no cpu buffer attached")
                    .buffer
                    .insert(index.value, attribute);
            }
            canvas.queue.write_buffer(
                self.gpu_buffer::<Attribute>(),
                0,
                bytemuck::cast_slice(&self.cpu_buffer::<Attribute>().buffer),
            );
        } else {
            // iter write requests to resolve writes
            let mut writes = Vec::<(Index, Attribute)>::new();
            for key in self.write_requests.requests.iter() {
                let write_value = Attribute::extract(
                    &self
                        .request_handler
                        .requests
                        .get(key)
                        .expect("no requested_value")
                        .data,
                );
                let index = self.index_handler.get_index(*key).expect("key not indexed");
                writes.push((index, write_value));
            }
            // combine write ranges
            let combined_writes = Self::combine(writes);
            // write all ranges to cpu / gpu
            for range in combined_writes.iter() {
                for write in range {
                    self.container
                        .get_resource_mut::<CpuBuffer<Attribute>>()
                        .expect("")
                        .buffer
                        .insert(write.0.value, write.1);
                }
                let first_index = range.first().expect("no writes in range");
                let offset = attribute::attribute_size::<Attribute>(first_index.0.value);
                let attributes = range
                    .iter()
                    .map(|write| -> Attribute { write.1 })
                    .collect::<Vec<Attribute>>();
                // write to gpu_buffer
                canvas.queue.write_buffer(
                    self.gpu_buffer::<Attribute>(),
                    offset as wgpu::BufferAddress,
                    bytemuck::cast_slice(&attributes),
                );
            }
        }
    }
    pub fn finish(&mut self) {
        // clear and reset
        self.growth.new_max.take();
        self.request_handler.requests.clear();
        self.remove_handler.removes.clear();
        self.write_requests.requests.clear();
        self.null_requests.requests.clear();
        self.cache_check_requests.requests.clear();
    }
    fn combine<Attribute: AttributeHandler<RequestData>>(
        mut writes: Vec<(Index, Attribute)>,
    ) -> Vec<Vec<(Index, Attribute)>> {
        writes.sort_by(|lhs, rhs| -> Ordering { lhs.0.value.cmp(&rhs.0.value) });
        (&(0..writes.len()).group_by(|&i| writes[i].0.value - i))
            .into_iter()
            .map(|(_, group)| group.map(|i| writes[i]).collect())
            .collect()
    }
    pub fn gpu_buffer<Attribute: AttributeHandler<RequestData>>(&self) -> &wgpu::Buffer {
        &self
            .container
            .get_resource::<GpuBuffer<Attribute>>()
            .expect("no gpu buffer attached")
            .buffer
    }
    pub fn count(&self) -> usize {
        self.index_handler.count()
    }
    pub fn max(&self) -> usize {
        self.index_handler.max()
    }
    pub fn has_instances(&self) -> bool {
        self.count() > 0
    }
}
