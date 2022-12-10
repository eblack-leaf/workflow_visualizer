use crate::coord::{Area, Depth};
use crate::uniform::Uniform;
use nalgebra::matrix;

pub struct Viewport {
    pub cpu: CpuViewport,
    pub gpu: GpuViewport,
    pub binding: u32,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub uniform: Uniform<GpuViewport>,
    pub depth_texture: wgpu::Texture,
}
impl Viewport {
    pub fn new(device: &wgpu::Device, area: Area) -> Self {
        let depth = 100f32.into();
        let cpu_viewport = CpuViewport::new(area, depth);
        let gpu_viewport = cpu_viewport.gpu_viewport();
        let uniform = Uniform::new(device, gpu_viewport);
        let binding = 0;
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
        let depth_texture = depth_texture(device, area);
        Self {
            cpu: cpu_viewport,
            gpu: gpu_viewport,
            binding,
            bind_group,
            bind_group_layout,
            uniform,
            depth_texture,
        }
    }
    pub fn adjust(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32) {
        let area = (width, height).into();
        self.cpu = CpuViewport::new(area, 100f32.into());
        self.gpu = self.cpu.gpu_viewport();
        self.uniform.update(queue, self.gpu);
        self.depth_texture = depth_texture(device, area);
    }
}
fn depth_texture(device: &wgpu::Device, area: Area) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth texture"),
        size: wgpu::Extent3d {
            width: area.width as u32,
            height: area.height as u32,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
    })
}
pub struct CpuViewport {
    pub area: Area,
    pub depth: Depth,
    pub orthographic: nalgebra::Matrix4<f32>,
}
impl CpuViewport {
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
