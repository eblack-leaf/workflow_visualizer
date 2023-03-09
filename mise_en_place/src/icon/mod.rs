use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Entity, Resource};
use wgpu::{include_wgsl, VertexState};

pub(crate) use cache::{DifferenceHolder, Differences};
pub(crate) use instance::IconAdd;
pub(crate) use mesh::GpuIconMesh;
pub use mesh::{
    read_mesh, write_mesh, ColorHooks, ColorInvert, IconDescriptors, IconKey, IconMesh,
    IconMeshAddRequest, IconVertex,
};

use crate::coord::{GpuArea, GpuPosition};
use crate::gfx::Extract;
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration};
use crate::gfx::{Render, RenderPassHandle, RenderPhase};
pub use crate::icon::attachment::IconAttachment;
pub use crate::icon::interface::{Icon, IconBundle, IconSize};
use crate::instance::index::Indexer;
use crate::instance::key::{Key, KeyFactory};
use crate::instance::InstanceAttributeManager;
use crate::instance::NullBit;
use crate::{Area, Color, Depth, DeviceView, Job, Viewport};

mod attachment;
mod backend_system;
mod cache;
mod frontend_system;
mod instance;
mod interface;
mod mesh;
mod proc_gen;

#[derive(Resource)]
pub(crate) struct IconRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) icon_entities: HashMap<IconKey, HashSet<Entity>>,
    pub(crate) entity_icons: HashMap<Entity, IconKey>,
    pub(crate) entity_keys: HashMap<IconKey, HashMap<Entity, Key>>,
    pub(crate) meshes: HashMap<IconKey, GpuIconMesh>,
    pub(crate) position: HashMap<IconKey, InstanceAttributeManager<GpuPosition>>,
    pub(crate) area: HashMap<IconKey, InstanceAttributeManager<GpuArea>>,
    pub(crate) depth: HashMap<IconKey, InstanceAttributeManager<Depth>>,
    pub(crate) color: HashMap<IconKey, InstanceAttributeManager<Color>>,
    pub(crate) secondary_color: HashMap<IconKey, InstanceAttributeManager<Color>>,
    pub(crate) color_invert: HashMap<IconKey, InstanceAttributeManager<ColorInvert>>,
    pub(crate) null_bit: HashMap<IconKey, InstanceAttributeManager<NullBit>>,
    pub(crate) key_factory: HashMap<IconKey, KeyFactory>,
    pub(crate) indexer: HashMap<IconKey, Indexer<Key>>,
}

