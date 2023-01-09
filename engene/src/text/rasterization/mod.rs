use crate::text::InstanceCoordinator;
use bevy_ecs::prelude::Resource;
pub(crate) use binding::Binding;

mod binding;
mod descriptor;

use crate::Canvas;
pub(crate) use descriptor::Descriptor;

pub(crate) fn bytes(num: usize) -> usize {
    num * std::mem::size_of::<u32>()
}
pub(crate) struct Handler {
    pub(crate) binding: Binding,
}
impl Handler {
    pub(crate) fn new(device: &wgpu::Device) -> Self {
        Self {
            binding: Binding::new(device, 10),
        }
    }
    pub(crate) fn read_requests(&mut self, coordinator: &InstanceCoordinator) {}
    pub(crate) fn prepare(&mut self, canvas: &Canvas) {}
    pub(crate) fn integrate_requests(&mut self, coordinator: &mut InstanceCoordinator) {}
}
