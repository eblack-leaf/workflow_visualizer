use std::collections::{HashMap, HashSet};
use std::ptr::null;

use bevy_ecs::prelude::{
    Added, Bundle, Changed, Commands, Component, Entity, Or, Query, RemovedComponents, Res, ResMut,
    Resource,
};
use bytemuck::{Pod, Zeroable};
use wgpu::{include_wgsl, VertexState};
use wgpu::util::DeviceExt;

use crate::{
    Area, Attach, Color, Depth, DeviceView, Engen, Job, Position, ScaleFactor, Section, UIView,
    Visibility,
};
use crate::coord::{GpuArea, GpuPosition, Panel};
use crate::extract::Extract;
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration};
pub use crate::icon::interface::{Icon, IconBundle, IconSize};
pub use crate::icon::plugin::IconPlugin;
use crate::index::Indexer;
use crate::instance_tools::{AttributeWrite, CpuAttributeBuffer, InstanceTools};
use crate::instance_tools::GpuAttributeBuffer;
use crate::instance_tools::NullBit;
use crate::key::{Key, KeyFactory};
use crate::render::{Render, RenderPassHandle, RenderPhase};
use crate::viewport::Viewport;

mod interface;
mod plugin;

#[derive(Resource)]
pub struct IconAreaGuide {
    pub guide: HashMap<IconSize, u32>,
}

impl IconAreaGuide {
    pub fn new() -> Self {
        Self {
            guide: HashMap::new(),
        }
    }
}

impl Default for IconAreaGuide {
    fn default() -> Self {
        let mut guide = Self::new();
        guide.guide.insert(IconSize::Small, 12);
        guide.guide.insert(IconSize::Medium, 15);
        guide.guide.insert(IconSize::Large, 18);
        guide
    }
}

pub(crate) fn calc_area(
    icon_area_guide: Res<IconAreaGuide>,
    scale_factor: Res<ScaleFactor>,
    icons: Query<(Entity, &IconSize), (Changed<IconSize>)>,
    mut cmd: Commands,
) {
    for (entity, size) in icons.iter() {
        let area_guide = *icon_area_guide.guide.get(size).unwrap();
        let scaled = area_guide as f64 * scale_factor.factor;
        cmd.entity(entity)
            .insert(Area::<UIView>::new(scaled as f32, scaled as f32));
    }
}

#[derive(Clone)]
pub struct IconMesh {
    pub mesh: Vec<IconVertex>,
}

impl IconMesh {
    pub fn new(mesh: Vec<IconVertex>) -> Self {
        Self { mesh }
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
pub struct IconVertex {
    pub position: GpuPosition,
}

impl IconVertex {
    pub const fn new(position: GpuPosition) -> Self {
        Self {
            position,
        }
    }
}

#[derive(Component, Hash, Eq, PartialEq, Copy, Clone)]
pub struct IconKey(pub &'static str);

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

impl IconAdd {
    pub(crate) fn new(
        key: IconKey,
        position: Position<UIView>,
        area: Area<UIView>,
        depth: Depth,
        color: Color,
        scale_factor: f64,
    ) -> Self {
        Self {
            key,
            panel: Panel::<DeviceView>::new(
                Section::<DeviceView>::new(
                    position.to_device(scale_factor),
                    Area::<DeviceView>::new(area.width, area.height),
                ),
                depth,
            ),
            color,
        }
    }
}

pub(crate) fn frontend_setup(mut cmd: Commands) {
    cmd.insert_resource(Cache::new());
    cmd.insert_resource(DifferenceHolder::new());
    cmd.insert_resource(IconAreaGuide::default());
}

pub(crate) fn initialization(
    icons: Query<
        (
            Entity,
            &IconKey,
            &Position<UIView>,
            &Area<UIView>,
            &Depth,
            &Color,
            &Visibility,
        ),
        Or<(Added<Icon>, Changed<Visibility>)>,
    >,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
    removed_icons: RemovedComponents<Icon>,
) {
    let mut removals = HashSet::new();
    for entity in removed_icons.iter() {
        removals.insert(entity);
    }
    for (entity, icon_key, position, area, depth, color, visibility) in icons.iter() {
        if visibility.visible() {
            difference_holder
                .differences
                .as_mut()
                .unwrap()
                .icon_adds
                .insert(entity, (*icon_key, *position, *area, *depth, *color));
            cache.icon_key.insert(entity, *icon_key);
            cache.position.insert(entity, *position);
            cache.area.insert(entity, *area);
            cache.depth.insert(entity, *depth);
            cache.color.insert(entity, *color);
        } else {
            removals.insert(entity);
        }
    }
    for removed in removals {
        difference_holder
            .differences
            .as_mut()
            .unwrap()
            .icon_removes
            .insert(removed);
        let icon_key = cache.icon_key.remove(&removed).unwrap();
    }
}

pub(crate) fn position_cache_check(
    icons: Query<(Entity, &Position<UIView>), (Changed<Position<UIView>>)>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, position) in icons.iter() {
        let cached_value = cache.position.get(&entity);
        if let Some(val) = cached_value {
            if position != val {
                difference_holder
                    .differences
                    .as_mut()
                    .unwrap()
                    .position
                    .insert(entity, *position);
                cache.position.insert(entity, *position);
            }
        }
    }
}

pub(crate) fn area_cache_check(
    icons: Query<(Entity, &Area<UIView>), (Changed<Area<UIView>>)>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, area) in icons.iter() {
        let cached_value = cache.area.get(&entity);
        if let Some(val) = cached_value {
            if area != val {
                difference_holder
                    .differences
                    .as_mut()
                    .unwrap()
                    .area
                    .insert(entity, *area);
                cache.area.insert(entity, *area);
            }
        }
    }
}

pub(crate) fn depth_cache_check(
    icons: Query<(Entity, &Depth), (Changed<Depth>)>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, depth) in icons.iter() {
        let cached_value = cache.depth.get(&entity);
        if let Some(val) = cached_value {
            if depth != val {
                difference_holder
                    .differences
                    .as_mut()
                    .unwrap()
                    .depth
                    .insert(entity, *depth);
                cache.depth.insert(entity, *depth);
            }
        }
    }
}