impl IconRenderer {
    pub(crate) fn add_mesh(
        &mut self,
        gfx_surface: &GfxSurface,
        icon_key: IconKey,
        mesh: GpuIconMesh,
        max: u32,
    ) {
        self.meshes.insert(icon_key, mesh);
        self.icon_entities.insert(icon_key, HashSet::new());
        self.position
            .insert(icon_key, InstanceAttributeManager::new(gfx_surface, max));
        self.area
            .insert(icon_key, InstanceAttributeManager::new(gfx_surface, max));
        self.depth
            .insert(icon_key, InstanceAttributeManager::new(gfx_surface, max));
        self.color
            .insert(icon_key, InstanceAttributeManager::new(gfx_surface, max));
        self.secondary_color
            .insert(icon_key, InstanceAttributeManager::new(gfx_surface, max));
        self.color_invert
            .insert(icon_key, InstanceAttributeManager::new(gfx_surface, max));
        self.null_bit
            .insert(icon_key, InstanceAttributeManager::new(gfx_surface, max));
        self.key_factory.insert(icon_key, KeyFactory::new());
        self.indexer.insert(icon_key, Indexer::new(max));
        self.entity_keys.insert(icon_key, HashMap::new());
    }
    pub(crate) fn add_icon(&mut self, entity: Entity, icon: IconAdd) {
        let key = self.key_factory.get_mut(&icon.key).unwrap().generate();
        self.entity_keys
            .get_mut(&icon.key)
            .unwrap()
            .insert(entity, key);
        self.icon_entities
            .get_mut(&icon.key)
            .unwrap()
            .insert(entity);
        self.entity_icons.insert(entity, icon.key);
        let index = self.indexer.get_mut(&icon.key).unwrap().next(key);
        self.position
            .get_mut(&icon.key)
            .unwrap()
            .write
            .write
            .insert(index, icon.panel.section.position.to_gpu());
        self.area
            .get_mut(&icon.key)
            .unwrap()
            .write
            .write
            .insert(index, icon.panel.section.area.to_gpu());
        self.depth
            .get_mut(&icon.key)
            .unwrap()
            .write
            .write
            .insert(index, icon.panel.depth);
        self.color
            .get_mut(&icon.key)
            .unwrap()
            .write
            .write
            .insert(index, icon.color);
        self.secondary_color
            .get_mut(&icon.key)
            .unwrap()
            .write
            .write
            .insert(index, icon.secondary_color);
        self.color_invert
            .get_mut(&icon.key)
            .unwrap()
            .write
            .write
            .insert(index, icon.color_invert);
        self.null_bit
            .get_mut(&icon.key)
            .unwrap()
            .write
            .write
            .insert(index, NullBit::not_null());
    }
    pub(crate) fn process_differences(
        &mut self,
        differences: &Differences,
        scale_factor: f64,
        gfx_surface: &GfxSurface,
    ) {
        for entity in differences.icon_removes.iter() {
            self.remove_icon(*entity);
        }
        for (entity, (key, position, area, depth, color, secondary_color, color_invert)) in
            differences.icon_adds.iter()
        {
            self.add_icon(
                *entity,
                IconAdd::new(
                    *key,
                    *position,
                    *area,
                    *depth,
                    *color,
                    *secondary_color,
                    *color_invert,
                    scale_factor,
                ),
            );
        }
        for (entity, pos) in differences.position.iter() {
            let icon_key = self.entity_icons.get(entity).unwrap();
            let key = self
                .entity_keys
                .get(&icon_key)
                .unwrap()
                .get(entity)
                .unwrap();
            let index = self
                .indexer
                .get(&icon_key)
                .unwrap()
                .get_index(*key)
                .unwrap();
            self.position
                .get_mut(&icon_key)
                .unwrap()
                .write
                .write
                .insert(index, pos.to_device(scale_factor).to_gpu());
        }
        for (entity, area) in differences.area.iter() {
            let icon_key = self.entity_icons.get(entity).unwrap();
            let key = self
                .entity_keys
                .get(&icon_key)
                .unwrap()
                .get(entity)
                .unwrap();
            let index = self
                .indexer
                .get(&icon_key)
                .unwrap()
                .get_index(*key)
                .unwrap();
            self.area.get_mut(&icon_key).unwrap().write.write.insert(
                index,
                Area::<DeviceView>::new(area.width, area.height).to_gpu(),
            );
        }
        for (entity, depth) in differences.depth.iter() {
            let icon_key = self.entity_icons.get(entity).unwrap();
            let key = self
                .entity_keys
                .get(&icon_key)
                .unwrap()
                .get(entity)
                .unwrap();
            let index = self
                .indexer
                .get(&icon_key)
                .unwrap()
                .get_index(*key)
                .unwrap();
            self.depth
                .get_mut(&icon_key)
                .unwrap()
                .write
                .write
                .insert(index, *depth);
        }
        for (entity, color) in differences.color.iter() {
            let icon_key = self.entity_icons.get(entity).unwrap();
            let key = self
                .entity_keys
                .get(&icon_key)
                .unwrap()
                .get(entity)
                .unwrap();
            let index = self
                .indexer
                .get(&icon_key)
                .unwrap()
                .get_index(*key)
                .unwrap();
            self.color
                .get_mut(&icon_key)
                .unwrap()
                .write
                .write
                .insert(index, *color);
        }
        for (entity, color) in differences.secondary_color.iter() {
            let icon_key = self.entity_icons.get(entity).unwrap();
            let key = self
                .entity_keys
                .get(&icon_key)
                .unwrap()
                .get(entity)
                .unwrap();
            let index = self
                .indexer
                .get(&icon_key)
                .unwrap()
                .get_index(*key)
                .unwrap();
            self.secondary_color
                .get_mut(&icon_key)
                .unwrap()
                .write
                .write
                .insert(index, *color);
        }
        for (entity, color_invert) in differences.color_invert.iter() {
            let icon_key = self.entity_icons.get(entity).unwrap();
            let key = self
                .entity_keys
                .get(&icon_key)
                .unwrap()
                .get(entity)
                .unwrap();
            let index = self
                .indexer
                .get(&icon_key)
                .unwrap()
                .get_index(*key)
                .unwrap();
            self.color_invert
                .get_mut(&icon_key)
                .unwrap()
                .write
                .write
                .insert(index, *color_invert);
        }
        self.grow(&gfx_surface);
        self.write(&gfx_surface);
    }
    pub(crate) fn grow(&mut self, gfx_surface: &GfxSurface) {
        let mut growth_requests = HashSet::new();
        for icon in self.icon_entities.iter() {
            if self.indexer.get_mut(icon.0).unwrap().should_grow() {
                growth_requests.insert(*icon.0);
            }
        }
        for icon_key in growth_requests {
            let new_max = self.indexer.get(&icon_key).unwrap().max();
            self.position
                .get_mut(&icon_key)
                .unwrap()
                .grow(gfx_surface, new_max);
            self.area
                .get_mut(&icon_key)
                .unwrap()
                .grow(gfx_surface, new_max);
            self.depth
                .get_mut(&icon_key)
                .unwrap()
                .grow(gfx_surface, new_max);
            self.color
                .get_mut(&icon_key)
                .unwrap()
                .grow(gfx_surface, new_max);
            self.secondary_color
                .get_mut(&icon_key)
                .unwrap()
                .grow(gfx_surface, new_max);
            self.color_invert
                .get_mut(&icon_key)
                .unwrap()
                .grow(gfx_surface, new_max);
            self.null_bit
                .get_mut(&icon_key)
                .unwrap()
                .grow(gfx_surface, new_max);
        }
    }
    pub(crate) fn write(&mut self, gfx_surface: &GfxSurface) {
        let mut writers = HashSet::new();
        for (key, _) in self.icon_entities.iter() {
            writers.insert(*key);
        }
        for key in writers {
            self.position
                .get_mut(&key)
                .unwrap()
                .write_attribute(&gfx_surface);
            self.area
                .get_mut(&key)
                .unwrap()
                .write_attribute(&gfx_surface);
            self.depth
                .get_mut(&key)
                .unwrap()
                .write_attribute(&gfx_surface);
            self.color
                .get_mut(&key)
                .unwrap()
                .write_attribute(&gfx_surface);
            self.secondary_color
                .get_mut(&key)
                .unwrap()
                .write_attribute(&gfx_surface);
            self.color_invert
                .get_mut(&key)
                .unwrap()
                .write_attribute(&gfx_surface);
            self.null_bit
                .get_mut(&key)
                .unwrap()
                .write_attribute(&gfx_surface);
        }
    }
    pub(crate) fn remove_icon(&mut self, entity: Entity) {
        if let Some(icon) = self.entity_icons.remove(&entity) {
            let key = self
                .entity_keys
                .get_mut(&icon)
                .unwrap()
                .remove(&entity)
                .unwrap();
            self.icon_entities.get_mut(&icon).unwrap().remove(&entity);
            let index = self.indexer.get_mut(&icon).unwrap().remove(key).unwrap();
            self.null_bit
                .get_mut(&icon)
                .unwrap()
                .write
                .write
                .insert(index, NullBit::null());
        }
    }
    pub(crate) fn new(
        gfx_surface: &GfxSurface,
        gfx_surface_config: &GfxSurfaceConfiguration,
        viewport: &Viewport,
        sample_count: u32,
    ) -> Self {
        Self {
            pipeline: {
                let shader = gfx_surface
                    .device
                    .create_shader_module(include_wgsl!("icon.wgsl"));
                let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
                    label: Some("icon pipeline layout"),
                    bind_group_layouts: &[&viewport.bind_group_layout],
                    push_constant_ranges: &[],
                };
                let pipeline_layout = gfx_surface
                    .device
                    .create_pipeline_layout(&pipeline_layout_descriptor);
                let fragment_state = wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fragment_entry",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: gfx_surface_config.configuration.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: Default::default(),
                    })],
                };
                let pipeline_descriptor = wgpu::RenderPipelineDescriptor {
                    label: Some("icon renderer pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: VertexState {
                        module: &shader,
                        entry_point: "vertex_entry",
                        buffers: &[
                            wgpu::VertexBufferLayout {
                                array_stride: std::mem::size_of::<IconVertex>()
                                    as wgpu::BufferAddress,
                                step_mode: wgpu::VertexStepMode::Vertex,
                                attributes: &wgpu::vertex_attr_array![0 => Float32x4],
                            },
                            wgpu::VertexBufferLayout {
                                array_stride: std::mem::size_of::<GpuPosition>()
                                    as wgpu::BufferAddress,
                                step_mode: wgpu::VertexStepMode::Instance,
                                attributes: &wgpu::vertex_attr_array![1 => Float32x2],
                            },
                            wgpu::VertexBufferLayout {
                                array_stride: std::mem::size_of::<GpuArea>() as wgpu::BufferAddress,
                                step_mode: wgpu::VertexStepMode::Instance,
                                attributes: &wgpu::vertex_attr_array![2 => Float32x2],
                            },
                            wgpu::VertexBufferLayout {
                                array_stride: std::mem::size_of::<Depth>() as wgpu::BufferAddress,
                                step_mode: wgpu::VertexStepMode::Instance,
                                attributes: &wgpu::vertex_attr_array![3 => Float32],
                            },
                            wgpu::VertexBufferLayout {
                                array_stride: std::mem::size_of::<Color>() as wgpu::BufferAddress,
                                step_mode: wgpu::VertexStepMode::Instance,
                                attributes: &wgpu::vertex_attr_array![4 => Float32x4],
                            },
                            wgpu::VertexBufferLayout {
                                array_stride: std::mem::size_of::<NullBit>() as wgpu::BufferAddress,
                                step_mode: wgpu::VertexStepMode::Instance,
                                attributes: &wgpu::vertex_attr_array![5 => Uint32],
                            },
                            wgpu::VertexBufferLayout {
                                array_stride: std::mem::size_of::<ColorInvert>()
                                    as wgpu::BufferAddress,
                                step_mode: wgpu::VertexStepMode::Instance,
                                attributes: &wgpu::vertex_attr_array![6 => Uint32],
                            },
                            wgpu::VertexBufferLayout {
                                array_stride: std::mem::size_of::<Color>() as wgpu::BufferAddress,
                                step_mode: wgpu::VertexStepMode::Instance,
                                attributes: &wgpu::vertex_attr_array![7 => Float32x4],
                            },
                        ],
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: viewport.depth_format,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: sample_count,
                        ..wgpu::MultisampleState::default()
                    },
                    fragment: Some(fragment_state),
                    multiview: None,
                };
                let pipeline = gfx_surface
                    .device
                    .create_render_pipeline(&pipeline_descriptor);
                pipeline
            },
            icon_entities: HashMap::new(),
            entity_icons: HashMap::new(),
            entity_keys: HashMap::new(),
            meshes: HashMap::new(),
            position: HashMap::new(),
            color: HashMap::new(),
            secondary_color: HashMap::new(),
            area: HashMap::new(),
            depth: HashMap::new(),
            key_factory: HashMap::new(),
            indexer: HashMap::new(),
            null_bit: HashMap::new(),
            color_invert: HashMap::new(),
        }
    }
}

