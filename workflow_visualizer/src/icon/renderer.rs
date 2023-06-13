use std::collections::HashMap;

use bevy_ecs::prelude::{Commands, Component, Entity, Query, Res, Resource};
use bytemuck::{Pod, Zeroable};
use compact_str::CompactString;
use wgpu::util::DeviceExt;

use crate::{
    Area, Color, GfxSurface, Indexer, InstanceAttributeManager, InterfaceContext, Key, KeyFactory,
    Layer, NullBit, NumericalContext, Position, RawArea, RawPosition, Render, RenderPassHandle,
    RenderPhase, Section, Viewport,
};
use crate::gfx::{GfxSurfaceConfiguration, MsaaRenderAdapter};
use crate::icon::bitmap::{
    ICON_BITMAP_DIMENSION, IconBitmapLayout, IconBitmapRequest, IconPixelData, TextureCoordinates,
};
use crate::icon::component::{ColorInvert, IconId};

#[derive(Resource)]
pub(crate) struct IconRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_quad: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    pub(crate) pos_attribute: InstanceAttributeManager<RawPosition>,
    pub(crate) area_and_layer_attribute: InstanceAttributeManager<AreaAndLayer>,
    pub(crate) tex_coords_attribute: InstanceAttributeManager<TextureCoordinates>,
    pub(crate) positive_space_color_attribute: InstanceAttributeManager<Color>,
    pub(crate) negative_space_color_attribute: InstanceAttributeManager<Color>,
    pub(crate) layer_attribute: InstanceAttributeManager<Layer>,
    pub(crate) color_invert_attribute: InstanceAttributeManager<ColorInvert>,
    pub(crate) null_bit_attribute: InstanceAttributeManager<NullBit>,
    pub(crate) indexer: Indexer<Entity>,
}

impl Render for IconRenderer {
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
                .set_vertex_buffer(2, self.area_and_layer_attribute.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(3, self.tex_coords_attribute.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(4, self.positive_space_color_attribute.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(5, self.negative_space_color_attribute.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(6, self.color_invert_attribute.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .set_vertex_buffer(7, self.null_bit_attribute.gpu.buffer.slice(..));
            render_pass_handle
                .0
                .draw(0..AABB.len() as u32, 0..self.indexer.count());
        }
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Component)]
pub(crate) struct AreaAndLayer {
    pub(crate) data: [f32; 3],
}

impl AreaAndLayer {
    pub fn new() -> Self {
        Self {
            data: [0.0, 0.0, 0.0],
        }
    }
}

impl Default for AreaAndLayer {
    fn default() -> Self {
        AreaAndLayer::new()
    }
}
fn icon_bitmap_bytes() -> u32 {
    let mem = std::mem::size_of::<IconPixelData>() as u32;
    let val = ICON_BITMAP_DIMENSION * mem;
    val
}

pub(crate) fn setup(
    gfx: Res<GfxSurface>,
    msaa: Res<MsaaRenderAdapter>,
    viewport: Res<Viewport>,
    gfx_config: Res<GfxSurfaceConfiguration>,
    requests: Query<(Entity, &IconBitmapRequest)>,
    mut cmd: Commands,
) {
    let bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
        label: Some("icon renderer bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
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
    for (entity, request) in requests.iter() {
        writes.push(request.clone());
        cmd.entity(entity).despawn();
    }
    let dimension = (writes.len() as f32 / 2f32).ceil() as u32;
    let total_dimension = dimension * ICON_BITMAP_DIMENSION;
    let total_dimension = total_dimension.max(1);
    let texture_descriptor = wgpu::TextureDescriptor {
        label: Some("icon texture descriptor"),
        size: wgpu::Extent3d {
            width: total_dimension,
            height: total_dimension,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
    };
    let texture = gfx.device.create_texture(&texture_descriptor);
    let mut x_index = 0;
    let mut y_index = 0;
    let mut icon_bitmap_layout = IconBitmapLayout::new();
    for mut write in writes {
        let bytes = icon_bitmap_bytes();
        let image_copy_texture = wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d {
                x: x_index * bytes,
                y: y_index * bytes,
                z: 0,
            },
            aspect: wgpu::TextureAspect::All,
        };
        let image_data_layout = wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(bytes),
            rows_per_image: Some(bytes),
        };
        let extent = wgpu::Extent3d {
            width: ICON_BITMAP_DIMENSION,
            height: ICON_BITMAP_DIMENSION,
            depth_or_array_layers: 1,
        };
        let mut raw_data = Vec::new();
        for d in write.bitmap.data.drain(..) {
            raw_data.extend(vec![d.data[0], d.data[1], d.data[2], d.data[3]]);
        }
        gfx.queue.write_texture(
            image_copy_texture,
            raw_data.as_slice(),
            image_data_layout,
            extent,
        );
        let l = x_index * ICON_BITMAP_DIMENSION / total_dimension;
        let t = y_index * ICON_BITMAP_DIMENSION / total_dimension;
        let pos = Position::from((l, t));
        let normalized_width_height = ICON_BITMAP_DIMENSION / total_dimension;
        let area = Area::from((normalized_width_height, normalized_width_height));
        let section = Section::<NumericalContext>::new(pos, area);
        let coordinates = TextureCoordinates {
            data: [
                section.left(),
                section.top(),
                section.right(),
                section.bottom(),
            ],
        };
        icon_bitmap_layout
            .bitmap_locations
            .insert(write.id, coordinates);
        if x_index + 1 >= dimension {
            x_index = 0;
            y_index += 1;
        } else {
            x_index += 1;
        }
    }
    cmd.insert_resource(icon_bitmap_layout);
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
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
                resource: wgpu::BindingResource::TextureView(&texture_view),
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
                array_stride: std::mem::size_of::<AreaAndLayer>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![2 => Float32x3],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<TextureCoordinates>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![3 => Float32x4],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Color>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![4 => Float32x4],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Color>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![5 => Float32x4],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<ColorInvert>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![6 => Uint32],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<NullBit>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![7 => Uint32],
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
    let depth_stencil_state = wgpu::DepthStencilState {
        format: viewport.depth_format(),
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: Default::default(),
        bias: Default::default(),
    };
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
    cmd.insert_resource(IconRenderer {
        pipeline,
        vertex_quad,
        bind_group,
        pos_attribute: InstanceAttributeManager::new(&gfx, max),
        area_and_layer_attribute: InstanceAttributeManager::new(&gfx, max),
        tex_coords_attribute: InstanceAttributeManager::new(&gfx, max),
        positive_space_color_attribute: InstanceAttributeManager::new(&gfx, max),
        negative_space_color_attribute: InstanceAttributeManager::new(&gfx, max),
        layer_attribute: InstanceAttributeManager::new(&gfx, max),
        color_invert_attribute: InstanceAttributeManager::new(&gfx, max),
        null_bit_attribute: InstanceAttributeManager::new(&gfx, max),
        indexer: Indexer::new(max),
    });
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
            label: Some("text vertex buffer"),
            contents: bytemuck::cast_slice(&AABB),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
}
