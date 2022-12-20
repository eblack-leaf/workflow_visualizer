use crate::canvas::Canvas;
use crate::text::r_rasterization::Rasterization;
use wgpu::Device;

pub(crate) struct Buffer {
    pub(crate) cpu: Vec<u32>,
    pub(crate) gpu: wgpu::Buffer,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) gpu_len: usize,
    pub(crate) growth_factor: usize,
}
impl Buffer {
    pub(crate) fn new(device: &wgpu::Device, num_elements: usize) -> Self {
        let size = bytes(num_elements);
        let mut cpu = Vec::new();
        cpu.resize(num_elements, 0);
        let gpu = Self::buffer(device, size);
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
        Self {
            cpu,
            gpu,
            bind_group,
            bind_group_layout,
            gpu_len: 0,
            growth_factor: 256,
        }
    }

    fn buffer(device: &Device, size: usize) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rasterization buffer"),
            size: size as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        })
    }
}
pub(crate) fn bytes(num: usize) -> usize {
    num * std::mem::size_of::<u32>()
}
pub(crate) fn write(rasterization: &mut Rasterization, canvas: &Canvas) {
    if !rasterization.write.is_empty() {
        rasterization.buffer.cpu.extend(&rasterization.write);
        let required_size = bytes(rasterization.buffer.cpu.len());
        if required_size > rasterization.buffer.gpu.size() as usize {
            let mut projected_size = rasterization.buffer.gpu.size() as usize;
            while projected_size < required_size {
                projected_size += rasterization.buffer.growth_factor;
            }
            rasterization.buffer.gpu = Buffer::buffer(&canvas.device, projected_size);
            canvas.queue.write_buffer(
                &rasterization.buffer.gpu,
                0,
                bytemuck::cast_slice(&rasterization.buffer.cpu),
            );
        } else {
            canvas.queue.write_buffer(
                &rasterization.buffer.gpu,
                rasterization.buffer.gpu_len as wgpu::BufferAddress,
                bytemuck::cast_slice(&rasterization.write),
            );
        }
        rasterization.buffer.gpu_len += bytes(rasterization.write.len());
    }
    rasterization.write.clear();
}
