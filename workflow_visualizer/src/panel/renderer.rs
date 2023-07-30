use bevy_ecs::prelude::{Commands, Entity, Res, Resource};
use wgpu::BlendState;

use crate::{
    Color, Indexer, InstanceAttributeManager, Layer, NullBit, RawArea, RawPosition, Render,
    RenderPassHandle, RenderPhase, ScaleFactor, Viewport,
};
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration, MsaaRenderAdapter};
use crate::panel::vertex::{generate_border_mesh, generate_panel_mesh, PanelVertex, vertex_buffer};

#[derive(Resource)]
pub(crate) struct PanelRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) positions: InstanceAttributeManager<RawPosition>,
    pub(crate) content_area: InstanceAttributeManager<RawArea>,
    pub(crate) layers: InstanceAttributeManager<Layer>,
    pub(crate) panel_colors: InstanceAttributeManager<Color>,
    pub(crate) panel_null_bits: InstanceAttributeManager<NullBit>,
    pub(crate) panel_vertex_buffer: wgpu::Buffer,
    pub(crate) panel_mesh_len: u32,
    pub(crate) border_null_bits: InstanceAttributeManager<NullBit>,
    pub(crate) border_colors: InstanceAttributeManager<Color>,
    pub(crate) border_vertex_buffer: wgpu::Buffer,
    pub(crate) border_mesh_len: u32,
    pub(crate) indexer: Indexer<Entity>,
}
pub(crate) fn setup(
    gfx_surface: Res<GfxSurface>,
    viewport: Res<Viewport>,
    msaa: Res<MsaaRenderAdapter>,
    gfx_surface_config: Res<GfxSurfaceConfiguration>,
    scale_factor: Res<ScaleFactor>,
    mut cmd: Commands,
) {
    let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
        label: Some("content panel pipeline layout descriptor"),
        bind_group_layouts: &[(viewport.bind_group_layout())],
        push_constant_ranges: &[],
    };
    let pipeline_layout = gfx_surface
        .device
        .create_pipeline_layout(&pipeline_layout_descriptor);
    let shader = gfx_surface
        .device
        .create_shader_module(wgpu::include_wgsl!("panel.wgsl"));
    let color_target_state = [Some(wgpu::ColorTargetState {
        format: gfx_surface_config.configuration.format,
        blend: Some(BlendState::ALPHA_BLENDING),
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
                    array_stride: std::mem::size_of::<PanelVertex>() as wgpu::BufferAddress,
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
            format: viewport.depth_format(),
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: msaa.requested(),
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
    let mesh = generate_panel_mesh(16, scale_factor.factor());
    let mesh_len = mesh.len() as u32;
    let buffer = vertex_buffer(gfx_surface.as_ref(), mesh);
    let border_mesh = generate_border_mesh(16, scale_factor.factor());
    let border_mesh_len = border_mesh.len() as u32;
    let border_vertex_buffer = vertex_buffer(&gfx_surface, border_mesh);
    let initial_max = 5;
    let renderer = PanelRenderer {
        pipeline,
        positions: InstanceAttributeManager::new(&gfx_surface, initial_max),
        content_area: InstanceAttributeManager::new(&gfx_surface, initial_max),
        layers: InstanceAttributeManager::new(&gfx_surface, initial_max),
        panel_colors: InstanceAttributeManager::new(&gfx_surface, initial_max),
        panel_null_bits: InstanceAttributeManager::new(&gfx_surface, initial_max),
        panel_vertex_buffer: buffer,
        panel_mesh_len: mesh_len,
        border_null_bits: InstanceAttributeManager::new(&gfx_surface, initial_max),
        border_colors: InstanceAttributeManager::new(&gfx_surface, initial_max),
        border_vertex_buffer,
        border_mesh_len,
        indexer: Indexer::new(initial_max),
    };
    cmd.insert_resource(renderer);
}
impl Render for PanelRenderer {
    fn phase() -> RenderPhase {
        RenderPhase::Alpha(5)
    }
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        if self.indexer.has_instances() {
            render_pass_handle.0.set_pipeline(&self.pipeline);
            render_pass_handle
                .0
                .set_bind_group(0, viewport.bind_group(), &[]);
            render_pass_handle
                .0
                .set_vertex_buffer(0, self.panel_vertex_buffer.slice(..));
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
                .set_vertex_buffer(4, self.panel_colors.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(5, self.panel_null_bits.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .draw(0..self.panel_mesh_len, 0..self.indexer.count());
            render_pass_handle
                .0
                .set_vertex_buffer(0, self.border_vertex_buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(4, self.border_colors.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(5, self.border_null_bits.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .draw(0..self.border_mesh_len, 0..self.indexer.count());
        }
    }
}
