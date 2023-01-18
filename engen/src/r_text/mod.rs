mod font;
mod index;

use crate::canvas::Viewport;
use crate::r_text::index::Indexer;
use crate::render::{Render, RenderPassHandle, RenderPhase};
use crate::task::Stage;
use crate::{Area, Attach, Canvas, Color, Depth, Engen, Position, Section, Task};
use bevy_ecs::prelude::{
    Added, Bundle, Changed, Commands, Component, Entity, Query, RemovedComponents, Res, ResMut,
    Resource,
};
pub(crate) use font::Font;
use std::collections::{HashMap, HashSet};
use wgpu::util::DeviceExt;
use wgpu::{
    include_wgsl, vertex_attr_array, BufferAddress, Extent3d, SamplerBindingType, TextureDimension,
    TextureFormat, TextureSampleType, TextureUsages, TextureViewDescriptor, TextureViewDimension,
    VertexAttribute, VertexState,
};

#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub scale: Scale,
    pub position: Position,
    pub depth: Depth,
    pub color: Color,
    // auto made
    pub(crate) placer: Placer,
    pub(crate) keys: Keys,
}
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct TextOffset(pub usize);
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct Key {
    pub entity: Entity,
    pub offset: TextOffset,
}
impl Key {
    pub(crate) fn new(entity: Entity, offset: TextOffset) -> Self {
        Self { entity, offset }
    }
}
#[derive(Component)]
pub struct Text {
    pub string: String,
}
#[derive(Component, Clone, Copy)]
pub struct Scale {
    pub scale: f32,
}
impl Scale {
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }
    pub fn px(&self) -> f32 {
        self.scale
    }
}
impl From<f32> for Scale {
    fn from(scale: f32) -> Self {
        Self { scale }
    }
}
impl From<u32> for Scale {
    fn from(scale: u32) -> Self {
        Self {
            scale: scale as f32,
        }
    }
}
#[derive(Component)]
pub struct Placer {
    pub placer: fontdue::layout::Layout,
}
#[derive(Component)]
pub(crate) struct Keys {
    pub keys: HashSet<Key>,
}
pub(crate) struct Alignment {
    pub(crate) dimensions: [f32; 2],
}
impl Alignment {
    pub(crate) fn new(dimensions: [f32; 2]) -> Self {
        Self { dimensions }
    }
}
#[derive(Resource)]
pub(crate) struct Changes {
    pub added_text_entities: HashMap<Entity, (usize, Alignment)>,
    pub removed_text_entities: HashSet<Entity>,
    pub adds: HashMap<Key, Attributes>,
    pub updates: HashMap<Key, Attributes>,
    pub removes: HashSet<Key>,
    pub glyphs: HashMap<Key, GlyphHash>,
    pub bounds: HashMap<Entity, Section>,
    pub removed_bounds: HashSet<Entity>,
}
impl Changes {
    pub(crate) fn new() -> Self {
        Self {
            added_text_entities: HashMap::new(),
            removed_text_entities: HashSet::new(),
            adds: HashMap::new(),
            updates: HashMap::new(),
            removes: HashSet::new(),
            glyphs: HashMap::new(),
            bounds: HashMap::new(),
            removed_bounds: HashSet::new(),
        }
    }
}
pub(crate) type GlyphHash = fontdue::layout::GlyphRasterConfig;
#[derive(Resource)]
pub(crate) struct Cache {
    pub glyphs: HashMap<Key, GlyphHash>,
    pub attributes: HashMap<Key, Attributes>,
    pub bounds: HashMap<Entity, Section>,
}
impl Cache {
    pub(crate) fn new() -> Self {
        Self {
            glyphs: HashMap::new(),
            attributes: HashMap::new(),
            bounds: HashMap::new(),
        }
    }
}
pub(crate) struct InstanceBuffer {
    pub(crate) cpu: Vec<Instance>,
    pub(crate) gpu: wgpu::Buffer,
    pub(crate) indexer: Indexer<Key>,
}
impl InstanceBuffer {
    pub(crate) fn new(canvas: &Canvas, initial_supported_instances: usize) -> Self {
        Self {
            cpu: Vec::new(),
            gpu: canvas.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("text instance buffer"),
                size: (std::mem::size_of::<Instance>() * initial_supported_instances)
                    as BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            indexer: Indexer::new(initial_supported_instances),
        }
    }
    pub(crate) fn count(&self) -> usize {
        self.cpu.len()
    }
}
const GLYPH_AABB: [Vertex; 6] = [
    Vertex::new(Position::new(0.0, 0.0)),
    Vertex::new(Position::new(0.0, 1.0)),
    Vertex::new(Position::new(1.0, 0.0)),
    Vertex::new(Position::new(1.0, 0.0)),
    Vertex::new(Position::new(0.0, 1.0)),
    Vertex::new(Position::new(1.0, 1.0)),
];
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub(crate) struct Vertex {
    pub position: Position,
}
impl Vertex {
    pub const fn new(position: Position) -> Self {
        Self { position }
    }
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct Attributes {
    pub position: Position,
    pub area: Area,
    pub depth: Depth,
    pub color: Color,
}
impl Attributes {
    pub fn new(position: Position, area: Area, depth: Depth, color: Color) -> Self {
        Self {
            position,
            area,
            depth,
            color,
        }
    }
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct TexCoords {
    pub coords: [f32; 4],
}
impl TexCoords {
    pub fn new(coords: [f32; 4]) -> Self {
        Self { coords }
    }
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct Instance {
    pub attributes: Attributes,
    pub tex_coords: TexCoords,
}
pub(crate) struct TextureAtlas {
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) grid: Grid,
}
#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) struct GridLocation {
    pub(crate) x: usize,
    pub(crate) y: usize,
}
impl GridLocation {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
pub(crate) type Bitmap = Vec<u8>;
pub(crate) struct GridEntry {
    pub(crate) bitmap: Bitmap,
    pub(crate) area: Area,
}
impl GridEntry {
    pub(crate) fn new(bitmap: Bitmap, area: Area) -> Self {
        Self { bitmap, area }
    }
}
pub(crate) struct Grid {
    pub(crate) grid: HashMap<GridLocation, GridEntry>,
    pub(crate) alignment: Alignment,
    pub(crate) logical_dimension: usize,
    pub(crate) next_location: GridLocation,
}
impl Grid {
    pub(crate) fn new(alignment: Alignment, logical_dimension: usize) -> Self {
        Self {
            grid: HashMap::new(),
            alignment,
            logical_dimension,
            next_location: GridLocation::new(0, 0),
        }
    }
    pub(crate) fn position(&self, grid_location: GridLocation) -> Position {
        (
            grid_location.x as f32 * self.alignment.dimensions[0],
            grid_location.y as f32 * self.alignment.dimensions[1],
        )
            .into()
    }
    pub(crate) fn total_x(&self) -> f32 {
        self.alignment.dimensions[0] * self.logical_dimension as f32
    }
    pub(crate) fn total_y(&self) -> f32 {
        self.alignment.dimensions[1] * self.logical_dimension as f32
    }
    pub(crate) fn place(&mut self, grid_entry: GridEntry) -> TexCoords {
        let position = self.position(self.next_location);
        let normalized_position: Position =
            (position.x / self.total_x(), position.y / self.total_y()).into();
        let normalized_area: Area = (
            grid_entry.area.width / self.total_x(),
            grid_entry.area.height / self.total_y(),
        )
            .into();
        self.grid.insert(self.next_location, grid_entry);
        let next = match self.next_location.x + 1 == self.logical_dimension {
            true => {
                let x = 0;
                let y = self.next_location.y.min(self.logical_dimension - 1);
                GridLocation::new(x, y)
            }
            false => {
                let x = self.next_location.x + 1;
                let y = self.next_location.y;
                GridLocation::new(x, y)
            }
        };
        self.next_location = next;
        TexCoords::new([
            normalized_position.x,
            normalized_position.y,
            normalized_position.x + normalized_area.width,
            normalized_position.y + normalized_area.height,
        ])
    }
}
impl TextureAtlas {
    pub(crate) fn new(
        canvas: &Canvas,
        alignment: Alignment,
        initial_supported_instances: usize,
    ) -> Self {
        let mut logical_dimension = (initial_supported_instances as f32).sqrt() as usize;
        while logical_dimension.pow(2) > initial_supported_instances {
            logical_dimension += 1;
        }
        let texture_width: u32 = (logical_dimension * alignment.dimensions[0] as usize) as u32;
        let texture_height: u32 = (logical_dimension * alignment.dimensions[1] as usize) as u32;
        let hardware_max = canvas.options.limits.max_texture_dimension_2d;
        if texture_width > hardware_max {
            panic!("requested larger than possible texture")
        }
        if texture_height > hardware_max {
            panic!("requested larger than possible texture")
        }
        let texture_descriptor = wgpu::TextureDescriptor {
            label: Some("texture atlas"),
            size: Extent3d {
                width: texture_width,
                height: texture_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::R8Uint,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        };
        let texture = canvas.device.create_texture(&texture_descriptor);
        let view = texture.create_view(&TextureViewDescriptor::default());
        let grid = Grid::new(alignment, logical_dimension);
        Self {
            texture,
            view,
            grid,
        }
    }
}
pub(crate) struct Rasterization {
    pub(crate) instances: InstanceBuffer,
    pub(crate) texture_atlas: TextureAtlas,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) bounds: Option<Section>,
}
impl Rasterization {
    pub(crate) fn new(
        canvas: &Canvas,
        bind_group_layout: &wgpu::BindGroupLayout,
        alignment: Alignment,
        initial_supported_instances: usize,
    ) -> Self {
        let instances = InstanceBuffer::new(canvas, initial_supported_instances);
        let texture_atlas = TextureAtlas::new(canvas, alignment, initial_supported_instances);
        let descriptor = wgpu::BindGroupDescriptor {
            label: Some("texture atlas bind group"),
            layout: bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_atlas.view),
            }],
        };
        let bind_group = canvas.device.create_bind_group(&descriptor);
        Self {
            instances,
            texture_atlas,
            bind_group,
            bounds: None,
        }
    }
}
#[derive(Resource)]
pub struct Renderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) rasterizations: HashMap<Entity, Rasterization>,
    pub(crate) rasterization_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) sampler_bind_group: wgpu::BindGroup,
}
pub fn render_setup(canvas: Res<Canvas>, mut cmd: Commands) {
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
        lod_min_clamp: 0.0,
        lod_max_clamp: 0.0,
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
                    2 => Float32x2,
                    3 => Float32,
                    4 => Float32x4,
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
    let vertex_buffer = canvas
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("text vertex buffer"),
            contents: bytemuck::cast_slice(&GLYPH_AABB),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
    let rasterizations = HashMap::new();
    cmd.insert_resource(Renderer {
        pipeline,
        vertex_buffer,
        rasterizations,
        rasterization_bind_group_layout,
        sampler,
        sampler_bind_group,
    });
    cmd.insert_resource(Extraction::new());
}
pub(crate) fn compute_setup(mut cmd: Commands) {
    cmd.insert_resource(Changes::new());
    cmd.insert_resource(Cache::new());
    cmd.insert_resource(Font::default());
}
pub(crate) fn emit(
    mut cache: ResMut<Cache>,
    mut changes: ResMut<Changes>,
    mut text: Query<
        (
            Entity,
            &Text,
            &mut Placer,
            &mut Keys,
            &Position,
            Option<&Area>,
            &Depth,
            &Color,
            &Scale,
        ),
        (Changed<Text>),
    >,
    font: Res<Font>,
) {
    for (entity, text, mut placer, mut keys, position, maybe_area, depth, color, scale) in
        text.iter_mut()
    {
        placer.placer.clear();
        placer.placer.append(
            font.font_slice(),
            &fontdue::layout::TextStyle::new(text.string.as_str(), scale.px(), Font::index()),
        );
        let mut retained_keys = HashSet::new();
        let mut added_keys = HashSet::new();
        if let Some(area) = maybe_area {
            cache.bounds.insert(entity, (*position, *area).into());
            changes.bounds.insert(entity, (*position, *area).into());
        } else {
            let old_bound = cache.bounds.remove(&entity);
            if old_bound.is_some() {
                changes.removed_bounds.insert(entity);
            }
        }
        for glyph in placer.placer.glyphs() {
            let key = Key::new(entity, TextOffset(glyph.byte_offset));
            let current_attributes = Attributes::new(
                *position + (glyph.x, glyph.y).into(),
                (glyph.width, glyph.height).into(),
                *depth,
                *color,
            );
            if cache.attributes.contains_key(&key) {
                retained_keys.insert(key);
                let cached_glyph = cache.glyphs.get(&key).expect("no cached glyph for key");
                if *cached_glyph != glyph.key {
                    changes.glyphs.insert(key, glyph.key);
                }
                let cached_attributes = cache.attributes.get(&key).expect("no cached attributes");
                // tolerance check each value to decide if should be replaced ond go to cache.changes.updates
                // also store in cache if changed
            } else {
                added_keys.insert(key);
                changes.adds.insert(key, current_attributes);
                cache.attributes.insert(key, current_attributes);
                changes.glyphs.insert(key, glyph.key);
                cache.glyphs.insert(key, glyph.key);
            }
        }
        let keys_to_remove = keys
            .keys
            .difference(&retained_keys)
            .copied()
            .collect::<HashSet<Key>>();
        changes.removes.extend(keys_to_remove);
        keys.keys.extend(added_keys);
    }
}
pub(crate) fn text_entity_changes(
    added: Query<(Entity, &Text, &Scale), (Added<Text>)>,
    removed: RemovedComponents<Text>,
    mut changes: ResMut<Changes>,
    font: Res<Font>,
) {
    for (entity, text, scale) in added.iter() {
        changes.added_text_entities.insert(
            entity,
            (
                text.string.len(),
                Alignment::new(font.character_dimensions('a', scale.px())),
            ),
        );
    }
    for entity in removed.iter() {
        changes.removed_text_entities.insert(entity);
    }
}
pub(crate) fn add_remove_rasterizations(
    mut changes: ResMut<Changes>,
    mut renderer: ResMut<Renderer>,
    canvas: Res<Canvas>,
) {
    for (entity, (length, alignment)) in changes.added_text_entities.drain() {
        let rasterization = Rasterization::new(
            &canvas,
            &renderer.rasterization_bind_group_layout,
            alignment,
            length,
        );
        renderer.rasterizations.insert(entity, rasterization);
    }
    for entity in changes.removed_text_entities.drain() {
        renderer.rasterizations.remove(&entity);
    }
}
impl Attach for Renderer {
    fn attach(engen: &mut Engen) {
        engen
            .compute
            .startup
            .schedule
            .add_system_to_stage(Stage::Before, compute_setup);
        engen
            .compute
            .main
            .schedule
            .add_system_to_stage(Stage::After, text_entity_changes);
        engen
            .compute
            .main
            .schedule
            .add_system_to_stage(Stage::Last, emit);
        engen
            .render
            .startup
            .schedule
            .add_system_to_stage(Stage::Before, render_setup);
        engen
            .render
            .main
            .schedule
            .add_system_to_stage(Stage::Before, add_remove_rasterizations);
    }
}
#[derive(Resource)]
pub(crate) struct Extraction {
    pub(crate) changes: Changes,
}
impl Extraction {
    pub(crate) fn new() -> Self {
        Self {
            changes: Changes::new(),
        }
    }
}
impl Render for Renderer {
    fn extract(compute: &mut Task, render: &mut Task)
    where
        Self: Sized,
    {
        let mut changes = compute
            .container
            .get_resource_mut::<Changes>()
            .expect("no text changes attached");
        render
            .container
            .get_resource_mut::<Extraction>()
            .expect("no extraction attached")
            .changes
            .adds = changes.adds.drain().collect();
        render
            .container
            .get_resource_mut::<Extraction>()
            .expect("no extraction attached")
            .changes
            .updates = changes.updates.drain().collect();
        render
            .container
            .get_resource_mut::<Extraction>()
            .expect("no extraction attached")
            .changes
            .removes = changes.removes.drain().collect();
        render
            .container
            .get_resource_mut::<Extraction>()
            .expect("no extraction attached")
            .changes
            .glyphs = changes.glyphs.drain().collect();
        render
            .container
            .get_resource_mut::<Extraction>()
            .expect("no extraction attached")
            .changes
            .bounds = changes.bounds.drain().collect();
        render
            .container
            .get_resource_mut::<Extraction>()
            .expect("no extraction attached")
            .changes
            .added_text_entities = changes.added_text_entities.drain().collect();
        render
            .container
            .get_resource_mut::<Extraction>()
            .expect("no extraction attached")
            .changes
            .removed_text_entities = changes.removed_text_entities.drain().collect();
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
        for (_entity, rasterization) in self.rasterizations.iter() {
            if rasterization.instances.count() > 0 {
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
