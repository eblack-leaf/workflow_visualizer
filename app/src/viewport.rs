use crate::coord::{Area, Depth};
use crate::gpu_bindings::bindings::VIEWPORT;
use crate::uniform::Uniform;
use bevy_ecs::prelude::{Commands, Res};
use nalgebra::matrix;

pub struct Viewport {
    pub area: Area,
    pub depth: Depth,
    pub orthographic: nalgebra::Matrix4<f32>,
}
impl Viewport {
    pub fn new(area: Area, depth: Depth) -> Self {
        Self {
            area,
            depth,
            orthographic: matrix![2f32/area.width, 0.0, 0.0, -1.0;
                                    0.0, 2f32/-area.height, 0.0, 1.0;
                                    0.0, 0.0, 1.0/depth.layer, 0.0;
                                    0.0, 0.0, 0.0, 1.0],
        }
    }
    pub fn gpu_viewport(&self) -> GpuViewport {
        return self.orthographic.data.0.into();
    }
    pub fn far_layer(&self) -> f32 {
        self.depth.layer
    }
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct GpuViewport {
    pub matrix: [[f32; 4]; 4],
}
impl From<[[f32; 4]; 4]> for GpuViewport {
    fn from(matrix: [[f32; 4]; 4]) -> Self {
        Self { matrix }
    }
}
pub struct ViewportBinding {
    pub uniform: Uniform<GpuViewport>,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}
impl ViewportBinding {
    pub fn new(device: &wgpu::Device, gpu_viewport: GpuViewport, binding: u32) -> Self {
        let uniform = Uniform::new(device, gpu_viewport);
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("view bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding,
                resource: uniform.buffer.as_entire_binding(),
            }],
        });
        Self {
            uniform,
            bind_group_layout,
            bind_group,
        }
    }
}
pub fn setup(
    device: Res<wgpu::Device>,
    surface_configuration: Res<wgpu::SurfaceConfiguration>,
    mut cmd: Commands,
) {
    let viewport = Viewport::new(
        (
            surface_configuration.width as f32,
            surface_configuration.height as f32,
        )
            .into(),
        100f32.into(),
    );
    let viewport_binding = ViewportBinding::new(&device, viewport.gpu_viewport(), VIEWPORT);
    cmd.insert_resource(viewport);
    cmd.insert_resource(viewport_binding);
}
