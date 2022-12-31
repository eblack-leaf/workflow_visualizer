use anymap::AnyMap;
use attribute::AttributeBuffer;
use bevy_ecs::prelude::Entity;
use indexer::{Index, Indexer};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

mod attribute;
mod indexer;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct EntityKey<Identifier: Eq + Hash + PartialEq + Copy + Clone> {
    pub(crate) entity: Entity,
    pub(crate) identifier: Identifier,
}
pub(crate) struct Coordinator<Key: Eq + Hash + PartialEq + Copy + Clone, InstanceRequest> {
    pub(crate) indexer: Indexer,
    pub(crate) attribute_buffers: AnyMap,
    pub(crate) cpu_attribute_buffers: AnyMap,
    pub(crate) writes: AnyMap,
    pub(crate) write_requests: HashSet<Key>,
    pub(crate) removes: HashSet<Index>,
    pub(crate) requests: HashMap<Key, InstanceRequest>,
    pub(crate) indices: HashMap<Key, Index>,
    pub(crate) attribute_cache: AnyMap,
}
impl<Key: Eq + Hash + PartialEq + Copy + Clone + 'static, InstanceRequest>
    Coordinator<Key, InstanceRequest>
{
    pub(crate) fn new(initial_max: usize) -> Self {
        Self {
            indexer: Indexer::new(initial_max),
            attribute_buffers: AnyMap::new(),
            cpu_attribute_buffers: AnyMap::new(),
            writes: AnyMap::new(),
            write_requests: HashSet::new(),
            removes: HashSet::new(),
            requests: HashMap::new(),
            indices: HashMap::new(),
            attribute_cache: AnyMap::new(),
        }
    }
    pub(crate) fn setup_attribute<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        &mut self,
        device: &wgpu::Device,
    ) {
        self.attribute_buffers
            .insert(attribute::gpu_buffer::<Attribute>(device, self.indexer.max));
        self.cpu_attribute_buffers
            .insert(attribute::cpu_buffer::<Attribute>(self.indexer.max));
        self.attribute_cache
            .insert(HashMap::<Key, Attribute>::new());
        self.writes.insert(HashMap::<Key, Attribute>::new());
    }
    pub(crate) fn attribute_buffer<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        &self,
    ) -> &wgpu::Buffer {
        &self
            .attribute_buffers
            .get::<AttributeBuffer<Attribute>>()
            .unwrap()
            .buffer
    }
    pub(crate) fn max(&self) -> usize {
        self.indexer.max
    }
    pub(crate) fn current(&self) -> usize {
        self.indexer.current
    }
    pub(crate) fn process_attribute<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
        Fetcher,
    >(
        &mut self,
        fetcher: Fetcher,
    ) where
        Fetcher: Fn(&InstanceRequest) -> Attribute + 'static,
    {
        // go through write_requests and check cached values to determine if should go to writes
        for key in self.write_requests.iter() {
            let requested_value = fetcher(self.requests.get(key).as_ref().expect("no request for key"));
            let cached_value = self
                .attribute_cache
                .get::<HashMap<Key, Attribute>>()
                .expect("no cache for attribute")
                .get(key);
            if let Some(value) = cached_value {
                if *value == requested_value {
                    continue;
                }
            }
            self.writes
                .get_mut::<HashMap<Key, Attribute>>()
                .expect("no write buffer for attribute")
                .insert(
                    *key,
                    requested_value,
                );
            self.attribute_cache
                .get_mut::<HashMap<Key, Attribute>>()
                .expect("no cache for attribute")
                .insert(*key, requested_value);
        }
    }
    pub(crate) fn write<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        &mut self,
    ) {
        // get indices of writes
        // combine sequential
        // if grown - write all of attribute buffer from cpu
    }
    fn get_index(&self, key: Key) -> Index {
        *self.indices.get(&key).unwrap()
    }
    pub(crate) fn prepare(&mut self) {
        // remove + swap
        for (key, request) in self.requests.iter() {
            if self.indices.contains_key(key) {
                /* make cache check with this key */
                /* this will remove the request if cached value is same */
            } else {
                let index = self.indexer.next();
                self.indices.insert(*key, index);
            }
            self.write_requests.insert(*key);
        }
        // grow
    }
}
