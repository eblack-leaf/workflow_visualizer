use crate::gpu_bindings::bindings;
use crate::text::rasterize::rasterization::GpuRasterization;
use bevy_ecs::prelude::{Commands, Res};

pub struct RasterizationBinding {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}
pub fn setup(mut cmd: Commands, device: Res<wgpu::Device>, gpu_rasterizations: Res<GpuRasterization>) {
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
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("rasterizer bind group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: bindings::RASTERIZATION,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &gpu_rasterizations.buffer,
                offset: 0,
                size: None,
            }),
        }],
    });
    cmd.insert_resource(RasterizationBinding {
        bind_group,
        bind_group_layout
    })
}
