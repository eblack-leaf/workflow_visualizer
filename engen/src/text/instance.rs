use crate::text::component::Key;
use crate::text::index::{Index, Indexer};
use crate::{Area, Canvas, Color, Depth, Position};
use std::collections::HashMap;
use wgpu::BufferAddress;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct Attributes {
    pub position: Position,
    pub depth: Depth,
    pub color: Color,
}

impl Attributes {
    pub fn new(position: Position, depth: Depth, color: Color) -> Self {
        Self {
            position,
            depth,
            color,
        }
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct TexCoords {
    pub coords: [f32; 4],
}

impl TexCoords {
    pub fn new(coords: [f32; 4]) -> Self {
        Self { coords }
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct Instance {
    pub attributes: Attributes,
    pub area: Area,
    pub tex_coords: TexCoords,
}

impl Instance {
    pub fn new(attributes: Attributes, area: Area, tex_coords: TexCoords) -> Self {
        Self {
            attributes,
            area,
            tex_coords,
        }
    }
    pub fn nullified() -> Self {
        Self {
            attributes: Attributes::new(Position::default(), Depth::default(), Color::default()),
            area: Area::default(),
            tex_coords: TexCoords::new([0.0, 0.0, 0.0, 0.0]),
        }
    }
}

pub(crate) struct InstanceBuffer {
    pub(crate) cpu: Vec<Instance>,
    pub(crate) gpu: wgpu::Buffer,
    pub(crate) indexer: Indexer<Key>,
    pub(crate) write: HashMap<Index, Instance>,
    pub(crate) keyed_indexes: HashMap<Key, Index>,
}

impl InstanceBuffer {
    pub(crate) fn update_non_attributes(&self, key: Key, area: Area, tex_coords: TexCoords) {
        todo!()
    }
    pub(crate) fn add(&mut self, key: Key, instance: Instance) {
        let index = self.indexer.next(key);
        self.keyed_indexes.insert(key, index);
        self.queue_write(index, instance);
    }
    pub(crate) fn write(&mut self, canvas: &Canvas) {
        for (index, instance) in self.write.iter() {
            self.cpu.insert(index.value as usize, *instance);
            let offset =
                (std::mem::size_of::<Instance>() as u32 * index.value) as wgpu::BufferAddress;
            canvas
                .queue
                .write_buffer(&self.gpu, offset, bytemuck::cast_slice(&[*instance]));
        }
    }
    pub(crate) fn update(&mut self, key: Key, attributes: Attributes) {
        let index = self.keyed_indexes.get(&key).expect("no index for key");
        let mut instance: Instance = *self
            .cpu
            .get(index.value as usize)
            .expect("no instance present");
        instance.attributes = attributes;
        self.queue_write(*index, instance);
    }
    pub(crate) fn remove(&mut self, key: Key) {
        let removed_index = self.indexer.remove(&key);
        if let Some(index) = removed_index {
            self.queue_write(index, Instance::nullified());
        }
    }
    pub(crate) fn queue_write(&mut self, index: Index, instance: Instance) {
        self.write.insert(index, instance);
    }
    pub(crate) fn new(canvas: &Canvas, initial_supported_instances: u32) -> Self {
        Self {
            cpu: Vec::new(),
            gpu: canvas.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("text instance buffer"),
                size: (std::mem::size_of::<Instance>() * initial_supported_instances as usize)
                    as BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            indexer: Indexer::new(initial_supported_instances),
            write: HashMap::new(),
            keyed_indexes: HashMap::new(),
        }
    }
    pub(crate) fn count(&self) -> usize {
        self.cpu.len()
    }
}
