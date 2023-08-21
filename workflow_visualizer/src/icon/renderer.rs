use bevy_ecs::prelude::{Entity, Resource};
use wgpu::util::DeviceExt;

use crate::gfx::{GfxSurfaceConfiguration, MsaaRenderAdapter};
use crate::icon::bitmap::{
    IconBitmapLayout, IconBitmapRequestManager, IconPixelData, ICON_BITMAP_DIMENSION,
};
use crate::texture_atlas::AtlasLocation;
use crate::{
    AtlasBlock, AtlasDimension, Color, GfxSurface, Indexer, InstanceAttributeManager, Layer,
    NullBit, RawArea, RawPosition, Render, RenderPassHandle, RenderPhase, ScaleFactor,
    TextureAtlas, TextureBindGroup, TextureCoordinates, Viewport, Visualizer,
};

#[cfg_attr(not(target_family = "wasm"), derive(Resource))]
pub(crate) struct IconRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_quad: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    pub(crate) pos_attribute: InstanceAttributeManager<RawPosition>,
    pub(crate) area_attribute: InstanceAttributeManager<RawArea>,
    pub(crate) layer_attribute: InstanceAttributeManager<Layer>,
    pub(crate) color_attribute: InstanceAttributeManager<Color>,
    pub(crate) tex_coords_attribute: InstanceAttributeManager<TextureCoordinates>,
    pub(crate) null_bit_attribute: InstanceAttributeManager<NullBit>,
    pub(crate) indexer: Indexer<Entity>,
    pub(crate) atlas: TextureAtlas,
    pub(crate) icon_bitmap_layout: IconBitmapLayout,
}

impl Render for IconRenderer {
    fn setup(
        visualizer: &Visualizer,
        gfx: &GfxSurface,
        viewport: &Viewport,
        gfx_config: &GfxSurfaceConfiguration,
        msaa: &MsaaRenderAdapter,
        scale_factor: &ScaleFactor,
    ) -> Self {
        let bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            label: Some("icon renderer bind group layout"),
            entries: &[
                TextureBindGroup::entry(0),
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        };
        let bind_group_layout = gfx
            .device
            .create_bind_group_layout(&bind_group_layout_descriptor);
        let mut writes = Vec::new();
        let manager = visualizer
            .job
            .container
            .get_resource::<IconBitmapRequestManager>()
            .unwrap();
        for request in manager.requests.iter() {
            writes.push(request.clone());
        }
        let mut icon_bitmap_layout = IconBitmapLayout::new();
        let dimension = AtlasDimension::new(((writes.len() as f32).sqrt().ceil() as u32).max(1));
        let block = AtlasBlock::new((ICON_BITMAP_DIMENSION, ICON_BITMAP_DIMENSION));
        let atlas = TextureAtlas::new(&gfx, block, dimension, wgpu::TextureFormat::R8Unorm);
        let mut x_index = 0;
        let mut y_index = 0;
        for mut write in writes {
            let atlas_location = AtlasLocation::new(x_index, y_index);
            let coordinates = atlas.write::<IconPixelData>(
                atlas_location,
                bytemuck::cast_slice(&write.bitmap.data),
                block.block,
                &gfx,
            );
            icon_bitmap_layout
                .bitmap_locations
                .insert(write.id, coordinates);
            if x_index + 1 >= dimension.dimension {
                x_index = 0;
                y_index += 1;
            } else {
                x_index += 1;
            }
        }
        let sampler_descriptor = wgpu::SamplerDescriptor {
            label: Some("icon renderer sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        };
        let sampler = gfx.device.create_sampler(&sampler_descriptor);
        let bind_group_descriptor = wgpu::BindGroupDescriptor {
            label: Some("icon renderer bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(atlas.view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        };
        let bind_group = gfx.device.create_bind_group(&bind_group_descriptor);
        let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
            label: Some("icon renderer layout"),
            bind_group_layouts: &[viewport.bind_group_layout(), &bind_group_layout],
            push_constant_ranges: &[],
        };
        let pipeline_layout = gfx
            .device
            .create_pipeline_layout(&pipeline_layout_descriptor);
        let shader = gfx
            .device
            .create_shader_module(wgpu::include_wgsl!("icon.wgsl"));
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
                    array_stride: std::mem::size_of::<TextureCoordinates>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![5 => Float32x4],
                },
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<NullBit>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![6 => Uint32],
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
        let depth_stencil_state = viewport.depth_stencil_state();
        let fragment_state = wgpu::FragmentState {
            module: &shader,
            entry_point: "fragment_entry",
            targets: &[Some(wgpu::ColorTargetState {
                format: gfx_config.configuration.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: Default::default(),
            })],
        };
        let pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("icon renderer"),
            layout: Some(&pipeline_layout),
            vertex: vertex_state,
            primitive: primitive_state,
            depth_stencil: Some(depth_stencil_state),
            multisample: msaa.multisample_state(),
            fragment: Some(fragment_state),
            multiview: None,
        };
        let pipeline = gfx.device.create_render_pipeline(&pipeline_descriptor);
        let vertex_quad = aabb_vertex_buffer(&gfx);
        let max = 10;
        let renderer = IconRenderer {
            pipeline,
            vertex_quad,
            bind_group,
            pos_attribute: InstanceAttributeManager::new(&gfx, max),
            area_attribute: InstanceAttributeManager::new(&gfx, max),
            tex_coords_attribute: InstanceAttributeManager::new(&gfx, max),
            color_attribute: InstanceAttributeManager::new(&gfx, max),
            layer_attribute: InstanceAttributeManager::new(&gfx, max),
            null_bit_attribute: InstanceAttributeManager::new(&gfx, max),
            indexer: Indexer::new(max),
            atlas,
            icon_bitmap_layout,
        };
        renderer
    }

    fn phase() -> RenderPhase {
        RenderPhase::Alpha(2)
    }
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        if self.indexer.has_instances() {
            render_pass_handle.0.set_pipeline(&self.pipeline);
            render_pass_handle
                .0
                .set_bind_group(0, viewport.bind_group(), &[]);
            render_pass_handle
                .0
                .set_bind_group(1, &self.bind_group, &[]);
            render_pass_handle
                .0
                .set_vertex_buffer(0, self.vertex_quad.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(1, self.pos_attribute.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(2, self.area_attribute.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(3, self.layer_attribute.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(4, self.color_attribute.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(5, self.tex_coords_attribute.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(6, self.null_bit_attribute.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .draw(0..AABB.len() as u32, 0..self.indexer.count());
        }
    }
}

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

pub(crate) const AABB: [Vertex; 6] = [
    Vertex::new(RawPosition { x: 0.0, y: 0.0 }),
    Vertex::new(RawPosition { x: 0.0, y: 1.0 }),
    Vertex::new(RawPosition { x: 1.0, y: 0.0 }),
    Vertex::new(RawPosition { x: 1.0, y: 0.0 }),
    Vertex::new(RawPosition { x: 0.0, y: 1.0 }),
    Vertex::new(RawPosition { x: 1.0, y: 1.0 }),
];

pub(crate) fn aabb_vertex_buffer(gfx_surface: &GfxSurface) -> wgpu::Buffer {
    gfx_surface
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("icon vertex buffer"),
            contents: bytemuck::cast_slice(&AABB),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
}
