use crate::text::attribute::Coordinator;

pub(crate) fn write<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default,
>(
    coordinator: &mut Coordinator,
) {
}
