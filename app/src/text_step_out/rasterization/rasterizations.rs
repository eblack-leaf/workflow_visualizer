use std::collections::HashMap;

use bevy_ecs::prelude::{Commands, Component, Entity, Query, Res, ResMut};
use fontdue::Metrics;
use wgpu::util::DeviceExt;
use wgpu::BufferAddress;

use crate::gpu_bindings::bindings;
use crate::text_step_out::attributes::add::{Add, Adds};
use crate::text_step_out::attributes::write::{Write, Writes};
use crate::text_step_out::attributes::Index;
use crate::text_step_out::font::Font;
use crate::text_step_out::rasterization::placement::RasterizationPlacement;
use crate::text_step_out::rasterization::references::RasterizationReferences;
use crate::text_step_out::scale::Scale;

pub type RasterizedGlyphHash = fontdue::layout::GlyphRasterConfig;
pub type RasterizedGlyph = (Metrics, Vec<u8>);

pub struct Rasterization {
    pub glyph: RasterizedGlyph,
    pub placement: RasterizationPlacement,
}

impl Rasterization {
    pub fn new(glyph: RasterizedGlyph, placement: RasterizationPlacement) -> Self {
        Self { glyph, placement }
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
}

pub enum RasterizationRequestCallPoint {
    Add,
    Write,
}
#[derive(Component)]
pub struct RasterizationRequest {
    pub call_point: RasterizationRequestCallPoint,
    pub entity: Entity,
    pub hash: RasterizedGlyphHash,
    pub character: char,
    pub scale: Scale,
    pub index: Index,
}

impl RasterizationRequest {
    pub fn new(
        call_point: RasterizationRequestCallPoint,
        entity: Entity,
        hash: RasterizedGlyphHash,
        character: char,
        scale: Scale,
        index: Index,
    ) -> Self {
        Self {
            call_point,
            entity,
            hash,
            character,
            scale,
            index,
        }
    }
}
#[derive(Component)]
pub struct RasterizationResponse {
    pub entity: Entity,
    pub rasterization_placement: RasterizationPlacement,
}

impl RasterizationResponse {
    pub fn new(entity: Entity, rasterization_placement: RasterizationPlacement) -> Self {
        Self {
            entity,
            rasterization_placement,
        }
    }
}

pub fn rasterize(
    mut rasterizations: ResMut<Rasterizations>,
    mut references: ResMut<RasterizationReferences>,
    font: Res<Font>,
    requests: Query<(Entity, &RasterizationRequest)>,
    mut adds: ResMut<Adds<RasterizationPlacement>>,
    mut writes: ResMut<Writes<RasterizationPlacement>>,
    mut cmd: Commands,
) {
    requests
        .iter()
        .for_each(|(entity, request): (Entity, &RasterizationRequest)| {
            references.add_ref(request.hash);
            let rasterization_placement = match rasterizations.rasterized_glyphs.get(&request.hash)
            {
                None => {
                    let rasterized_glyph =
                        font.font().rasterize(request.character, request.scale.px());
                    let start: u32 = (rasterizations.cpu.len() - 1) as u32;
                    let row_size: u32 = rasterized_glyph.0.width as u32;
                    let rows: u32 = (rasterized_glyph.1.len() / row_size as usize) as u32;
                    let rasterization_placement =
                        RasterizationPlacement::new(start, row_size, rows);
                    let rasterization =
                        Rasterization::new(rasterized_glyph, rasterization_placement);
                    rasterizations.cpu.extend(&rasterization.glyph.1);
                    rasterizations
                        .rasterized_glyphs
                        .insert(request.hash, rasterization);
                    rasterization_placement
                }
                Some(rasterization) => rasterization.placement,
            };
            cmd.entity(entity).insert(RasterizationResponse::new(
                request.entity,
                rasterization_placement,
            ));
            match request.call_point {
                RasterizationRequestCallPoint::Add => {
                    adds.adds
                        .push(Add::new(request.index, rasterization_placement));
                }
                RasterizationRequestCallPoint::Write => {
                    writes
                        .writes
                        .push(Write::new(request.index, rasterization_placement));
                }
            }
        });
}
