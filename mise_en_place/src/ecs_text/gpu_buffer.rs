use bevy_ecs::prelude::Component;
use std::marker::PhantomData;

use crate::gfx::GfxSurface;
#[derive(Component)]
pub(crate) struct GpuBuffer<T> {
    pub(crate) buffer: wgpu::Buffer,
    _phantom_data: PhantomData<T>,
}

impl<T> GpuBuffer<T> {
    pub(crate) fn new(gfx_surface: &GfxSurface, max: u32, label: &'static str) -> Self {
        Self {
            buffer: gfx_surface.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: (std::mem::size_of::<T>() * max as usize) as wgpu::BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            _phantom_data: PhantomData,
        }
    }
}
