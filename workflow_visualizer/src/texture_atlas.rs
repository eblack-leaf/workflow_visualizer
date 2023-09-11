use std::collections::HashSet;

use bytemuck::{Pod, Zeroable};
use wgpu::{BindGroupEntry, BindGroupLayoutEntry, Texture, TextureView};

use crate::{Area, GfxOptions, GfxSurface, NumericalContext, Position, Section};
pub struct TextureSampler {
    pub sampler: wgpu::Sampler,
}
impl TextureSampler {
    pub fn layout_entry(binding: u32) -> BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        }
    }
    pub fn bind_group_entry(sampler: &wgpu::Sampler, binding: u32) -> BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Sampler(sampler),
        }
    }
    pub fn new(gfx: &GfxSurface) -> Self {
        let sampler_descriptor = wgpu::SamplerDescriptor {
            label: Some("images sampler"),
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
        Self { sampler }
    }
}
pub struct TextureAtlas {
    pub texture: AtlasTexture,
    pub block: AtlasBlock,
    pub dimension: AtlasDimension,
    pub texture_dimensions: AtlasTextureDimensions,
    pub free_locations: AtlasFreeLocations,
}

impl TextureAtlas {
    pub fn new(
        gfx: &GfxSurface,
        block: AtlasBlock,
        dimension: AtlasDimension,
        texture_format: wgpu::TextureFormat,
    ) -> Self {
        let texture_dimensions = AtlasTextureDimensions::new(block, dimension);
        let atlas = AtlasTexture::new(gfx, texture_dimensions, texture_format);
        Self {
            texture: atlas,
            block,
            dimension,
            texture_dimensions,
            free_locations: AtlasFreeLocations::new(dimension),
        }
    }
    pub(crate) fn texture(&self) -> &Texture {
        &self.texture.resource
    }
    pub(crate) fn view(&self) -> &TextureView {
        &self.texture.view
    }
    pub fn write<TexelData: Sized>(
        &self,
        location: AtlasLocation,
        data: &[u8],
        extent_dim: Area<NumericalContext>,
        gfx: &GfxSurface,
    ) -> TextureCoordinates {
        let position = AtlasPosition::new(location, self.block).position;
        let image_copy_texture = wgpu::ImageCopyTexture {
            texture: self.texture(),
            mip_level: 0,
            origin: wgpu::Origin3d {
                x: position.x as u32,
                y: position.y as u32,
                z: 0,
            },
            aspect: wgpu::TextureAspect::All,
        };
        let extent_w = extent_dim.width as u32;
        let extent_h = extent_dim.height as u32;
        let image_data_layout = wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(extent_w * std::mem::size_of::<TexelData>() as u32),
            rows_per_image: Some(extent_h * std::mem::size_of::<TexelData>() as u32),
        };
        let size = wgpu::Extent3d {
            width: extent_w,
            height: extent_h,
            depth_or_array_layers: 1,
        };
        gfx.queue
            .write_texture(image_copy_texture, data, image_data_layout, size);
        let position = AtlasPosition::new(location, self.block).position;
        TextureCoordinates::from_section(
            Section::new(position, extent_dim),
            self.texture_dimensions,
        )
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Default, Debug)]
pub struct TextureCoordinates {
    pub data: [f32; 4],
}

impl TextureCoordinates {
    pub fn from_section(
        glyph_section: Section<NumericalContext>,
        texture_dimensions: AtlasTextureDimensions,
    ) -> Self {
        let normalized_position = Position::<NumericalContext>::new(
            glyph_section.position.x / texture_dimensions.dimensions.width,
            glyph_section.position.y / texture_dimensions.dimensions.height,
        );
        let normalized_area = Area::<NumericalContext>::new(
            glyph_section.width() / texture_dimensions.dimensions.width,
            glyph_section.height() / texture_dimensions.dimensions.height,
        );
        let normalized_section = Section::new(normalized_position, normalized_area);
        TextureCoordinates::new(
            normalized_section.left(),
            normalized_section.top(),
            normalized_section.right(),
            normalized_section.bottom(),
        )
    }
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            data: [left, top, right, bottom],
        }
    }
}

pub struct TextureBindGroup {
    pub bind_group: wgpu::BindGroup,
}

impl TextureBindGroup {
    pub fn new(
        gfx_surface: &GfxSurface,
        layout: &wgpu::BindGroupLayout,
        texture: &wgpu::TextureView,
    ) -> Self {
        Self {
            bind_group: gfx_surface
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("atlas bind group"),
                    layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(texture),
                    }],
                }),
        }
    }
    pub fn entry(binding: u32) -> BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        }
    }
}

pub struct AtlasTexture {
    pub resource: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl AtlasTexture {
    pub fn new(
        gfx_surface: &GfxSurface,
        texture_dimensions: AtlasTextureDimensions,
        texture_format: wgpu::TextureFormat,
    ) -> Self {
        let formats = [texture_format];
        let descriptor = Self::texture_descriptor(texture_dimensions, &formats);
        let texture = gfx_surface.device.create_texture(&descriptor);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self {
            resource: texture,
            view,
        }
    }
    fn hardware_max_check(texture_dimensions: AtlasTextureDimensions) {
        let hardware_max = GfxOptions::limited_environment()
            .limits
            .max_texture_dimension_2d;
        if texture_dimensions.dimensions.width as u32 > hardware_max {
            panic!("requested larger than possible texture")
        }
        if texture_dimensions.dimensions.height as u32 > hardware_max {
            panic!("requested larger than possible texture")
        }
    }
    pub fn texture_descriptor(
        texture_dimensions: AtlasTextureDimensions,
        texture_formats: &[wgpu::TextureFormat],
    ) -> wgpu::TextureDescriptor {
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
            format: texture_formats[0],
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: texture_formats,
        }
    }
}

#[derive(Copy, Clone)]
pub struct AtlasBlock {
    pub block: Area<NumericalContext>,
}

impl AtlasBlock {
    pub fn new<T: Into<Area<NumericalContext>>>(block: T) -> Self {
        Self {
            block: block.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct AtlasTextureDimensions {
    pub dimensions: Area<NumericalContext>,
}

impl AtlasTextureDimensions {
    pub fn new(block: AtlasBlock, dimension: AtlasDimension) -> Self {
        Self {
            dimensions: (
                block.block.width * dimension.dimension as f32,
                block.block.height * dimension.dimension as f32,
            )
                .into(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct AtlasDimension {
    pub dimension: u32,
}

impl AtlasDimension {
    pub fn new(dimension: u32) -> Self {
        Self { dimension }
    }
    pub(crate) fn from_unique_glyphs(unique_glyphs: u32) -> Self {
        Self {
            dimension: {
                let mut dimension = (unique_glyphs as f32).sqrt() as u32;
                while dimension.pow(2) < unique_glyphs {
                    dimension += 1;
                }
                dimension.max(1)
            },
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct AtlasLocation {
    pub x: u32,
    pub y: u32,
}

impl AtlasLocation {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

pub struct AtlasFreeLocations {
    pub free: HashSet<AtlasLocation>,
}

impl AtlasFreeLocations {
    pub fn new(dimension: AtlasDimension) -> Self {
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
    pub fn next(&mut self) -> AtlasLocation {
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
pub struct AtlasPosition {
    pub position: Position<NumericalContext>,
}

impl AtlasPosition {
    pub fn new(atlas_location: AtlasLocation, atlas_block: AtlasBlock) -> Self {
        Self {
            position: (
                atlas_location.x as f32 * atlas_block.block.width,
                atlas_location.y as f32 * atlas_block.block.height,
            )
                .into(),
        }
    }
}
