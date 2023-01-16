use crate::text::rasterization;
use crate::text::rasterization::bytes;
use crate::Canvas;
use wgpu::{
    Extent3d, ImageCopyTexture, ImageDataLayout, Origin3d, TextureAspect, TextureSampleType,
    TextureViewDescriptor, TextureViewDimension,
};

pub(crate) struct Binding {
    pub(crate) cpu: Vec<u8>,
    pub(crate) write: Vec<u8>,
    pub(crate) gpu: wgpu::Texture,
    pub(crate) gpu_view: wgpu::TextureView,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) gpu_current: usize,
    pub(crate) cpu_current: usize,
}

impl Binding {
    pub(crate) fn write_queued(&mut self, canvas: &Canvas) {
        // handle grow here
        let projected_size = bytes(self.write.len()) + self.gpu_current;
        if projected_size > self.gpu.size() as usize {
            self.cpu.extend(&self.write);
            self.cpu_current += self.write.len();
            self.gpu = Self::texture(&canvas.device, projected_size);
            canvas.queue.write_texture();
        } else {
            let offset = bytes(self.cpu_current) as wgpu::BufferAddress;
            let data = bytemuck::cast_slice(&self.write);
            canvas.queue.write_texture(
                ImageCopyTexture {
                    texture: &self.gpu,
                    mip_level: 0,
                    origin: Origin3d { x: 0, y: 0, z: 0 },
                    aspect: TextureAspect::All,
                },
                data,
                ImageDataLayout {
                    offset,
                    bytes_per_row: None,
                    rows_per_image: None,
                },
                Extent3d {
                    width: 0,
                    height: 0,
                    depth_or_array_layers: 0,
                },
            );
            self.cpu.extend(&self.write);
            self.cpu_current += self.write.len();
        }
        self.write.clear();
        self.gpu_current = bytes(self.cpu_current);
    }
    pub(crate) fn queue_bitmap(&mut self, bitmap: Vec<u8>) -> usize {
        self.write.extend(bitmap);
        return self.cpu_current + self.write.len();
    }
    pub(crate) fn new(device: &wgpu::Device, num_elements: usize) -> Self {
        let size = rasterization::bytes(num_elements);
        let mut cpu = Vec::new();
        let gpu = Self::texture(device, size);
        let gpu_view = gpu.create_view(&TextureViewDescriptor::default());
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("rasterizer bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: TextureSampleType::Uint,
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("rasterizer bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&gpu_view),
            }],
        });
        Self {
            cpu,
            write: Vec::new(),
            gpu,
            gpu_view,
            bind_group,
            bind_group_layout,
            gpu_current: 0,
            cpu_current: 0,
        }
    }

    fn texture(device: &wgpu::Device, size: usize) -> wgpu::Texture {
        let dimension = (size / 2).min(2048) as u32;
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("rasterization buffer"),
            size: Extent3d {
                width: dimension,
                height: dimension,
                depth_or_array_layers: 0,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Uint,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
        })
    }
}
