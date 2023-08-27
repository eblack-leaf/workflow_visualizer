use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Commands, Entity, Res, Resource};
use tracing::trace;
use wgpu::util::DeviceExt;

use crate::gfx::{GfxSurface, GfxSurfaceConfiguration, MsaaRenderAdapter};
use crate::text::component::{Difference, TextScaleAlignment};
use crate::text::render_group::{RenderGroup, RenderGroupUniqueGlyphs};
use crate::texture_atlas::TextureCoordinates;
use crate::texture_atlas::{AtlasBlock, TextureBindGroup};
use crate::{
    Color, InterfaceContext, Layer, NullBit, Position, RawArea, RawPosition, Render,
    RenderPassHandle, RenderPhase, ScaleFactor, Viewport, VisibleSection, Visualizer,
};

pub(crate) const AABB: [Vertex; 6] = [
    Vertex::new(RawPosition { x: 0.0, y: 0.0 }),
    Vertex::new(RawPosition { x: 0.0, y: 1.0 }),
    Vertex::new(RawPosition { x: 1.0, y: 0.0 }),
    Vertex::new(RawPosition { x: 1.0, y: 0.0 }),
    Vertex::new(RawPosition { x: 0.0, y: 1.0 }),
    Vertex::new(RawPosition { x: 1.0, y: 1.0 }),
];

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub(crate) struct Vertex {
    pub position: RawPosition,
}

impl Vertex {
    pub const fn new(position: RawPosition) -> Self {
        Self { position }
    }
}

pub(crate) fn aabb_vertex_buffer(gfx_surface: &GfxSurface) -> wgpu::Buffer {
    gfx_surface
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("text vertex buffer"),
            contents: bytemuck::cast_slice(&AABB),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
}

#[cfg_attr(not(target_family = "wasm"), derive(Resource))]
pub(crate) struct TextRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) sampler_bind_group: wgpu::BindGroup,
    pub(crate) render_groups: HashMap<Entity, RenderGroup>,
    pub(crate) render_group_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) atlas_bind_group_layout: wgpu::BindGroupLayout,
}

#[derive(Resource, Clone)]
pub(crate) struct Extraction {
    pub(crate) added: HashMap<
        Entity,
        (
            u32,
            Position<InterfaceContext>,
            VisibleSection,
            Layer,
            RenderGroupUniqueGlyphs,
            TextScaleAlignment,
            AtlasBlock,
        ),
    >,
    pub(crate) removed: HashSet<Entity>,
    pub(crate) differences: HashMap<Entity, Difference>,
}

impl Extraction {
    pub(crate) fn new() -> Self {
        Self {
            added: HashMap::new(),
            removed: HashSet::new(),
            differences: HashMap::new(),
        }
    }
}

