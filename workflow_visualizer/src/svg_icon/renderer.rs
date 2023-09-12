use crate::svg_icon::interface::{Difference, SvgTag};
use crate::svg_icon::render_group::RenderGroup;
use crate::svg_icon::tessellation::SvgRequest;
use crate::{
    Color, Disabled, GfxSurface, GfxSurfaceConfiguration, Layer, MsaaRenderAdapter, NullBit,
    RawArea, RawPosition, Render, RenderPassHandle, RenderPhase, ResourceHandle, ScaleFactor,
    Viewport, Visibility, Visualizer,
};
use bevy_ecs::entity::Entity;
#[cfg(not(target_family = "wasm"))]
use bevy_ecs::prelude::Resource;
use bevy_ecs::prelude::{Res, ResMut};
use bevy_ecs::query::{Changed, Or, Without};
use bevy_ecs::removal_detection::RemovedComponents;
use bevy_ecs::system::{Commands, Query};
#[cfg(target_family = "wasm")]
use bevy_ecs::system::{NonSend, NonSendMut};
use std::collections::HashMap;
use wgpu::util::DeviceExt;
use wgpu::VertexState;

pub(crate) struct TessellatedSvgBuffer {
    pub(crate) vertices: wgpu::Buffer,
    pub(crate) indices: wgpu::Buffer,
    pub(crate) indices_count: u32,
}
impl TessellatedSvgBuffer {
    pub(crate) fn new(gfx: &GfxSurface, vertices: Vec<RawPosition>, indices: Vec<u32>) -> Self {
        Self {
            vertices: gfx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("tessellated svg buffer (vertices)"),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }),
            indices: gfx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("tessellated svg buffer (indices)"),
                    contents: bytemuck::cast_slice(&indices),
                    usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                }),
            indices_count: indices.len() as u32,
        }
    }
}
#[cfg_attr(not(target_family = "wasm"), derive(Resource))]
pub(crate) struct SvgRenderer {
    pipeline: wgpu::RenderPipeline,
    pub(crate) svg_buffers: HashMap<ResourceHandle, TessellatedSvgBuffer>,
    pub(crate) render_groups: HashMap<ResourceHandle, RenderGroup>,
    pub(crate) entity_mapping: HashMap<Entity, ResourceHandle>,
}
impl SvgRenderer {
    fn remove_existing_resources(&mut self, entity: Entity) {
        if let Some(handle) = self.entity_mapping.remove(&entity) {
            if let Some(group) = self.render_groups.get_mut(&handle) {
                if let Some(index) = group.indexer.remove(entity) {
                    group.null_bits.queue_write(index, NullBit::null());
                }
            }
        }
    }
    fn associate_resource(&mut self, handle: ResourceHandle, entity: Entity) {
        self.entity_mapping.insert(entity, handle);
        if let Some(group) = self.render_groups.get_mut(&handle) {
            let index = group.indexer.next(entity);
            group.null_bits.queue_write(index, NullBit::not_null());
        }
    }
    fn queue_position(&mut self, entity: Entity, attr: RawPosition) {
        if let Some(handle) = self.entity_mapping.get(&entity) {
            if let Some(index) = self
                .render_groups
                .get(handle)
                .expect("group")
                .indexer
                .get_index(entity)
            {
                self.render_groups
                    .get_mut(handle)
                    .expect("group")
                    .positions
                    .queue_write(index, attr);
            }
        }
    }
    fn queue_area(&mut self, entity: Entity, attr: RawArea) {
        if let Some(handle) = self.entity_mapping.get(&entity) {
            if let Some(index) = self
                .render_groups
                .get(handle)
                .expect("group")
                .indexer
                .get_index(entity)
            {
                self.render_groups
                    .get_mut(handle)
                    .expect("group")
                    .areas
                    .queue_write(index, attr);
            }
        }
    }
    fn queue_layer(&mut self, entity: Entity, attr: Layer) {
        if let Some(handle) = self.entity_mapping.get(&entity) {
            if let Some(index) = self
                .render_groups
                .get(handle)
                .expect("group")
                .indexer
                .get_index(entity)
            {
                self.render_groups
                    .get_mut(handle)
                    .expect("group")
                    .layers
                    .queue_write(index, attr);
            }
        }
    }
    fn queue_color(&mut self, entity: Entity, attr: Color) {
        if let Some(handle) = self.entity_mapping.get(&entity) {
            if let Some(index) = self
                .render_groups
                .get(handle)
                .expect("group")
                .indexer
                .get_index(entity)
            {
                self.render_groups
                    .get_mut(handle)
                    .expect("group")
                    .colors
                    .queue_write(index, attr);
            }
        }
    }
}
pub(crate) fn load_svg_buffers(
    mut requests: Query<(Entity, &mut SvgRequest)>,
    mut cmd: Commands,
    #[cfg(not(target_family = "wasm"))] mut renderer: ResMut<SvgRenderer>,
    #[cfg(target_family = "wasm")] mut renderer: NonSendMut<SvgRenderer>,
    #[cfg(not(target_family = "wasm"))] gfx: Res<GfxSurface>,
    #[cfg(target_family = "wasm")] gfx: NonSend<GfxSurface>,
) {
    for (entity, mut request) in requests.iter_mut() {
        let svg = request.tessellated_svg.take().expect("svg");
        let buffer = TessellatedSvgBuffer::new(&gfx, svg.vertices, svg.indices);
        renderer.svg_buffers.insert(request.handle, buffer);
        renderer
            .render_groups
            .insert(request.handle, RenderGroup::new(&gfx, 1));
        cmd.entity(entity).despawn();
    }
}
pub(crate) fn cull_entities(
    query: Query<
        (Entity, &Visibility, Option<&Disabled>),
        Or<(Changed<Visibility>, Changed<Disabled>)>,
    >,
    #[cfg(not(target_family = "wasm"))] mut renderer: ResMut<SvgRenderer>,
    #[cfg(target_family = "wasm")] mut renderer: NonSendMut<SvgRenderer>,
    mut removed: RemovedComponents<SvgTag>,
) {
    for entity in removed.iter() {
        renderer.remove_existing_resources(entity);
    }
    for (entity, vis, disable) in query.iter() {
        if !vis.visible() || disable.is_some() {
            renderer.remove_existing_resources(entity);
        }
    }
}
pub(crate) fn read_differences(
    mut query: Query<(Entity, &mut Difference, &Visibility), Without<Disabled>>,
    #[cfg(not(target_family = "wasm"))] mut renderer: ResMut<SvgRenderer>,
    #[cfg(target_family = "wasm")] mut renderer: NonSendMut<SvgRenderer>,
    #[cfg(not(target_family = "wasm"))] gfx: Res<GfxSurface>,
    #[cfg(target_family = "wasm")] gfx: NonSend<GfxSurface>,
    scale_factor: Res<ScaleFactor>,
) {
    for (entity, mut diff, vis) in query.iter_mut() {
        if vis.visible() {
            if let Some(svg) = diff.attributes.svg.take() {
                renderer.remove_existing_resources(entity);
                renderer.associate_resource(svg, entity);
            }
        }
    }
    for (_handle, group) in renderer.render_groups.iter_mut() {
        if group.indexer.should_grow() {
            let max = group.indexer.max();
            group.positions.grow(&gfx, max);
            group.areas.grow(&gfx, max);
            group.layers.grow(&gfx, max);
            group.colors.grow(&gfx, max);
            group.null_bits.grow(&gfx, max);
        }
        group.positions.write(&gfx);
        group.areas.write(&gfx);
        group.layers.write(&gfx);
        group.colors.write(&gfx);
        group.null_bits.write(&gfx);
    }
    for (entity, mut diff, vis) in query.iter_mut() {
        if vis.visible() {
            if let Some(position) = diff.attributes.position.take() {
                renderer.queue_position(entity, position.to_device(scale_factor.factor()).as_raw());
            }
            if let Some(area) = diff.attributes.area.take() {
                renderer.queue_area(entity, area.to_device(scale_factor.factor()).as_raw());
            }
            if let Some(layer) = diff.attributes.layer.take() {
                renderer.queue_layer(entity, layer);
            }
            if let Some(color) = diff.attributes.color.take() {
                renderer.queue_color(entity, color);
            }
        }
    }
}
impl Render for SvgRenderer {
    fn setup(
        _visualizer: &Visualizer,
        gfx: &GfxSurface,
        viewport: &Viewport,
        gfx_config: &GfxSurfaceConfiguration,
        msaa: &MsaaRenderAdapter,
        _scale_factor: &ScaleFactor,
    ) -> Self {
        let shader = gfx
            .device
            .create_shader_module(wgpu::include_wgsl!("svg_icon.wgsl"));
        let vertex_state = VertexState {
            module: &shader,
            entry_point: "vertex_entry",
            buffers: &[
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<RawPosition>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                },
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<RawPosition>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![1 => Float32x2],
                },
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<RawArea>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![2 => Float32x2],
                },
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Layer>() as wgpu::BufferAddress,
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
        };
        let fragment_state = wgpu::FragmentState {
            module: &shader,
            entry_point: "fragment_entry",
            targets: &gfx_config.alpha_color_target_state(),
        };
        let pipeline_layout = gfx
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("svg-icon-renderer"),
                bind_group_layouts: &[viewport.bind_group_layout()],
                push_constant_ranges: &[],
            });
        let pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("svg-icon-render-pipeline"),
            layout: Some(&pipeline_layout),
            vertex: vertex_state,
            primitive: gfx.triangle_primitive(),
            depth_stencil: Option::from(viewport.depth_stencil_state()),
            multisample: msaa.multisample_state(),
            fragment: Some(fragment_state),
            multiview: None,
        };
        let pipeline = gfx.device.create_render_pipeline(&pipeline_descriptor);
        Self {
            pipeline,
            svg_buffers: HashMap::new(),
            render_groups: HashMap::new(),
            entity_mapping: HashMap::new(),
        }
    }

    fn phase() -> RenderPhase {
        RenderPhase::Alpha(1)
    }

    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        render_pass_handle.0.set_pipeline(&self.pipeline);
        render_pass_handle
            .0
            .set_bind_group(0, viewport.bind_group(), &[]);
        for (handle, buffers) in self.svg_buffers.iter() {
            render_pass_handle
                .0
                .set_vertex_buffer(0, buffers.vertices.slice(..));
            render_pass_handle
                .0
                .set_index_buffer(buffers.indices.slice(..), wgpu::IndexFormat::Uint32);
            if let Some(group) = self.render_groups.get(handle) {
                if group.indexer.has_instances() {
                    render_pass_handle
                        .0
                        .set_vertex_buffer(1, group.positions.gpu.buffer.slice(..));
                    render_pass_handle
                        .0
                        .set_vertex_buffer(2, group.areas.gpu.buffer.slice(..));
                    render_pass_handle
                        .0
                        .set_vertex_buffer(3, group.layers.gpu.buffer.slice(..));
                    render_pass_handle
                        .0
                        .set_vertex_buffer(4, group.colors.gpu.buffer.slice(..));
                    render_pass_handle
                        .0
                        .set_vertex_buffer(5, group.null_bits.gpu.buffer.slice(..));
                    render_pass_handle.0.draw_indexed(
                        0..buffers.indices_count,
                        0,
                        0..group.indexer.count(),
                    );
                }
            }
        }
    }
}
