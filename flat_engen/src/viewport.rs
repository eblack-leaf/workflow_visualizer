use bevy_ecs::change_detection::Mut;
use bevy_ecs::prelude::{Commands, Res, Resource};
use nalgebra::matrix;

use crate::canvas::Canvas;
use crate::coord::{Depth, ScaledArea, ScaledPosition, ScaledSection};
use crate::task::Stage;
use crate::uniform::Uniform;
use crate::{Attach, Engen, Section};

#[derive(Resource)]
pub struct Viewport {
    pub(crate) cpu: CpuViewport,
    pub(crate) gpu: GpuViewport,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) uniform: Uniform<GpuViewport>,
    pub(crate) depth_texture: wgpu::Texture,
    pub(crate) depth_format: wgpu::TextureFormat,
    pub(crate) offset: ViewportOffset,
    pub(crate) offset_uniform: Uniform<ViewportOffset>,
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Default, PartialEq)]
pub(crate) struct ViewportOffset {
    pub(crate) offset: [f32; 4],
}

impl ViewportOffset {
    pub(crate) fn new(position: ScaledPosition) -> Self {
        Self {
            offset: [position.x, position.y, 0.0, 0.0],
        }
    }
}

impl Viewport {
    pub(crate) fn new(device: &wgpu::Device, area: ScaledArea) -> Self {
        let depth = 100u32.into();
        let cpu_viewport = CpuViewport::new(area, depth);
        let gpu_viewport = cpu_viewport.gpu_viewport();
        let uniform = Uniform::new(device, gpu_viewport);
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("view bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let offset = ViewportOffset::new(ScaledPosition::new(0.0, 0.0));
        let offset_uniform = Uniform::new(device, offset);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform.buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: offset_uniform.buffer.as_entire_binding(),
                },
            ],
        });
        let depth_format = wgpu::TextureFormat::Depth32Float;
        let depth_texture = depth_texture(device, area, depth_format);
        Self {
            cpu: cpu_viewport,
            gpu: gpu_viewport,
            bind_group,
            bind_group_layout,
            uniform,
            depth_texture,
            depth_format,
            offset,
            offset_uniform,
        }
    }
    pub(crate) fn as_section(&self) -> ScaledSection {
        ScaledSection::new(
            ScaledPosition::new(self.offset.offset[0], self.offset.offset[1]),
            self.cpu.area,
        )
    }
    pub(crate) fn adjust_area(&mut self, canvas: &Canvas, width: u32, height: u32) {
        let area = ScaledArea::new(width as f32, height as f32);
        self.cpu = CpuViewport::new(area, 100u32.into());
        self.gpu = self.cpu.gpu_viewport();
        self.uniform.update(&canvas.queue, self.gpu);
        self.depth_texture = depth_texture(&canvas.device, area, self.depth_format);
    }
    pub(crate) fn update_offset(&mut self, queue: &wgpu::Queue, offset: ScaledPosition) {
        self.offset = ViewportOffset::new(offset);
        self.offset_uniform.update(queue, self.offset);
    }
    pub(crate) fn get_mut(engen: &mut Engen) -> Mut<'_, Viewport> {
        engen
            .backend
            .container
            .get_resource_mut::<Viewport>()
            .expect("no viewport")
    }
    pub(crate) fn get(engen: &Engen) -> &Viewport {
        engen
            .backend
            .container
            .get_resource::<Viewport>()
            .expect("no viewport")
    }
}

fn depth_texture(
    device: &wgpu::Device,
    area: ScaledArea,
    format: wgpu::TextureFormat,
) -> wgpu::Texture {
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
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[format],
    })
}

#[derive(Resource)]
pub(crate) struct CpuViewport {
    pub(crate) area: ScaledArea,
    pub(crate) depth: Depth,
    pub(crate) orthographic: nalgebra::Matrix4<f32>,
}

impl CpuViewport {
    pub(crate) fn new(area: ScaledArea, depth: Depth) -> Self {
        Self {
            area,
            depth,
            orthographic: matrix![2f32/area.width, 0.0, 0.0, -1.0;
                                    0.0, 2f32/-area.height, 0.0, 1.0;
                                    0.0, 0.0, 1.0/depth.layer, 0.0;
                                    0.0, 0.0, 0.0, 1.0],
        }
    }
    pub(crate) fn gpu_viewport(&self) -> GpuViewport {
        return self.orthographic.data.0.into();
    }
    pub(crate) fn far_layer(&self) -> f32 {
        self.depth.layer
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Resource)]
pub struct GpuViewport {
    pub matrix: [[f32; 4]; 4],
}

impl From<[[f32; 4]; 4]> for GpuViewport {
    fn from(matrix: [[f32; 4]; 4]) -> Self {
        Self { matrix }
    }
}

pub(crate) fn attach(canvas: Res<Canvas>, mut cmd: Commands) {
    let surface_configuration = &canvas.surface_configuration;
    let area = ScaledArea::new(
        surface_configuration.width as f32,
        surface_configuration.height as f32,
    );
    cmd.insert_resource(Viewport::new(&canvas.device, area));
}
