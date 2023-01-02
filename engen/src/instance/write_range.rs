use crate::instance::attribute::attribute_size;
use crate::instance::indexer::IndexedAttribute;

pub struct WriteRange<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
> {
    pub(crate) writes: Vec<IndexedAttribute<Attribute>>,
}

impl<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    > WriteRange<Attribute>
{
    pub(crate) fn new(writes: Vec<IndexedAttribute<Attribute>>) -> Self {
        Self { writes }
    }
    pub(crate) fn write(&self, queue: &wgpu::Queue, gpu_buffer: &wgpu::Buffer) {
        let offset = attribute_size::<Attribute>(
            self.writes
                .first()
                .expect("no writes in write range")
                .index
                .0,
        );
        let write_data = self
            .writes
            .iter()
            .map(|write| -> Attribute { write.attribute })
            .collect::<Vec<Attribute>>();
        queue.write_buffer(
            gpu_buffer,
            offset as wgpu::BufferAddress,
            bytemuck::cast_slice(&write_data),
        );
    }
}
