use crate::Signal;
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
    pub fn emit<T>(&mut self, signal: T) {
        self.container
            .insert_resource::<Signal<T>>(Signal::new(Some(signal)));
    }
    pub fn receive<T>(&mut self) -> Option<T> {
        return self
            .container
            .get_resource_mut::<Signal<T>>()
            .expect("no signal to receive")
            .receive();
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
