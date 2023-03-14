use bevy_ecs::prelude::Res;
use crate::{Color, Indexer, InstanceAttributeManager, Key, RawArea, RawPosition, Render, RenderPassHandle, RenderPhase, Viewport};
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration, MsaaRenderAttachment};
use crate::instance::key::KeyFactory;

pub(crate) struct ContentPanelRenderer{
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) positions: InstanceAttributeManager<RawPosition>,
    pub(crate) content_area: InstanceAttributeManager<RawArea>,
    pub(crate) colors: InstanceAttributeManager<Color>,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) indexer: Indexer<Key>,
    pub(crate) key_factory: KeyFactory,
}
pub(crate) fn setup(gfx_surface: Res<GfxSurface>, viewport: Res<Viewport>, msaa: Res<MsaaRenderAttachment>, gfx_surface_config: Res<GfxSurfaceConfiguration>) {
    let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
        label: Some("content panel pipeline layout descriptor"),
        bind_group_layouts: &[&viewport.bind_group_layout],
        push_constant_ranges: &[],
    };
    let pipeline_layout = gfx_surface.device.create_pipeline_layout(&pipeline_layout_descriptor);
    let shader = gfx_surface.device.create_shader_module(wgpu::include_wgsl!("content_panel.wgsl"));
    let descriptor = wgpu::RenderPipelineDescriptor{
        label: Some("content panel renderer"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState{
            module: &shader,
            entry_point: "vertex_entry",
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState{
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
        fragment: Some(wgpu::FragmentState{
            module: &shader,
            entry_point: "fragment_entry",
            targets: &[Some(wgpu::ColorTargetState{
                format: gfx_surface_config.configuration.format,
                blend: None,
                write_mask: Default::default(),
            })],
        }),
        multiview: None,
    };
    let pipeline = gfx_surface.device.create_render_pipeline(&descriptor);

}
impl Render for ContentPanelRenderer {
    fn phase() -> RenderPhase {
        RenderPhase::Opaque
    }
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        if self.indexer.has_instances() {
            render_pass_handle.0.set_pipeline(&self.pipeline);
            render_pass_handle.0.set_bind_group(0, &viewport.bind_group, &[]);
            render_pass_handle.0.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass_handle.0.set_vertex_buffer(1, self.positions.gpu.buffer.slice(..));
            render_pass_handle.0.set_vertex_buffer(2, self.content_area.gpu.buffer.slice(..));
            render_pass_handle.0.set_vertex_buffer(3, self.colors.gpu.buffer.slice(..));
        }
    }
}