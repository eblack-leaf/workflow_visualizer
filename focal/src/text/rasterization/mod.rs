use crate::text::attribute::Instance;
use crate::text::font::{font, Font};
use crate::text::scale::Scale;
use crate::text::TextRenderer;
pub(crate) use placement::Placement;
use std::collections::{HashMap, HashSet};
use wgpu::BufferAddress;

mod placement;
pub(crate) struct RasterizationRequest {
    pub(crate) instance: Instance,
    pub(crate) character: char,
    pub(crate) scale: Scale,
    pub(crate) hash: GlyphHash,
}
pub(crate) struct RasterizationResponse {
    pub(crate) instance: Instance,
    pub(crate) placement: Placement,
}
impl RasterizationResponse {
    pub(crate) fn new(instance: Instance, placement: Placement) -> Self {
        Self {
            instance,
            placement,
        }
    }
}
pub(crate) struct RasterizationRemoval {
    pub(crate) hash: GlyphHash,
    pub(crate) placement: Placement,
}
pub(crate) struct RasterizationReference(pub(crate) u32);
pub(crate) struct RasterizationSwap {
    pub(crate) new: Placement,
}
pub(crate) struct Glyph {
    pub(crate) placement: Placement,
    pub(crate) bitmap: Vec<u32>,
    pub(crate) references: RasterizationReference,
}
pub(crate) type GlyphHash = fontdue::layout::GlyphRasterConfig;
pub(crate) struct Rasterization {
    pub(crate) cpu: Vec<u32>,
    pub(crate) gpu: wgpu::Buffer,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) glyphs: HashMap<GlyphHash, Glyph>,
    pub(crate) rasterization_requests: Vec<RasterizationRequest>,
    pub(crate) rasterization_responses: Vec<RasterizationResponse>,
    pub(crate) rasterization_removals: Vec<RasterizationRemoval>,
    pub(crate) rasterization_write: Vec<u32>,
    pub(crate) retain_glyphs: HashSet<GlyphHash>,
    pub(crate) rasterization_swaps: HashMap<GlyphHash, RasterizationSwap>,
    pub(crate) font: Font,
}
impl Rasterization {
    pub(crate) fn new(device: &wgpu::Device, num_elements: usize) -> Self {
        let size = num_elements * std::mem::size_of::<u32>();
        let mut cpu = Vec::new();
        cpu.resize(num_elements, 0);
        let gpu = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rasterization buffer"),
            size: size as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("rasterizer bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("rasterizer bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &gpu,
                    offset: 0,
                    size: None,
                }),
            }],
        });
        let glyphs = HashMap::new();
        Self {
            cpu,
            gpu,
            bind_group,
            bind_group_layout,
            glyphs,
            rasterization_requests: Vec::new(),
            rasterization_responses: Vec::new(),
            rasterization_removals: Vec::new(),
            rasterization_write: Vec::new(),
            retain_glyphs: HashSet::new(),
            rasterization_swaps: HashMap::new(),
            font: font(),
        }
    }
    pub(crate) fn current_cpu_offset(&self) -> u32 {
        (self.cpu.len() - 1) as u32
    }
    pub(crate) fn interval_adjusted_size(&self, required_size: usize) -> usize {
        // get to next growth interval to avoid frequent allocations
        // returning input now as placeholder
        required_size
    }
    pub(crate) fn current_gpu_offset(&self) -> usize {
        self.current_cpu_offset() as usize - self.rasterization_write.len()
            * std::mem::size_of::<u32>()
    }
}
pub(crate) fn remove(queue: &wgpu::Queue, rasterization: &mut Rasterization) {
    let mut removals = rasterization
        .rasterization_removals
        .drain(..)
        .collect::<Vec<RasterizationRemoval>>();
    removals.iter().for_each(|remove: &RasterizationRemoval| {
        let glyph = rasterization.glyphs.get_mut(&remove.hash).unwrap();
        if glyph.references.0 == 0 {
            if !rasterization.retain_glyphs.contains(&remove.hash) {
                let start = remove.placement.start() as usize;
                let end = (remove.placement.start() + remove.placement.size()) as usize;
                // TODO need to update swaps of moved placements
                // change start of all antecedent placements by range
                // add new placement to swaps
                rasterization.cpu.drain(start..end);
            }
        } else {
            glyph.references.0 -= 1;
        }
    });
    queue.write_buffer(
        &rasterization.gpu,
        0,
        bytemuck::cast_slice(&rasterization.cpu),
    );
}
pub(crate) fn grow(device: &wgpu::Device, queue: &wgpu::Queue, rasterization: &mut Rasterization) {
    let growth = rasterization.rasterization_write.len() * std::mem::size_of::<u32>();
    let current = rasterization.cpu.len() * std::mem::size_of::<u32>();
    let required_size = current + growth;
    if required_size > rasterization.gpu.size() as usize {
        rasterization.gpu = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rasterization buffer"),
            size: rasterization.interval_adjusted_size(required_size) as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        queue.write_buffer(
            &rasterization.gpu,
            0,
            bytemuck::cast_slice(&rasterization.cpu),
        );
        rasterization.rasterization_write.clear();
    }
}
pub(crate) fn write(queue: &wgpu::Queue, rasterization: &mut Rasterization) {
    if !rasterization.rasterization_write.is_empty() {
        queue.write_buffer(
            &rasterization.gpu,
            rasterization.current_gpu_offset() as BufferAddress,
            bytemuck::cast_slice(&rasterization.rasterization_write),
        );
    }
    rasterization.rasterization_write.clear();
}
pub(crate) fn rasterize(rasterization: &mut Rasterization) {
    let mut requests = rasterization
        .rasterization_requests
        .drain(..)
        .collect::<Vec<RasterizationRequest>>();
    requests
        .drain(..)
        .for_each(|request: RasterizationRequest| {
            if let Some(cached_glyph) = rasterization.glyphs.get(&request.hash) {
                rasterization
                    .rasterization_responses
                    .push(RasterizationResponse::new(
                        request.instance,
                        cached_glyph.placement,
                    ));
            } else {
                let rasterized_glyph = rasterization
                    .font
                    .font()
                    .rasterize(request.character, request.scale.px());
                let start: u32 = rasterization.current_cpu_offset();
                let row_size: u32 = rasterized_glyph.0.width as u32;
                let rows: u32 = (rasterized_glyph.1.len() / row_size as usize) as u32;
                let placement = Placement::new(start, row_size, rows);
                let bitmap = rasterized_glyph
                    .1
                    .iter()
                    .map(|g| *g as u32)
                    .collect::<Vec<u32>>();
                rasterization.cpu.extend(&bitmap);
                rasterization.rasterization_write.extend(&bitmap);
                rasterization
                    .rasterization_responses
                    .push(RasterizationResponse::new(request.instance, placement));
                rasterization.glyphs.insert(
                    request.hash,
                    Glyph {
                        bitmap,
                        placement,
                        references: RasterizationReference(0),
                    },
                );
            }
            rasterization
                .glyphs
                .get_mut(&request.hash)
                .unwrap()
                .references
                .0 += 1;
        });
}
