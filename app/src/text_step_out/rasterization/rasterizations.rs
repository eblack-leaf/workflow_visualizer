use std::collections::HashMap;
use fontdue::Metrics;
use wgpu::util::DeviceExt;
use wgpu::BufferAddress;

use crate::gpu_bindings::bindings;
use crate::text_step_out::font::Font;
use crate::text_step_out::glyph::Glyph;
use crate::text_step_out::rasterization::placement::RasterizationPlacement;

pub type RasterizedGlyphHash = fontdue::layout::GlyphRasterConfig;
pub type RasterizedGlyph = (Metrics, Vec<u8>);
pub struct Rasterization {
    pub glyph: RasterizedGlyph,
    pub placement: RasterizationPlacement,
}
impl Rasterization {
    pub fn new(glyph: RasterizedGlyph, placement: RasterizationPlacement) -> Self {
        Self {
            glyph,
            placement,
        }
    }
}
pub struct Rasterizations {
    pub cpu: Vec<u8>,
    pub gpu: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub max: u32,
    pub rasterized_glyphs: HashMap<RasterizedGlyphHash, Rasterization>,
}

impl Rasterizations {
    pub fn new(device: &wgpu::Device, max: u32) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("rasterizer bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: bindings::RASTERIZATION,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let gpu = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rasterization"),
            size: max as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("rasterizer bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: bindings::RASTERIZATION,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &gpu,
                    offset: 0,
                    size: None,
                }),
            }],
        });
        Self {
            cpu: Vec::new(),
            gpu,
            bind_group,
            bind_group_layout,
            max,
            rasterized_glyphs: HashMap::new(),
        }
    }
    // need to separate cause cant decide placement here
    pub fn rasterize(&mut self, font: Font, glyph: Glyph) -> RasterizationPlacement {
        if let Some(rasterization) = self.rasterized_glyphs.get(&positioned_glyph.key) {
            return rasterization.1;
        }
        let rasterized_glyph = font.font().rasterize(glyph.character, glyph.px);
        let start: u32 = (self.cpu.len() - 1) as u32;
        let row_size: u32 = glyph.width();
        let rows: u32 = (rasterized_glyph.1.len() / row_size as usize) as u32;
        let rasterization_placement = RasterizationPlacement::new(start, row_size, rows);
        let rasterization = Rasterization::new(rasterized_glyph, rasterization_placement);
        self.cpu.extend(&rasterization.glyph.1);
        self.rasterized_glyphs.insert(
            glyph.hash,
            rasterization,
        );
        return rasterization_placement;
    }
}
