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
    pub exec: Workload,
    pub teardown: Workload,
}
impl Job {
    pub fn new() -> Self {
        Self {
            execution_state: ExecutionState::Active,
            container: Container::new(),
            startup: Workload::default(),
            exec: Workload::default(),
            teardown: Workload::default(),
        }
    }
    pub fn startup(&mut self) {
        self.startup.run_once(&mut self.container);
    }
    pub fn exec(&mut self) {
        self.exec.run_once(&mut self.container);
    }
    pub fn teardown(&mut self) {
        self.teardown.run_once(&mut self.container);
    }
}
