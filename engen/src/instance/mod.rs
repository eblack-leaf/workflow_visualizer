use crate::canvas::Canvas;
use crate::instance::attribute::{attribute_size, gpu_buffer};
use crate::instance::indexer::IndexedAttribute;
use anymap::AnyMap;
use attribute::AttributeBuffer;
use bevy_ecs::prelude::Entity;
use indexer::{Index, Indexer};
use itertools::Itertools;
pub(crate) use key::EntityKey;
use key::IndexedKey;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use write_range::WriteRange;
mod attribute;
mod indexer;
mod key;
mod write_range;

pub(crate) struct Swap<Key: Eq + Hash + PartialEq + Copy + Clone> {
    pub(crate) src: IndexedKey<Key>,
    pub(crate) dest: IndexedKey<Key>,
}
impl<Key: Eq + Hash + PartialEq + Copy + Clone> Swap<Key> {
    pub(crate) fn new(src: IndexedKey<Key>, dest: IndexedKey<Key>) -> Self {
        Self { src, dest }
    }
}
pub(crate) struct SwapRange<Key: Eq + Hash + PartialEq + Copy + Clone> {
    pub(crate) range: Vec<IndexedKey<Key>>,
}
impl<Key: Eq + Hash + PartialEq + Copy + Clone> SwapRange<Key> {
    pub(crate) fn new(range: Vec<IndexedKey<Key>>) -> Self {
        Self { range }
    }
}
pub(crate) struct Coordinator<Key: Eq + Hash + PartialEq + Copy + Clone, InstanceRequest> {
    pub(crate) indexer: Indexer,
    pub(crate) gpu_buffers: AnyMap,
    pub(crate) cpu_buffers: AnyMap,
    pub(crate) writes: AnyMap,
    pub(crate) write_requests: HashSet<Key>,
    pub(crate) removes: HashSet<Key>,
    pub(crate) swap_ranges: Vec<SwapRange<Key>>,
    pub(crate) requests: HashMap<Key, InstanceRequest>,
    pub(crate) indices: HashMap<Key, Index>,
    pub(crate) growth: Option<usize>,
    pub(crate) swaps: Vec<Swap<Key>>,
    pub(crate) keys: HashMap<Index, Key>,
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
            keys: HashMap::new(),
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
    pub(crate) fn current(&self) -> usize {
        self.indexer.current
    }
    pub(crate) fn prepare(&mut self, device: &wgpu::Device) {
        self.remove();
        self.prepare_swaps();
        self.prepare_writes();
        self.prepare_growth();
    }
    pub(crate) fn process_attribute<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
        Fetcher,
    >(
        &mut self,
        canvas: &Canvas,
        fetcher: Fetcher,
    ) where
        Fetcher: Fn(&InstanceRequest) -> Attribute + 'static,
    {
        self.swap::<Attribute>();
        self.resolve::<Attribute, _>(fetcher);
        self.write::<Attribute>(canvas);
    }
    pub(crate) fn finish(&mut self) {
        let _ = self.growth.take();
        self.write_requests.clear();
        self.requests.clear();
        for swap in self.swaps.drain(..) {
            self.indices.remove(&swap.src.key);
            self.keys.remove(&swap.src.index);
        }
        self.removes.clear();
    }
}
impl<Key: Eq + Hash + PartialEq + Copy + Clone + 'static, InstanceRequest>
    Coordinator<Key, InstanceRequest>
{
    fn max(&self) -> usize {
        self.indexer.max
    }
    fn prepare_growth(&mut self) {
        if self.indexer.should_grow() {
            self.growth.replace(self.indexer.grow(self.growth_factor()));
        }
    }
    fn prepare_writes(&mut self) {
        for (key, request) in self.requests.iter() {
            if !self.indices.contains_key(key) {
                let index = self.indexer.next();
                self.indices.insert(*key, index);
                self.keys.insert(index, *key);
            }
            self.write_requests.insert(*key);
        }
    }
    fn indexed_removals(&self) -> Vec<IndexedKey<Key>> {
        let mut indexed_removals = Vec::new();
        for key in self.removes.iter() {
            let index = self.get_index(*key);
            indexed_removals.push(IndexedKey::new(*key, index));
        }
        indexed_removals
    }
    fn remove(&mut self) {
        let mut indexed_removals = self.indexed_removals();
        indexed_removals.sort_by(|lhs, rhs| -> Ordering {
            if lhs.index.0 < rhs.index.0 {
                return Ordering::Less;
            }
            if lhs.index.0 > rhs.index.0 {
                return Ordering::Greater;
            }
            Ordering::Equal
        });
        let mut remove_ranges = Self::consecutive_removes(indexed_removals);
        remove_ranges.retain(|range| {
            let mut contains = false;
            for indexed_key in range.iter() {
                if indexed_key.index == self.indexer.current_index() {
                    contains = true;
                }
            }
            if contains {
                for indexed_key in range.iter() {
                    Self::remove_attribute(&mut self.cpu_buffers, indexed_key.index);
                    self.indexer.decrement();
                    self.indices.remove(indexed_key.key);
                    self.keys.remove(&indexed_key.index);
                }
            }
            !contains
        });
        self.swap_ranges = remove_ranges
            .drain(..)
            .map(|range| SwapRange::new(range))
            .collect();
    }
    fn prepare_swaps(&mut self) {
        let mut keys_to_swap = HashSet::new();
        for swap_range in self.swap_ranges.iter() {
            for indexed_key in swap_range.range.iter() {
                keys_to_swap.insert(indexed_key.key);
            }
        }
        let mut swap_ranges = self.swap_ranges.drain(..).collect::<Vec<SwapRange<Key>>>();
        for swap_range in swap_ranges.iter() {
            for indexed_key in swap_range.range.iter() {
                if self.indexer.current == 1 {}
                let mut src = self.indexer.decrement();
                let mut src_key = self.get_key(src);
                while keys_to_swap.contains(src_key) {
                    src -= 1;
                    src_key = self.get_key(src);
                }
                let dest = self.get_index(*key);
                self.swaps.push(Swap::new(
                    IndexedKey::new(src_key, src),
                    IndexedKey::new(*key, dest),
                ));
            }
        }
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
    fn get_key(&self, index: Index) -> Key {
        *self.keys.get(&index).expect("no key for index")
    }
    fn swap<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        &mut self,
    ) {
        for swap in self.swaps.iter() {
            let index = *self.indices.get(&swap.src.key).expect("no index present");
            let attribute: Attribute = self.get_attribute(index);
            Self::send_write(&mut self.writes, swap.dest.key, attribute);
            Self::set_attribute(&mut self.cpu_buffers, swap.dest.index, attribute);
            Self::remove_attribute::<Attribute>(&mut self.cpu_buffers, swap.src.index);
        }
    }

    fn get_attribute<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        &self,
        index: Index,
    ) -> Attribute {
        *self
            .cpu_buffers
            .get::<Vec<Attribute>>()
            .expect("no cpu buffer for attribute")
            .get(index.0)
            .expect("invalid index")
    }
    fn remove_attribute<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        cpu_buffers: &mut AnyMap,
        index: Index,
    ) {
        cpu_buffers
            .get_mut::<Vec<Attribute>>()
            .expect("no cpu buffer for attribute")
            .remove(index.0);
    }
    fn resolve<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
        Fetcher,
    >(
        &mut self,
        fetcher: Fetcher,
    ) where
        Fetcher: Fn(&InstanceRequest) -> Attribute + 'static,
    {
        if let Some(growth_amount) = self.growth {
            self.grow_cpu_buffer::<Attribute>(growth_amount);
        }
        for key in self.write_requests.iter() {
            let index = self.get_index(*key);
            let requested_value =
                fetcher(self.requests.get(key).as_ref().expect("no request for key"));
            let cached_value: Option<Attribute> = self.get_cached_value::<Attribute>(index);
            if let Some(value) = cached_value {
                if value == requested_value {
                    continue;
                }
            }
            Self::send_write(&mut self.writes, *key, requested_value);
            Self::set_attribute(&mut self.cpu_buffers, index, requested_value);
        }
    }

    fn get_cached_value<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        &self,
        index: Index,
    ) -> Option<Attribute> {
        self.cpu_buffers
            .get::<Vec<Attribute>>()
            .expect("no cache for attribute")
            .get(index.0)
            .copied()
    }

    fn grow_cpu_buffer<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        &mut self,
        growth_amount: usize,
    ) {
        self.cpu_buffers
            .get_mut::<Vec<Attribute>>()
            .expect("no cpu buffer for attribute")
            .resize(growth_amount, Attribute::default());
    }

    fn set_attribute<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        cpu_buffers: &mut AnyMap,
        index: Index,
        requested_value: Attribute,
    ) {
        cpu_buffers
            .get_mut::<Vec<Attribute>>()
            .expect("no cpu buffer for attribute")
            .insert(index.0, requested_value);
    }

    fn send_write<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        writes: &mut AnyMap,
        key: Key,
        requested_value: Attribute,
    ) {
        writes
            .get_mut::<HashMap<Key, Attribute>>()
            .expect("no write buffer for attribute")
            .insert(key, requested_value);
    }
    fn write<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        &mut self,
        canvas: &Canvas,
    ) {
        if let Some(growth_amount) = self.growth.as_ref() {
            self.gpu_buffers
                .insert(gpu_buffer::<Attribute>(&canvas.device, self.max()));
            canvas.queue.write_buffer(
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
                range.write(&canvas.queue, self.gpu_buffer::<Attribute>());
            }
        }
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
        let mut consecutive_slices = Self::consecutive_indexed_attributes(indexed_writes);
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
    fn consecutive_indexed_attributes<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    >(
        data: Vec<IndexedAttribute<Attribute>>,
    ) -> Vec<Vec<IndexedAttribute<Attribute>>> {
        (&(0..data.len()).group_by(|&i| data[i].index.0 - i))
            .into_iter()
            .map(|(_, group)| group.map(|i| data[i]).collect())
            .collect()
    }
    fn consecutive_removes(data: Vec<IndexedKey<Key>>) -> Vec<Vec<IndexedKey<Key>>> {
        (&(0..data.len()).group_by(|&i| data[i].index.0 - i))
            .into_iter()
            .map(|(_, group)| group.map(|i| data[i]).collect())
            .collect()
    }
}
