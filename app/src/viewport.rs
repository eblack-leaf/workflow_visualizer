use crate::coord::{Area, Panel};
use crate::uniform::Uniform;
use nalgebra::Matrix4;

pub struct Viewport {
    pub projection: nalgebra::Orthographic3<f32>,
    pub translation: nalgebra::Translation2<f32>,
}
impl Viewport {
    pub fn new(panel: Panel) -> Self {
        Self {
            projection: nalgebra::Orthographic3::<f32>::new(
                0.0,
                panel.width(),
                panel.height(),
                0.0,
                0.0,
                panel.layer(),
            ),
            translation: nalgebra::Translation2::default(),
        }
    }
    pub fn width(&self) -> f32 {
        return self.projection.right();
    }
    pub fn height(&self) -> f32 {
        return self.projection.bottom();
    }
    pub fn far_layer(&self) -> f32 {
        return self.projection.zfar();
    }
    pub fn near_layer(&self) -> f32 {
        return self.projection.znear();
    }
    pub fn area(&self) -> Area {
        return Area::new(self.width(), self.height());
    }
    pub fn set_width(&mut self, width: f32) {
        self.projection.set_right(width.into());
    }
    pub fn set_height(&mut self, height: f32) {
        self.projection.set_bottom(height.into());
    }
    pub fn translate(&mut self, translation: nalgebra::Translation2<f32>) {
        self.translation.vector += translation.vector;
    }
    pub fn matrix(&self) -> Matrix4<f32> {
        return self.projection.as_matrix().append_translation(
            &nalgebra::Translation3::new(self.translation.x, self.translation.y, 0.0).vector,
        );
    }
    pub fn gpu_viewport(&self) -> GpuViewport {
        return self.matrix().data.0.into();
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
