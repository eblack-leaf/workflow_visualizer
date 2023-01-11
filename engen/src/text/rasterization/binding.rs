use crate::Canvas;
use crate::text::rasterization;
use crate::text::rasterization::bytes;

pub(crate) struct Binding {
    pub(crate) cpu: Vec<u32>,
    pub(crate) write: Vec<u32>,
    pub(crate) gpu: wgpu::Buffer,
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
            self.gpu = Self::buffer(&canvas.device, projected_size);
            canvas
                .queue
                .write_buffer(&self.gpu, 0, bytemuck::cast_slice(&self.cpu));
        } else {
            let offset = bytes(self.cpu_current) as wgpu::BufferAddress;
            canvas.queue.write_buffer(
                &self.gpu,
                offset,
                bytemuck::cast_slice(&self.write),
            );
            self.cpu.extend(&self.write);
            self.cpu_current += self.write.len();
        }
        self.write.clear();
        self.gpu_current = bytes(self.cpu_current);
    }
    pub(crate) fn queue_bitmap(&mut self, bitmap: Vec<u32>) -> usize {
        self.write.extend(bitmap);
        return self.cpu_current + self.write.len();
    }
    pub(crate) fn new(device: &wgpu::Device, num_elements: usize) -> Self {
        let size = rasterization::bytes(num_elements);
        let mut cpu = Vec::new();
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
            write: Vec::new(),
            gpu,
            bind_group,
            bind_group_layout,
            gpu_current: 0,
            cpu_current: 0,
        }
    }

    fn buffer(device: &wgpu::Device, size: usize) -> wgpu::Buffer {
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
