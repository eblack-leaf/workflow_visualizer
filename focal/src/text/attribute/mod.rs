use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::text::attribute::buffer::Buffer;
use crate::text::attribute::instance::Instance;
use crate::text::rasterization;
use crate::text::rasterization::{GlyphHash, Placement};
use add::Add;
use anymap::AnyMap;
use instance::Indexer;
use std::collections::{HashMap, HashSet};
use wgpu::BufferAddress;

mod add;
mod buffer;
mod instance;
mod write;

pub(crate) struct Coordinator {
    pub(crate) indexer: Indexer,
    pub(crate) adds: Vec<Add>,
    pub(crate) rasterization_requests: Vec<rasterization::Add>,
    pub(crate) rasterization_response_listeners: HashSet<(Instance, GlyphHash)>,
    pub(crate) buffers: AnyMap,
    pub(crate) attribute_adds: AnyMap,
    pub(crate) attribute_updates: AnyMap,
    pub(crate) removes: HashSet<Instance>,
}
impl Coordinator {
    pub(crate) fn new(device: &wgpu::Device) -> Self {
        let indexer = Indexer::new(10);
        let buffers = {
            let mut map = AnyMap::new();
            map.insert(buffer::buffer::<Position>(&device, indexer.max));
            map.insert(buffer::buffer::<Area>(&device, indexer.max));
            map.insert(buffer::buffer::<Depth>(&device, indexer.max));
            map.insert(buffer::buffer::<Color>(&device, indexer.max));
            map.insert(buffer::buffer::<Placement>(&device, indexer.max));
            map
        };
        let attribute_adds = {
            let mut map = AnyMap::new();
            map.insert(Vec::<Position>::new());
            map.insert(Vec::<Area>::new());
            map.insert(Vec::<Depth>::new());
            map.insert(Vec::<Color>::new());
            map.insert(Vec::<rasterization::Placement>::new());
            map
        };
        let attribute_updates = {
            let mut map = AnyMap::new();
            map.insert(HashMap::<Instance, Position>::new());
            map.insert(HashMap::<Instance, Area>::new());
            map.insert(HashMap::<Instance, Depth>::new());
            map.insert(HashMap::<Instance, Color>::new());
            map.insert(HashMap::<Instance, rasterization::Placement>::new());
            map
        };
        Self {
            indexer,
            adds: Vec::new(),
            rasterization_requests: Vec::new(),
            rasterization_response_listeners: HashSet::new(),
            buffers,
            attribute_adds,
            attribute_updates,
            removes: HashSet::new(),
        }
    }
    pub(crate) fn max(&self) -> u32 {
        self.indexer.max
    }
    pub(crate) fn current(&self) -> u32 {
        self.indexer.current
    }
    pub(crate) fn buffer<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
    >(
        &self,
    ) -> &Buffer<Attribute> {
        self.buffers.get::<Buffer<Attribute>>().unwrap()
    }
}
