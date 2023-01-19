use crate::canvas::Viewport;
use crate::render::{Render, RenderPassHandle, RenderPhase};
use crate::text::changes::Changes;
use crate::text::extract::Extraction;
use crate::text::font::Font;
use crate::text::instance::Instance;
use crate::text::vertex::{Vertex, GLYPH_AABB};
use crate::text::Renderer;
use crate::{Canvas, Task};
use bevy_ecs::prelude::{Commands, Res, Resource};
use std::collections::{HashMap, HashSet};
use wgpu::util::DeviceExt;
use wgpu::{
    include_wgsl, vertex_attr_array, BindGroup, BindGroupLayout, Buffer, RenderPipeline, Sampler,
    SamplerBindingType, TextureSampleType, TextureViewDimension, VertexState,
};

impl Render for Renderer {
    fn extract(compute: &mut Task, render: &mut Task)
    where
        Self: Sized,
    {
        let mut changes = compute
            .container
            .get_resource_mut::<Changes>()
            .expect("no text changes attached");
        let mut extraction_changes = &mut render
            .container
            .get_resource_mut::<Extraction>()
            .expect("no extraction attached")
            .changes;
        extraction_changes.adds = changes.adds.drain().collect();
        extraction_changes.updates = changes.updates.drain().collect();
        extraction_changes.removes = changes.removes.drain().collect();
        extraction_changes.glyphs = changes.glyphs.drain().collect();
        extraction_changes.bounds = changes.bounds.drain().collect();
        extraction_changes.added_text_entities = changes.added_text_entities.drain().collect();
        extraction_changes.removed_text_entities = changes.removed_text_entities.drain().collect();
        extraction_changes.removed_glyphs = changes.removed_glyphs.drain().collect();
    }

    fn phase() -> RenderPhase {
        RenderPhase::Alpha
    }

    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        render_pass_handle.0.set_pipeline(&self.pipeline);
        render_pass_handle
            .0
            .set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass_handle
            .0
            .set_bind_group(0, &viewport.bind_group, &[]);
        render_pass_handle
            .0
            .set_bind_group(1, &self.sampler_bind_group, &[]);
        for (entity, rasterization) in self.rasterizations.iter() {
            if rasterization.instances.count() > 0 && self.visible_text_entities.contains(entity) {
                render_pass_handle
                    .0
                    .set_vertex_buffer(1, rasterization.instances.gpu.slice(..));
                render_pass_handle
                    .0
                    .set_bind_group(2, &rasterization.bind_group, &[]);
                if let Some(bound) = &rasterization.bounds {
                    render_pass_handle.0.set_scissor_rect(
                        bound.position.x as u32,
                        bound.position.y as u32,
                        bound.area.width as u32,
                        bound.area.height as u32,
                    );
                }
                render_pass_handle.0.draw(
                    0..GLYPH_AABB.len() as u32,
                    0..rasterization.instances.count() as u32,
                );
            }
        }
    }
}

pub fn render_setup(canvas: Res<Canvas>, mut cmd: Commands) {
    let (sampler_bind_group_layout, sampler, sampler_bind_group) = sampler_resources(&canvas);
    let rasterization_bind_group_layout = rasterization_resources(&canvas);
    let pipeline = pipeline(
        &canvas,
        &sampler_bind_group_layout,
        &rasterization_bind_group_layout,
    );
    let vertex_buffer = vertex_buffer(&canvas);
    cmd.insert_resource(Renderer {
        pipeline,
        vertex_buffer,
        rasterizations: HashMap::new(),
        rasterization_bind_group_layout,
        visible_text_entities: HashSet::new(),
        sampler,
        sampler_bind_group,
    });
    cmd.insert_resource(Extraction::new());
    cmd.insert_resource(Font::default());
}

fn vertex_buffer(canvas: &Canvas) -> Buffer {
    canvas
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("text vertex buffer"),
            contents: bytemuck::cast_slice(&GLYPH_AABB),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
}

fn pipeline(
    canvas: &Canvas,
    sampler_bind_group_layout: &BindGroupLayout,
    rasterization_bind_group_layout: &BindGroupLayout,
) -> RenderPipeline {
    let layout_descriptor = wgpu::PipelineLayoutDescriptor {
        label: Some("text pipeline layout descriptor"),
        bind_group_layouts: &[
            &canvas.viewport.bind_group_layout,
            &sampler_bind_group_layout,
            &rasterization_bind_group_layout,
        ],
        push_constant_ranges: &[],
    };
    let layout = canvas.device.create_pipeline_layout(&layout_descriptor);
    let shader = canvas
        .device
        .create_shader_module(include_wgsl!("text.wgsl"));
    let vertex_state = VertexState {
        module: &shader,
        entry_point: "vertex_entry",
        buffers: &[
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &vertex_attr_array![0 => Float32x2],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &vertex_attr_array![
                    1 => Float32x2,
                    3 => Float32,
                    4 => Float32x4,
                    2 => Float32x2,
                    5 => Float32x4,
                ],
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
        format: canvas.viewport.depth_format,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    });
    let fragment_state = wgpu::FragmentState {
        module: &shader,
        entry_point: "fragment_entry",
        targets: &[Some(wgpu::ColorTargetState {
            format: canvas.surface_configuration.format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: Default::default(),
        })],
    };
    let descriptor = wgpu::RenderPipelineDescriptor {
        label: Some("text pipeline"),
        layout: Some(&layout),
        vertex: vertex_state,
        primitive: primitive_state,
        depth_stencil: depth_stencil_state,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(fragment_state),
        multiview: None,
    };
    let pipeline = canvas.device.create_render_pipeline(&descriptor);
    pipeline
}

fn rasterization_resources(canvas: &Canvas) -> BindGroupLayout {
    let rasterization_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
        label: Some("rasterization bind group"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: TextureSampleType::Uint,
                view_dimension: TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        }],
    };
    let rasterization_bind_group_layout = canvas
        .device
        .create_bind_group_layout(&rasterization_bind_group_layout_descriptor);
    rasterization_bind_group_layout
}

fn sampler_resources(canvas: &Canvas) -> (BindGroupLayout, Sampler, BindGroup) {
    let sampler_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
        label: Some("sampler bind group layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(SamplerBindingType::Filtering),
            count: None,
        }],
    };
    let sampler_bind_group_layout = canvas
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
        anisotropy_clamp: None,
        border_color: None,
    };
    let sampler = canvas.device.create_sampler(&sampler_descriptor);
    let sampler_bind_group_descriptor = wgpu::BindGroupDescriptor {
        label: Some("sampler bind group"),
        layout: &sampler_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Sampler(&sampler),
        }],
    };
    let sampler_bind_group = canvas
        .device
        .create_bind_group(&sampler_bind_group_descriptor);
    (sampler_bind_group_layout, sampler, sampler_bind_group)
}
