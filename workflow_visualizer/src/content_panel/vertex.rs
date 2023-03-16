use std::f32::consts::FRAC_PI_2;
use crate::gfx::GfxSurface;
use crate::RawPosition;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Default)]
pub struct ListenOffset {
    pub listen_x: f32,
    pub listen_y: f32,
}
impl ListenOffset {
    pub const LISTEN_X_ON: f32 = 1.0;
    pub const LISTEN_Y_ON: f32 = 1.0;
    pub const LISTEN_X_OFF: f32 = 0.0;
    pub const LISTEN_Y_OFF: f32 = 0.0;
    pub fn new(listen_x: f32, listen_y: f32) -> Self {
        Self { listen_x, listen_y }
    }
}
#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Default)]
pub struct ContentPanelVertex {
    pub position: RawPosition,
    pub listen_offset: ListenOffset,
}
pub(crate) const CORNER_DEPTH: u32 = 3;
pub(crate) fn generate_mesh(corner_precision: u32, scale_factor: f64) -> Vec<ContentPanelVertex> {
    let interval = 1f32 / corner_precision as f32;
    let mut corners = Vec::new();
    let mut mesh = Vec::new();
    let mut current = 0f32;
    let mut end = FRAC_PI_2;
    let mut current_corner =
    vec![]
}
pub(crate) fn vertex_buffer(
    gfx_surface: &GfxSurface,
    mesh: Vec<ContentPanelVertex>,
) -> wgpu::Buffer {
    gfx_surface
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("content panel vertex buffer"),
            contents: bytemuck::cast_slice(&mesh),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
}
