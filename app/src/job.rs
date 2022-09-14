use crate::Signal;
use bevy_ecs::prelude::{ResMut, Schedule, SystemStage, World};

pub type Workload = Schedule;
pub type Container = World;
#[derive(PartialEq)]
pub enum ExecutionState {
    Active,
    Suspended,
}
#[derive(Copy, Clone)]
pub struct Idle {
    pub can_idle: bool,
}
impl Idle {
    pub fn new() -> Self {
        Self { can_idle: false }
    }
}
pub fn attempt_to_idle(mut idle: ResMut<Idle>) {
    idle.can_idle = true;
}
#[derive(Copy, Clone)]
pub struct Exit {
    pub exit_requested: bool,
}
impl Exit {
    pub fn new() -> Self {
        Self {
            exit_requested: false,
        }
    }
    pub fn request_exit(&mut self) {
        self.exit_requested = true;
    }
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
            container: {
                let mut container = Container::new();
                container.insert_resource(Exit::new());
                container.insert_resource(Idle::new());
                container
            },
            startup: Workload::default(),
            exec: {
                let mut workload = Workload::default();
                workload.add_stage("attempt to idle", SystemStage::single(attempt_to_idle));
                workload
            },
            teardown: Workload::default(),
        }
    }
    pub fn emit<T: Send + Sync + 'static>(&mut self, signal: T) {
        self.container
            .insert_resource::<Signal<T>>(Signal::new(Some(signal)));
    }
    pub fn receive<T: Send + Sync + 'static>(&mut self) -> Option<T> {
        return self
            .container
            .get_resource_mut::<Signal<T>>()
            .expect("no signal to receive")
            .receive();
    }
    pub fn suspend(&mut self) {
        self.execution_state = ExecutionState::Suspended;
    }
    pub fn activate(&mut self) {
        self.execution_state = ExecutionState::Active;
    }
    pub fn suspended(&self) -> bool {
        return self.execution_state == ExecutionState::Suspended;
    }
    pub fn active(&self) -> bool {
        return self.execution_state == ExecutionState::Active;
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
    pub fn should_exit(&self) -> bool {
        return self
            .container
            .get_resource::<Exit>()
            .expect("no exit found")
            .exit_requested;
    }
    pub fn can_idle(&self) -> bool {
        return self
            .container
            .get_resource::<Idle>()
            .expect("no idle found")
            .can_idle;
    }
}
