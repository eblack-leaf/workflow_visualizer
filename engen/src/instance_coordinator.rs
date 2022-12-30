use anymap::AnyMap;
use bevy_ecs::prelude::{Commands, Entity, NonSendMut, Query};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::marker::PhantomData;
use wgpu::BufferAddress;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct Index(pub(crate) usize);
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct EntityKey<Identifier: Eq + Hash + PartialEq + Copy + Clone> {
    pub(crate) entity: Entity,
    pub(crate) identifier: Identifier,
}
pub(crate) struct InstanceCoordinator<Key: Eq + Hash + PartialEq + Copy + Clone, InstanceRequest> {
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
    InstanceCoordinator<Key, InstanceRequest>
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
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
    >(
        &mut self,
        device: &wgpu::Device,
    ) {
        self.attribute_buffers
            .insert(attribute_buffer::<Attribute>(device, self.indexer.max));
        self.cpu_attribute_buffers
            .insert(cpu_attribute_buffer::<Attribute>(self.indexer.max));
        self.attribute_cache
            .insert(HashMap::<Key, Attribute>::new());
        self.writes.insert(HashMap::<Key, Attribute>::new());
    }
    pub(crate) fn attribute_buffer<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
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
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
        Fetcher,
    >(
        &mut self,
        fetcher: Fetcher,
    ) where
        Fetcher: Fn(InstanceRequest) -> Attribute + 'static,
    {
        // go through write_requests and check cached values to determine if should go to writes

        // add to cached attribute values
    }
    pub(crate) fn write<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
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
pub(crate) struct Indexer {
    pub(crate) current: usize,
    pub(crate) max: usize,
}
impl Indexer {
    pub(crate) fn new(max: usize) -> Self {
        Self { current: 0, max }
    }
    pub(crate) fn next(&mut self) -> Index {
        self.current += 1;
        Index(self.current)
    }
    pub(crate) fn decrement(&mut self) {
        self.current -= 1;
    }
    pub(crate) fn should_grow(&self) -> bool {
        self.current > self.max
    }
}
pub(crate) struct AttributeBuffer<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
> {
    pub(crate) buffer: wgpu::Buffer,
    _phantom_data: PhantomData<Attribute>,
}
impl<Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default>
    AttributeBuffer<Attribute>
{
    pub(crate) fn new(buffer: wgpu::Buffer) -> Self {
        Self {
            buffer,
            _phantom_data: PhantomData,
        }
    }
}
pub(crate) fn attribute_buffer<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
>(
    device: &wgpu::Device,
    max_instances: usize,
) -> AttributeBuffer<Attribute> {
    AttributeBuffer::new(device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("attribute buffer"),
        size: attribute_size::<Attribute>(max_instances) as BufferAddress,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
}
pub(crate) fn attribute_size<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
>(
    num: usize,
) -> usize {
    std::mem::size_of::<Attribute>() * num
}
pub(crate) fn cpu_attribute_buffer<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
>(
    initial_max: usize,
) {
    let mut buffer = Vec::new();
    buffer.resize(initial_max, Attribute::default());
}
