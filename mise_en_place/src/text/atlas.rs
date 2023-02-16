use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::Component;

use crate::coord::{Device, Logical};
use crate::gfx::{GfxOptions, GfxSurface};
use crate::text::coords::Coords;
use crate::text::font::MonoSpacedFont;
use crate::text::glyph::{Glyph, GlyphId};
use crate::text::render_group::RenderGroupUniqueGlyphs;
use crate::text::scale::TextScale;
use crate::{Area, Position};

#[derive(Component)]
pub(crate) struct AtlasBindGroup {
    pub(crate) bind_group: wgpu::BindGroup,
}

impl AtlasBindGroup {
    pub(crate) fn new(
        gfx_surface: &GfxSurface,
        layout: &wgpu::BindGroupLayout,
        atlas: &Atlas,
    ) -> Self {
        Self {
            bind_group: gfx_surface
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("atlas bind group"),
                    layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&atlas.view),
                    }],
                }),
        }
    }
}

#[derive(Component)]
pub(crate) struct Atlas {
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
}

impl Atlas {
    pub(crate) fn new(
        gfx_surface: &GfxSurface,
        texture_dimensions: AtlasTextureDimensions,
    ) -> Self {
        let descriptor = Self::texture_descriptor(texture_dimensions);
        let texture = gfx_surface.device.create_texture(&descriptor);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self { texture, view }
    }
    fn hardware_max_check(texture_dimensions: AtlasTextureDimensions) {
        let hardware_max = GfxOptions::web().limits.max_texture_dimension_2d;
        if texture_dimensions.dimensions.width as u32 > hardware_max {
            panic!("requested larger than possible texture")
        }
        if texture_dimensions.dimensions.height as u32 > hardware_max {
            panic!("requested larger than possible texture")
        }
    }
    fn texture_descriptor(
        texture_dimensions: AtlasTextureDimensions,
    ) -> wgpu::TextureDescriptor<'static> {
        Self::hardware_max_check(texture_dimensions);
        wgpu::TextureDescriptor {
            label: Some("texture atlas"),
            size: wgpu::Extent3d {
                width: texture_dimensions.dimensions.width as u32,
                height: texture_dimensions.dimensions.height as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[wgpu::TextureFormat::R8Unorm],
        }
    }
}

#[derive(Component, Copy, Clone)]
pub(crate) struct AtlasBlock {
    pub(crate) block: Area<Logical>,
}

impl AtlasBlock {
    pub(crate) fn new(font: &MonoSpacedFont, scale: &TextScale) -> Self {
        Self {
            block: font.character_dimensions('a', scale.px()),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub(crate) struct AtlasTextureDimensions {
    pub(crate) dimensions: Area<Logical>,
}

impl AtlasTextureDimensions {
    pub(crate) fn new(block: AtlasBlock, dimension: AtlasDimension) -> Self {
        Self {
            dimensions: (
                block.block.width * dimension.dimension as f32,
                block.block.height * dimension.dimension as f32,
            )
                .into(),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub(crate) struct AtlasDimension {
    pub(crate) dimension: u32,
}

impl AtlasDimension {
    pub(crate) fn new(dimension: u32) -> Self {
        Self { dimension }
    }
    pub(crate) fn from_unique_glyphs(unique_glyphs: RenderGroupUniqueGlyphs) -> Self {
        Self {
            dimension: {
                let mut dimension = (unique_glyphs.unique_glyphs as f32).sqrt() as u32;
                while dimension.pow(2) < unique_glyphs.unique_glyphs {
                    dimension += 1;
                }
                dimension.max(1)
            },
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub(crate) struct AtlasLocation {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl AtlasLocation {
    pub(crate) fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
pub(crate) struct AtlasFreeLocations {
    pub(crate) free: HashSet<AtlasLocation>,
}

impl AtlasFreeLocations {
    pub(crate) fn new(dimension: AtlasDimension) -> Self {
        Self {
            free: {
                let mut free = HashSet::new();
                for x in 0..dimension.dimension {
                    for y in 0..dimension.dimension {
                        let location = AtlasLocation::new(x, y);
                        free.insert(location);
                    }
                }
                free
            },
        }
    }
    pub(crate) fn next(&mut self) -> AtlasLocation {
        let location = match self.free.is_empty() {
            true => {
                panic!("no free locations")
            }
            false => *self.free.iter().next().expect("no free locations"),
        };
        self.free.remove(&location);
        location
    }
}

pub(crate) struct AtlasGlyphReference {
    pub(crate) count: u32,
}

impl AtlasGlyphReference {
    pub(crate) fn new() -> Self {
        Self { count: 0 }
    }
    pub(crate) fn increment(&mut self) {
        self.count += 1;
    }
    pub(crate) fn decrement(&mut self) {
        let sub_value = 1 * (self.count == 0) as u32;
        self.count -= sub_value;
    }
}

pub(crate) type Bitmap = Vec<u8>;

#[derive(Component)]
pub(crate) struct AtlasGlyphReferences {
    pub(crate) references: HashMap<GlyphId, AtlasGlyphReference>,
}

impl AtlasGlyphReferences {
    pub(crate) fn new() -> Self {
        Self {
            references: HashMap::new(),
        }
    }
}

#[derive(Component)]
pub(crate) struct AtlasWriteQueue {
    pub(crate) queue: HashMap<AtlasLocation, (Coords, Area<Logical>, Bitmap)>,
}

impl AtlasWriteQueue {
    pub(crate) fn new() -> Self {
        Self {
            queue: HashMap::new(),
        }
    }
}

#[derive(Component)]
pub(crate) struct AtlasAddQueue {
    pub(crate) queue: HashSet<Glyph>,
}

impl AtlasAddQueue {
    pub(crate) fn new() -> Self {
        Self {
            queue: HashSet::new(),
        }
    }
}

#[derive(Component)]
pub(crate) struct AtlasGlyphs {
    pub(crate) glyphs: HashMap<GlyphId, (Coords, Area<Logical>, AtlasLocation, Bitmap)>,
}

impl AtlasGlyphs {
    pub(crate) fn new() -> Self {
        Self {
            glyphs: HashMap::new(),
        }
    }
}

pub(crate) struct AtlasPosition {
    pub(crate) position: Position<Logical>,
}

impl AtlasPosition {
    pub(crate) fn new(atlas_location: AtlasLocation, atlas_block: AtlasBlock) -> Self {
        Self {
            position: (
                atlas_location.x as f32 * atlas_block.block.width,
                atlas_location.y as f32 * atlas_block.block.height,
            )
                .into(),
        }
    }
}
