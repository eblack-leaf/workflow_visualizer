use crate::text_refactor::instances::Index;
use crate::text_step_out::attributes::Coordinator;
use bevy_ecs::prelude::ResMut;
pub struct Swap {
    pub old: Index,
    pub new: Index,
}
// for app.post_processing() to read back to compute
pub struct Swaps {
    pub swaps: Vec<Swap>,
}
pub struct RemovedInstances {
    pub to_remove: Vec<Index>,
}
pub fn remove_instances(
    mut coordinator: ResMut<Coordinator>,
    mut removed_instances: ResMut<RemovedInstances>,
    mut swaps: ResMut<Swaps>,
) {
    // swap each removed cell with one from the end
    // update coordinator.current
    // remove from cpu_attributes
    // add swaps
}
