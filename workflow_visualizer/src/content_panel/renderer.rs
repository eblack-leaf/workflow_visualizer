use crate::content_panel::vertex::{generate_mesh, vertex_buffer, ContentPanelVertex};
use crate::content_panel::{Difference, Extraction};
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration, MsaaRenderAttachment};
use crate::instance::key::KeyFactory;
use crate::{
    Color, Extract, Index, Indexer, InstanceAttributeManager, Job, Key, Layer, NullBit, RawArea,
    RawPosition, Render, RenderPassHandle, RenderPhase, ScaleFactor, Viewport,
};
use bevy_ecs::prelude::{Commands, Entity, Res, Resource};
use std::collections::HashMap;
#[derive(Resource)]
pub(crate) struct ContentPanelRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) positions: InstanceAttributeManager<RawPosition>,
    pub(crate) content_area: InstanceAttributeManager<RawArea>,
    pub(crate) layers: InstanceAttributeManager<Layer>,
    pub(crate) colors: InstanceAttributeManager<Color>,
    pub(crate) null_bits: InstanceAttributeManager<NullBit>,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) mesh_length: u32,
    pub(crate) indexer: Indexer<Entity>,
}
pub(crate) fn setup(
    gfx_surface: Res<GfxSurface>,
    viewport: Res<Viewport>,
    msaa: Res<MsaaRenderAttachment>,
    gfx_surface_config: Res<GfxSurfaceConfiguration>,
    scale_factor: Res<ScaleFactor>,
    mut cmd: Commands,
) {
    let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
        label: Some("content panel pipeline layout descriptor"),
        bind_group_layouts: &[&viewport.bind_group_layout],
        push_constant_ranges: &[],
    };
    let pipeline_layout = gfx_surface
        .device
        .create_pipeline_layout(&pipeline_layout_descriptor);
    let shader = gfx_surface
        .device
        .create_shader_module(wgpu::include_wgsl!("content_panel.wgsl"));
    let color_target_state = [Some(wgpu::ColorTargetState {
        format: gfx_surface_config.configuration.format,
        blend: None,
        write_mask: Default::default(),
    })];
    let descriptor = wgpu::RenderPipelineDescriptor {
        label: Some("content panel renderer"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vertex_entry",
            buffers: &[
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<ContentPanelVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x4],
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
            count: msaa.requested,
            ..wgpu::MultisampleState::default()
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fragment_entry",
            targets: &color_target_state,
        }),
        multiview: None,
    };
    let pipeline = gfx_surface.device.create_render_pipeline(&descriptor);
    let mesh = generate_mesh(64, scale_factor.factor);
    let mesh_len = mesh.len() as u32;
    let buffer = vertex_buffer(gfx_surface.as_ref(), mesh);
    let initial_max = 5;
    let renderer = ContentPanelRenderer {
        pipeline,
        positions: InstanceAttributeManager::new(&gfx_surface, initial_max),
        content_area: InstanceAttributeManager::new(&gfx_surface, initial_max),
        layers: InstanceAttributeManager::new(&gfx_surface, initial_max),
        colors: InstanceAttributeManager::new(&gfx_surface, initial_max),
        null_bits: InstanceAttributeManager::new(&gfx_surface, initial_max),
        vertex_buffer: buffer,
        mesh_length: mesh_len,
        indexer: Indexer::new(initial_max),
    };
    cmd.insert_resource(renderer);
}
impl Extract for ContentPanelRenderer {
    fn extract(frontend: &mut Job, backend: &mut Job) {
        let extracted_differences = frontend
            .container
            .get_resource_mut::<Extraction>()
            .expect("no extraction")
            .differences
            .drain()
            .collect::<Vec<(Entity, Difference)>>();
        for (entity, difference) in extracted_differences {
            backend
                .container
                .get_resource_mut::<Extraction>()
                .expect("no extraction")
                .differences
                .insert(entity, difference);
        }
    }
}
impl Render for ContentPanelRenderer {
    fn phase() -> RenderPhase {
        RenderPhase::Opaque
    }
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        if self.indexer.has_instances() {
            render_pass_handle.0.set_pipeline(&self.pipeline);
            render_pass_handle
                .0
                .set_bind_group(0, &viewport.bind_group, &[]);
            render_pass_handle
                .0
                .set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(1, self.positions.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(2, self.content_area.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(3, self.layers.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(4, self.colors.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(5, self.null_bits.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .draw(0..self.mesh_length, 0..self.indexer.count());
        }
    }
}