pub(crate) fn color_cache_check(
    icons: Query<(Entity, &Color), (Changed<Color>)>,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, color) in icons.iter() {
        let cached_value = cache.color.get(&entity);
        if let Some(val) = cached_value {
            if color != val {
                difference_holder
                    .differences
                    .as_mut()
                    .unwrap()
                    .color
                    .insert(entity, *color);
                cache.color.insert(entity, *color);
            }
        }
    }
}

pub(crate) fn icon_key_cache_check(
    icons: Query<
        (
            Entity,
            &IconKey,
            &Position<UIView>,
            &Area<UIView>,
            &Depth,
            &Color,
        ),
        (Changed<IconKey>),
    >,
    mut cache: ResMut<Cache>,
    mut difference_holder: ResMut<DifferenceHolder>,
) {
    for (entity, icon_key, position, area, depth, color) in icons.iter() {
        let cached_value = cache.icon_key.get(&entity);
        if let Some(val) = cached_value {
            if icon_key != val {
                difference_holder
                    .differences
                    .as_mut()
                    .unwrap()
                    .icon_removes
                    .insert(entity);
                difference_holder
                    .differences
                    .as_mut()
                    .unwrap()
                    .icon_adds
                    .insert(entity, (*icon_key, *position, *area, *depth, *color));
                cache.icon_key.insert(entity, *icon_key);
            }
        }
    }
}

#[derive(Resource)]
pub(crate) struct Cache {
    pub(crate) icon_key: HashMap<Entity, IconKey>,
    pub(crate) depth: HashMap<Entity, Depth>,
    pub(crate) position: HashMap<Entity, Position<UIView>>,
    pub(crate) area: HashMap<Entity, Area<UIView>>,
    pub(crate) color: HashMap<Entity, Color>,
}

impl Cache {
    pub(crate) fn new() -> Self {
        Self {
            icon_key: HashMap::new(),
            depth: HashMap::new(),
            position: HashMap::new(),
            area: HashMap::new(),
            color: HashMap::new(),
        }
    }
}

#[derive(Resource)]
pub(crate) struct DifferenceHolder {
    pub(crate) differences: Option<Differences>,
}

impl DifferenceHolder {
    pub(crate) fn new() -> Self {
        Self {
            differences: Some(Differences::new()),
        }
    }
}

#[derive(Resource)]
pub(crate) struct Differences {
    pub(crate) icon_adds: HashMap<Entity, (IconKey, Position<UIView>, Area<UIView>, Depth, Color)>,
    pub(crate) icon_removes: HashSet<Entity>,
    pub(crate) depth: HashMap<Entity, Depth>,
    pub(crate) position: HashMap<Entity, Position<UIView>>,
    pub(crate) area: HashMap<Entity, Area<UIView>>,
    pub(crate) color: HashMap<Entity, Color>,
}

