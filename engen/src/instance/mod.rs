use crate::instance::attribute::attribute_size;
use anymap::AnyMap;
use attribute::AttributeBuffer;
use bevy_ecs::prelude::Entity;
use indexer::{Index, Indexer};
use itertools::Itertools;
use std::cmp::Ordering;
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
    pub(crate) gpu_buffers: AnyMap,
    pub(crate) cpu_buffers: AnyMap,
    pub(crate) writes: AnyMap,
    pub(crate) write_requests: HashSet<Key>,
    pub(crate) removes: HashSet<Index>,
    pub(crate) requests: HashMap<Key, InstanceRequest>,
    pub(crate) indices: HashMap<Key, Index>,
    pub(crate) grown: bool,
}
impl<Key: Eq + Hash + PartialEq + Copy + Clone + 'static, InstanceRequest>
    Coordinator<Key, InstanceRequest>
{
    pub(crate) fn new(initial_max: usize) -> Self {
        Self {
            indexer: Indexer::new(initial_max),
            gpu_buffers: AnyMap::new(),
            cpu_buffers: AnyMap::new(),
            writes: AnyMap::new(),
            write_requests: HashSet::new(),
            removes: HashSet::new(),
            requests: HashMap::new(),
            indices: HashMap::new(),
            grown: false,
        }
    }
    pub(crate) fn setup_attribute<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        &mut self,
        device: &wgpu::Device,
    ) {
        self.gpu_buffers
            .insert(attribute::gpu_buffer::<Attribute>(device, self.indexer.max));
        self.cpu_buffers
            .insert(attribute::cpu_buffer::<Attribute>(self.indexer.max));
        self.writes.insert(HashMap::<Key, Attribute>::new());
    }
    pub(crate) fn attribute_buffer<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        &self,
    ) -> &wgpu::Buffer {
        &self
            .gpu_buffers
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
    pub(crate) fn prepare(&mut self, device: &wgpu::Device) {
        // remove + swap
        for (key, request) in self.requests.iter() {
            if !self.indices.contains_key(key) {
                let index = self.indexer.next();
                self.indices.insert(*key, index);
            }
            self.write_requests.insert(*key);
        }
        // grow
        if self.indexer.should_grow() {
            self.grown = true;
        }
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
            let index = self.get_index(*key);
            let requested_value =
                fetcher(self.requests.get(key).as_ref().expect("no request for key"));
            let cached_value = self
                .cpu_buffers
                .get::<Vec<Attribute>>()
                .expect("no cache for attribute")
                .get(index.0);
            if let Some(value) = cached_value {
                if *value == requested_value {
                    continue;
                }
            }
            self.writes
                .get_mut::<HashMap<Key, Attribute>>()
                .expect("no write buffer for attribute")
                .insert(*key, requested_value);
            self.cpu_buffers
                .get_mut::<Vec<Attribute>>()
                .expect("no cpu buffer for attribute")
                .insert(index.0, requested_value);
        }
    }
    pub(crate) fn write<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        &mut self,
        queue: &wgpu::Queue,
    ) {
        if self.grown {
            self.grown = false;
        } else {
            // get indices of writes
            let indexed_writes = self.indexed_writes::<Attribute>();
            // combine sequential
            let mut combined_writes = Self::combined_writes(indexed_writes);
            // write the write_ranges from combine
            for range in combined_writes.drain(..) {
                range.write(queue, self.attribute_buffer::<Attribute>());
            }
        }
    }
    fn combined_writes<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        mut indexed_writes: Vec<(Index, Attribute)>,
    ) -> Vec<WriteRange<Attribute>> {
        let mut write_ranges = Vec::new();
        indexed_writes.sort_by(|lhs, rhs| -> Ordering {
            let right_index = rhs.0 .0;
            let left_index = lhs.0 .0;
            if left_index < right_index {
                return Ordering::Less;
            }
            if left_index > right_index {
                return Ordering::Greater;
            }
            Ordering::Equal
        });
        let mut consecutive_slices = consecutive_slices(indexed_writes);
        for slice in consecutive_slices.drain(..) {
            write_ranges.push(WriteRange::new(slice));
        }
        write_ranges
    }
    fn get_index(&self, key: Key) -> Index {
        *self.indices.get(&key).unwrap()
    }
    fn indexed_writes<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        &mut self,
    ) -> Vec<(Index, Attribute)> {
        self.writes
            .get_mut::<HashMap<Key, Attribute>>()
            .expect("no write map for attribute")
            .drain()
            .map(|(key, attr)| -> (Index, Attribute) {
                return (*self.indices.get(&key).unwrap(), attr);
            })
            .collect::<Vec<(Index, Attribute)>>()
    }
}
struct WriteRange<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
> {
    pub(crate) writes: Vec<(Index, Attribute)>,
}
impl<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    > WriteRange<Attribute>
{
    fn new(writes: Vec<(Index, Attribute)>) -> Self {
        Self { writes }
    }
    pub(crate) fn write(&self, queue: &wgpu::Queue, gpu_buffer: &wgpu::Buffer) {
        let offset = attribute_size::<Attribute>(
            self.writes.first().expect("no writes in write range").0 .0,
        );
        let write_data = self
            .writes
            .iter()
            .map(|write| -> Attribute { write.1 })
            .collect::<Vec<Attribute>>();
        queue.write_buffer(
            gpu_buffer,
            offset as wgpu::BufferAddress,
            bytemuck::cast_slice(&write_data),
        );
    }
}
fn consecutive_slices<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
>(
    data: Vec<(Index, Attribute)>,
) -> Vec<Vec<(Index, Attribute)>> {
    (&(0..data.len()).group_by(|&i| data[i].0 .0 - i))
        .into_iter()
        .map(|(_, group)| group.map(|i| data[i]).collect())
        .collect()
}