impl Render for TextRenderer {
    fn setup(
        _visualizer: &Visualizer,
        gfx: &GfxSurface,
        viewport: &Viewport,
        gfx_config: &GfxSurfaceConfiguration,
        msaa: &MsaaRenderAdapter,
        scale_factor: &ScaleFactor,
    ) -> Self {
        let sampler_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            label: Some("sampler bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            }],
        };
        let sampler_bind_group_layout = gfx
            .device
            .create_bind_group_layout(&sampler_bind_group_layout_descriptor);
        let sampler_descriptor = wgpu::SamplerDescriptor {
            label: Some("text sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            lod_min_clamp: Default::default(),
            lod_max_clamp: Default::default(),
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        };
        let sampler = gfx.device.create_sampler(&sampler_descriptor);
        let sampler_bind_group_descriptor = wgpu::BindGroupDescriptor {
            label: Some("sampler bind group"),
            layout: &sampler_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&sampler),
            }],
        };
        let sampler_bind_group = gfx.device.create_bind_group(&sampler_bind_group_descriptor);
        let render_group_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            label: Some("rasterization bind group"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        };
        let render_group_bind_group_layout = gfx
            .device
            .create_bind_group_layout(&render_group_bind_group_layout_descriptor);
        let atlas_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            label: Some("atlas bind group layout descriptor"),
            entries: &[TextureBindGroup::entry(0)],
        };
        let atlas_bind_group_layout = gfx
            .device
            .create_bind_group_layout(&atlas_bind_group_layout_descriptor);
        let layout_descriptor = wgpu::PipelineLayoutDescriptor {
            label: Some("text pipeline layout descriptor"),
            bind_group_layouts: &[
                viewport.bind_group_layout(),
                &sampler_bind_group_layout,
                &render_group_bind_group_layout,
                &atlas_bind_group_layout,
            ],
            push_constant_ranges: &[],
        };
        let layout = gfx.device.create_pipeline_layout(&layout_descriptor);
        let shader = gfx
            .device
            .create_shader_module(wgpu::include_wgsl!("padded_text.wgsl"));
        let vertex_state = wgpu::VertexState {
            module: &shader,
            entry_point: "vertex_entry",
            buffers: &[
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
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
                    array_stride: std::mem::size_of::<NullBit>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![3 => Uint32],
                },
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Color>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![4 => Float32x4],
                },
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<TextureCoordinates>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![5 => Float32x4],
                },
            ],
        };
        let primitive_state = wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        };
        let depth_stencil_state = Some(wgpu::DepthStencilState {
            format: viewport.depth_format(),
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });
        let fragment_state = wgpu::FragmentState {
            module: &shader,
            entry_point: "fragment_entry",
            targets: &[Some(gfx_config.alpha_color_target_state())],
        };
        let descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("text pipeline"),
            layout: Some(&layout),
            vertex: vertex_state,
            primitive: primitive_state,
            depth_stencil: depth_stencil_state,
            multisample: msaa.multisample_state(),
            fragment: Some(fragment_state),
            multiview: None,
        };
        let pipeline = gfx.device.create_render_pipeline(&descriptor);
        let renderer = TextRenderer {
            pipeline,
            vertex_buffer: aabb_vertex_buffer(&gfx),
            sampler_bind_group,
            render_groups: HashMap::new(),
            render_group_bind_group_layout,
            atlas_bind_group_layout,
        };
        renderer
    }

    fn phase() -> RenderPhase {
        RenderPhase::Alpha(0)
    }
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        trace!("starting text render fn");
        render_pass_handle.0.set_pipeline(&self.pipeline);
        render_pass_handle
            .0
            .set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass_handle
            .0
            .set_bind_group(0, viewport.bind_group(), &[]);
        render_pass_handle
            .0
            .set_bind_group(1, &self.sampler_bind_group, &[]);
        for (_, render_group) in self.render_groups.iter() {
            if render_group.indexer.has_instances() {
                render_pass_handle.0.set_bind_group(
                    2,
                    &render_group.render_group_bind_group.bind_group,
                    &[],
                );
                render_pass_handle.0.set_bind_group(
                    3,
                    &render_group.atlas_bind_group.bind_group,
                    &[],
                );
                render_pass_handle
                    .0
                    .set_vertex_buffer(1, render_group.glyph_positions.gpu.buffer.slice(..));
                render_pass_handle
                    .0
                    .set_vertex_buffer(2, render_group.glyph_areas.gpu.buffer.slice(..));
                render_pass_handle
                    .0
                    .set_vertex_buffer(3, render_group.null_bits.gpu.buffer.slice(..));
                render_pass_handle
                    .0
                    .set_vertex_buffer(4, render_group.glyph_colors.gpu.buffer.slice(..));
                render_pass_handle
                    .0
                    .set_vertex_buffer(5, render_group.glyph_tex_coords.gpu.buffer.slice(..));
                if let Some(draw_section) = render_group.draw_section.section {
                    render_pass_handle.0.set_scissor_rect(
                        draw_section.position.x as u32,
                        draw_section.position.y as u32,
                        draw_section.area.width.max(1.0) as u32,
                        draw_section.area.height.max(1.0) as u32,
                    )
                }
                render_pass_handle
                    .0
                    .draw(0..AABB.len() as u32, 0..render_group.indexer.count());
                if render_group.draw_section.section.is_some() {
                    render_pass_handle.0.set_scissor_rect(
                        0,
                        0,
                        viewport.as_section().area.width as u32,
                        viewport.as_section().area.height as u32,
                    )
                }
            }
        }
    }
}
