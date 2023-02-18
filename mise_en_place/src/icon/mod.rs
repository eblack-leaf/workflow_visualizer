use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Bundle, Commands, Component, Entity, Query, Res, ResMut, Resource};
use bytemuck::{Pod, Zeroable};
use wgpu::{include_wgsl, VertexState};
use wgpu::util::DeviceExt;

use crate::{Area, Attach, Color, Depth, DeviceView, Engen, Job};
use crate::coord::{GpuArea, GpuPosition, Panel};
use crate::extract::Extract;
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration};
use crate::icon::interface::Icon;
pub use crate::icon::plugin::IconPlugin;
use crate::index::Indexer;
use crate::instance_tools::{AttributeWrite, CpuAttributeBuffer, InstanceTools};
use crate::instance_tools::GpuAttributeBuffer;
use crate::key::{Key, KeyFactory};
use crate::render::{Render, RenderPassHandle, RenderPhase};
use crate::viewport::Viewport;

mod plugin;
mod interface;

pub(crate) fn calc_area() {
    // calc area by icon-size * scale_factor
}

#[derive(Clone)]
pub struct IconMesh {
    pub mesh: Vec<Vertex>,
}

impl IconMesh {
    pub fn new(mesh: Vec<Vertex>) -> Self {
        Self {
            mesh
        }
    }
    pub(crate) fn to_gpu(&self, gfx_surface: &GfxSurface) -> GpuIconMesh {
        GpuIconMesh {
            mesh: gfx_surface
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("text vertex buffer"),
                    contents: bytemuck::cast_slice(&self.mesh),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }),
            length: self.mesh.len() as u32,
        }
    }
}

pub(crate) struct GpuIconMesh {
    pub(crate) mesh: wgpu::Buffer,
    pub(crate) length: u32,
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
pub struct Vertex {
    pub position: GpuPosition,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct IconKey(&'static str);

#[derive(Resource)]
pub(crate) struct IconRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) icons: HashMap<IconKey, HashSet<Entity>>,
    pub(crate) meshes: HashMap<IconKey, GpuIconMesh>,
    pub(crate) position: HashMap<IconKey, InstanceTools<GpuPosition>>,
    pub(crate) area: HashMap<IconKey, InstanceTools<GpuArea>>,
    pub(crate) depth: HashMap<IconKey, InstanceTools<Depth>>,
    pub(crate) color: HashMap<IconKey, InstanceTools<Color>>,
    pub(crate) key_factory: HashMap<IconKey, KeyFactory>,
    pub(crate) indexer: HashMap<IconKey, Indexer<Key>>,
}

#[derive(Component, Clone)]
pub struct IconMeshAddRequest {
    pub icon_key: IconKey,
    pub icon_mesh: IconMesh,
    pub max: u32,
}

impl IconMeshAddRequest {
    pub fn new(icon_key: IconKey, icon_mesh: IconMesh, max: u32) -> Self {
        Self {
            icon_key,
            icon_mesh,
            max,
        }
    }
}

pub(crate) struct IconAdd {
    pub(crate) key: IconKey,
    pub(crate) panel: Panel<DeviceView>,
    pub(crate) color: Color,
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
        backend.container.spawn_batch(backend_requests);
    }
}

impl Render for IconRenderer {
    fn phase() -> RenderPhase {
        RenderPhase::Opaque
    }
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        for (icon_key, entities) in self.icons.iter() {
            let indexer = self.indexer.get(icon_key).unwrap();
            if indexer.count() > 0 {
                let mesh = self.meshes.get(icon_key).unwrap();
                let positions = self.position.get(icon_key).unwrap();
                let area = self.area.get(icon_key).unwrap();
                let depth = self.depth.get(icon_key).unwrap();
                let color = self.color.get(icon_key).unwrap();
                render_pass_handle.0.set_vertex_buffer(0, mesh.mesh.slice(..));
                render_pass_handle.0.set_vertex_buffer(1, positions.gpu.buffer.slice(..));
                render_pass_handle.0.set_vertex_buffer(2, area.gpu.buffer.slice(..));
                render_pass_handle.0.set_vertex_buffer(3, depth.gpu.buffer.slice(..));
                render_pass_handle.0.set_vertex_buffer(4, color.gpu.buffer.slice(..));
                render_pass_handle.0.draw(0..mesh.length, 0..indexer.count());
            }
        }
    }
}

pub(crate) fn read_add_requests(
    mut renderer: ResMut<IconRenderer>,
    mut cmd: Commands,
    requests: Query<(Entity, &IconMeshAddRequest)>,
    gfx_surface: Res<GfxSurface>,
) {
    for (entity, request) in requests.iter() {
        cmd.entity(entity).despawn();
        renderer.add_mesh(&gfx_surface, request.icon_key, request.icon_mesh.to_gpu(&gfx_surface), request.max)
    }
}

impl IconRenderer {
    pub(crate) fn add_mesh(&mut self, gfx_surface: &GfxSurface, icon_key: IconKey, mesh: GpuIconMesh, max: u32) {
        self.meshes.insert(icon_key, mesh);
        self.icons.insert(icon_key, HashSet::new());
        self.position.insert(icon_key, InstanceTools::new(gfx_surface, max));
        self.area.insert(icon_key, InstanceTools::new(gfx_surface, max));
        self.depth.insert(icon_key, InstanceTools::new(gfx_surface, max));
        self.color.insert(icon_key, InstanceTools::new(gfx_surface, max));
        self.key_factory.insert(icon_key, KeyFactory::new());
        self.indexer.insert(icon_key, Indexer::new(max));
    }
    pub(crate) fn add_icon(&mut self, entity: Entity, icon: IconAdd) {
        let key = self.key_factory.get_mut(&icon.key).unwrap().generate();
        self.icons.get_mut(&icon.key).unwrap().insert(entity);
        let index = self.indexer.get_mut(&icon.key).unwrap().next(key);
        self.position.get_mut(&icon.key).unwrap().write.write.insert(index, icon.panel.section.position.to_gpu());
        self.area.get_mut(&icon.key).unwrap().write.write.insert(index, icon.panel.section.area.to_gpu());
        self.depth.get_mut(&icon.key).unwrap().write.write.insert(index, icon.panel.depth);
        self.color.get_mut(&icon.key).unwrap().write.write.insert(index, icon.color);
    }
    pub(crate) fn new(gfx_surface: &GfxSurface, gfx_surface_config: &GfxSurfaceConfiguration, viewport: &Viewport) -> Self {
        Self {
            pipeline: {
                let shader = gfx_surface.device.create_shader_module(include_wgsl!("icon.wgsl"));
                let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
                    label: Some("icon pipeline layout"),
                    bind_group_layouts: &[
                        &viewport.bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                };
                let pipeline_layout = gfx_surface.device.create_pipeline_layout(&pipeline_layout_descriptor);
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
                                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                                step_mode: wgpu::VertexStepMode::Vertex,
                                attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                            },
                            wgpu::VertexBufferLayout {
                                array_stride: std::mem::size_of::<GpuPosition>() as wgpu::BufferAddress,
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
                    multisample: Default::default(),
                    fragment: Some(fragment_state),
                    multiview: None,
                };
                let pipeline = gfx_surface.device.create_render_pipeline(&pipeline_descriptor);
                pipeline
            },
            icons: HashMap::new(),
            meshes: HashMap::new(),
            position: HashMap::new(),
            color: HashMap::new(),
            area: HashMap::new(),
            depth: HashMap::new(),
            key_factory: HashMap::new(),
            indexer: HashMap::new(),
        }
    }
}
