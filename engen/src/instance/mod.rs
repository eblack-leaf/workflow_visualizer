use crate::instance::attribute::{attribute_size, gpu_buffer};
use crate::instance::indexer::IndexedAttribute;
use anymap::AnyMap;
use attribute::AttributeBuffer;
use bevy_ecs::prelude::Entity;
use indexer::{Index, Indexer};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use write_range::WriteRange;

mod attribute;
mod indexer;
mod write_range;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct EntityKey<Identifier: Eq + Hash + PartialEq + Copy + Clone> {
    pub(crate) entity: Entity,
    pub(crate) identifier: Identifier,
}
pub(crate) struct Swap<Key: Eq + Hash + PartialEq + Copy + Clone> {
    pub(crate) src: (Key, Index),
    pub(crate) dest: Index,
}
impl<Key: Eq + Hash + PartialEq + Copy + Clone> Swap<Key> {
    pub(crate) fn new(src: (Key, Index), dest: Index) -> Self {
        Self { src, dest }
    }
}
pub(crate) struct Coordinator<Key: Eq + Hash + PartialEq + Copy + Clone, InstanceRequest> {
    pub(crate) indexer: Indexer,
    pub(crate) gpu_buffers: AnyMap,
    pub(crate) cpu_buffers: AnyMap,
    pub(crate) writes: AnyMap,
    pub(crate) write_requests: HashSet<Key>,
    pub(crate) removes: HashSet<Key>,
    pub(crate) requests: HashMap<Key, InstanceRequest>,
    pub(crate) indices: HashMap<Key, Index>,
    pub(crate) growth: Option<usize>,
    pub(crate) swaps: Vec<Swap<Key>>,
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
            growth: None,
            swaps: Vec::new(),
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
    pub(crate) fn gpu_buffer<
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
    fn cpu_buffer<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        &self,
    ) -> &Vec<Attribute> {
        &self
            .cpu_buffers
            .get::<Vec<Attribute>>()
            .expect("no cpu buffer for attribute")
    }
    pub(crate) fn max(&self) -> usize {
        self.indexer.max
    }
    pub(crate) fn current(&self) -> usize {
        self.indexer.current
    }
    pub(crate) fn prepare(&mut self, device: &wgpu::Device) {
        // remove + swap (just store indexes of changes + attr)
        for key in self.removes.iter() {
            let src = self.indexer.decrement();
            let dest = self.get_index(*key);
            self.swaps.push(Swap::new((*key, src), dest));
        }
        for (key, request) in self.requests.iter() {
            if !self.indices.contains_key(key) {
                let index = self.indexer.next();
                self.indices.insert(*key, index);
            }
            self.write_requests.insert(*key);
        }
        if self.indexer.should_grow() {
            self.growth.replace(self.indexer.grow(self.growth_factor()));
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
        if let Some(growth_amount) = self.growth.as_ref() {
            self.cpu_buffers
                .get_mut::<Vec<Attribute>>()
                .expect("no cpu buffer for attribute")
                .resize(*growth_amount, Attribute::default());
        }
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
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        if let Some(growth_amount) = self.growth.as_ref() {
            self.gpu_buffers
                .insert(gpu_buffer::<Attribute>(device, self.max()));
            queue.write_buffer(
                self.gpu_buffer::<Attribute>(),
                0,
                bytemuck::cast_slice(self.cpu_buffer::<Attribute>()),
            );
        } else {
            // get indices of writes
            let indexed_writes = self.indexed_writes::<Attribute>();
            // combine sequential
            let mut combined_writes = Self::combined_writes(indexed_writes);
            // write the write_ranges from combine
            for range in combined_writes.drain(..) {
                range.write(queue, self.gpu_buffer::<Attribute>());
            }
        }
    }
    pub(crate) fn finish(&mut self) {
        // clear buffers used in processing if not drained already
        let _ = self.growth.take();
    }
    fn growth_factor(&self) -> usize {
        10
    }
    fn combined_writes<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        mut indexed_writes: Vec<IndexedAttribute<Attribute>>,
    ) -> Vec<WriteRange<Attribute>> {
        let mut write_ranges = Vec::new();
        indexed_writes.sort_by(|lhs, rhs| -> Ordering {
            let right_index = rhs.index.0;
            let left_index = lhs.index.0;
            if left_index < right_index {
                return Ordering::Less;
            }
            if left_index > right_index {
                return Ordering::Greater;
            }
            Ordering::Equal
        });
        let mut consecutive_slices = Self::consecutive_slices(indexed_writes);
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
    ) -> Vec<IndexedAttribute<Attribute>> {
        self.writes
            .get_mut::<HashMap<Key, Attribute>>()
            .expect("no write map for attribute")
            .drain()
            .map(|(key, attr)| -> IndexedAttribute<Attribute> {
                return IndexedAttribute::new(*self.indices.get(&key).unwrap(), attr);
            })
            .collect::<Vec<IndexedAttribute<Attribute>>>()
    }
    fn consecutive_slices<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        data: Vec<IndexedAttribute<Attribute>>,
    ) -> Vec<Vec<IndexedAttribute<Attribute>>> {
        (&(0..data.len()).group_by(|&i| data[i].index.0 - i))
            .into_iter()
            .map(|(_, group)| group.map(|i| data[i]).collect())
            .collect()
    }
}
