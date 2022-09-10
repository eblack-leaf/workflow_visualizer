use bevy_ecs::prelude::{Schedule, World};
pub type Workload = Schedule;
pub type Container = World;
pub enum ExecutionState {
    Active,
    Suspended,
}
pub struct Job {
    pub execution_state: ExecutionState,
    pub container: Container,
    pub startup: Workload,
    pub job: Workload,
    pub teardown: Workload,
}
impl Job {
    pub fn new() -> Self {
        Self {
            execution_state: ExecutionState::Active,
            container: Container::new(),
            startup: Workload::default(),
            job: Workload::default(),
            teardown: Workload::default(),
        }
    }
}