impl Extract for IconRenderer {
    fn extract(frontend: &mut Job, backend: &mut Job) {
        let mut requests = frontend.container.query::<(Entity, &IconMeshAddRequest)>();
        let mut despawn = Vec::new();
        let mut backend_requests = Vec::new();
        for (entity, request) in requests.iter(&frontend.container) {
            despawn.push(entity);
            backend_requests.push(request.clone())
        }
        for ent in despawn {
            frontend.container.despawn(ent);
        }
        for request in backend_requests {
            backend.container.spawn(request);
        }
        let mut differences = frontend
            .container
            .get_resource_mut::<DifferenceHolder>()
            .unwrap()
            .differences
            .replace(Differences::new())
            .unwrap();
        differences.clean();
        backend.container.insert_resource(differences);
    }
}

impl Render for IconRenderer {
    fn phase() -> RenderPhase {
        RenderPhase::Alpha
    }
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        for (icon_key, _) in self.icon_entities.iter() {
            let indexer = self.indexer.get(icon_key).unwrap();
            if indexer.count() > 0 {
                let mesh = self.meshes.get(icon_key).unwrap();
                let positions = self.position.get(icon_key).unwrap();
                let area = self.area.get(icon_key).unwrap();
                let depth = self.depth.get(icon_key).unwrap();
                let color = self.color.get(icon_key).unwrap();
                let secondary_color = self.secondary_color.get(icon_key).unwrap();
                let null_bit = self.null_bit.get(icon_key).unwrap();
                let color_invert = self.color_invert.get(icon_key).unwrap();
                render_pass_handle.0.set_pipeline(&self.pipeline);
                render_pass_handle
                    .0
                    .set_bind_group(0, &viewport.bind_group, &[]);
                render_pass_handle
                    .0
                    .set_vertex_buffer(0, mesh.mesh.slice(..));
                render_pass_handle
                    .0
                    .set_vertex_buffer(1, positions.gpu.buffer.slice(..));
                render_pass_handle
                    .0
                    .set_vertex_buffer(2, area.gpu.buffer.slice(..));
                render_pass_handle
                    .0
                    .set_vertex_buffer(3, depth.gpu.buffer.slice(..));
                render_pass_handle
                    .0
                    .set_vertex_buffer(4, color.gpu.buffer.slice(..));
                render_pass_handle
                    .0
                    .set_vertex_buffer(5, null_bit.gpu.buffer.slice(..));
                render_pass_handle
                    .0
                    .set_vertex_buffer(6, color_invert.gpu.buffer.slice(..));
                render_pass_handle
                    .0
                    .set_vertex_buffer(7, secondary_color.gpu.buffer.slice(..));
                render_pass_handle
                    .0
                    .draw(0..mesh.length, 0..indexer.count());
            }
        }
    }
}