impl Differences {
    pub(crate) fn new() -> Self {
        Self {
            icon_adds: HashMap::new(),
            icon_removes: HashSet::new(),
            depth: HashMap::new(),
            position: HashMap::new(),
            area: HashMap::new(),
            color: HashMap::new(),
        }
    }
    pub(crate) fn clean(&mut self) {
        let removed_entities = self.icon_removes.clone();
        for entity in removed_entities {
            self.position.remove(&entity);
            self.area.remove(&entity);
            self.depth.remove(&entity);
            self.color.remove(&entity);
        }
        let added_entities = self
            .icon_adds
            .iter()
            .map(|a| *a.0)
            .collect::<HashSet<Entity>>();
        for entity in added_entities {
            self.position.remove(&entity);
            self.area.remove(&entity);
            self.depth.remove(&entity);
            self.color.remove(&entity);
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
        renderer.add_mesh(
            &gfx_surface,
            request.icon_key,
            request.icon_mesh.to_gpu(&gfx_surface),
            request.max,
        )
    }
}

pub(crate) fn process_differences(
    mut renderer: ResMut<IconRenderer>,
    differences: Res<Differences>,
    scale_factor: Res<ScaleFactor>,
    gfx_surface: Res<GfxSurface>,
) {
    renderer.process_differences(&differences, scale_factor.factor, &gfx_surface);
}

#[derive(Resource)]
pub(crate) struct IconRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) icon_entities: HashMap<IconKey, HashSet<Entity>>,
    pub(crate) entity_icons: HashMap<Entity, IconKey>,
    pub(crate) entity_keys: HashMap<IconKey, HashMap<Entity, Key>>,
    pub(crate) meshes: HashMap<IconKey, GpuIconMesh>,
    pub(crate) position: HashMap<IconKey, InstanceTools<GpuPosition>>,
    pub(crate) area: HashMap<IconKey, InstanceTools<GpuArea>>,
    pub(crate) depth: HashMap<IconKey, InstanceTools<Depth>>,
    pub(crate) color: HashMap<IconKey, InstanceTools<Color>>,
    pub(crate) null_bit: HashMap<IconKey, InstanceTools<NullBit>>,
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
            .insert(icon_key, InstanceTools::new(gfx_surface, max));
        self.area
            .insert(icon_key, InstanceTools::new(gfx_surface, max));
        self.depth
            .insert(icon_key, InstanceTools::new(gfx_surface, max));
        self.color
            .insert(icon_key, InstanceTools::new(gfx_surface, max));
        self.null_bit
            .insert(icon_key, InstanceTools::new(gfx_surface, max));
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
        for (entity, (key, position, area, depth, color)) in differences.icon_adds.iter() {
            self.add_icon(
                *entity,
                IconAdd::new(*key, *position, *area, *depth, *color, scale_factor),
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
                                array_stride: std::mem::size_of::<IconVertex>() as wgpu::BufferAddress,
                                step_mode: wgpu::VertexStepMode::Vertex,
                                attributes: &wgpu::vertex_attr_array![0 => Float32x2],
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
                    multisample: wgpu::MultisampleState::default(),
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
            area: HashMap::new(),
            depth: HashMap::new(),
            key_factory: HashMap::new(),
            indexer: HashMap::new(),
            null_bit: HashMap::new(),
        }
    }
}

pub(crate) fn setup(
    gfx_surface: Res<GfxSurface>,
    gfx_surface_config: Res<GfxSurfaceConfiguration>,
    viewport: Res<Viewport>,
    mut cmd: Commands,
) {
    cmd.insert_resource(IconRenderer::new(
        &gfx_surface,
        &gfx_surface_config,
        &viewport,
    ));
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
        RenderPhase::Opaque
    }
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        for (icon_key, entities) in self.icon_entities.iter() {
            let indexer = self.indexer.get(icon_key).unwrap();
            if indexer.count() > 0 {
                let mesh = self.meshes.get(icon_key).unwrap();
                let positions = self.position.get(icon_key).unwrap();
                let area = self.area.get(icon_key).unwrap();
                let depth = self.depth.get(icon_key).unwrap();
                let color = self.color.get(icon_key).unwrap();
                let null_bit = self.null_bit.get(icon_key).unwrap();
                render_pass_handle.0.set_pipeline(&self.pipeline);
                render_pass_handle.0.set_bind_group(0, &viewport.bind_group, &[]);
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
                    .draw(0..mesh.length, 0..indexer.count());
            }
        }
    }
}
