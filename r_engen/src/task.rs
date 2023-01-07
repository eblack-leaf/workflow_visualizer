use bevy_ecs::prelude::{ResMut, Resource, Schedule, SystemStage, World};

pub type Container = World;
pub type Workload = Schedule;
#[derive(PartialEq)]
pub enum ExecutionState {
    Active,
    Suspended,
}

#[derive(Copy, Clone, Resource)]
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

#[derive(Copy, Clone, Resource)]
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
pub struct Task {
    pub execution_state: ExecutionState,
    pub container: Container,
    pub startup: Workload,
    pub main: Workload,
    pub teardown: Workload,
}

impl Task {
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
            main: {
                let mut workload = Workload::default();
                workload.add_stage("attempt to idle", SystemStage::single(attempt_to_idle));
                workload
            },
            teardown: Workload::default(),
        }
    }
    pub fn exec(&mut self, task_workload: TaskWorkload) {
        let workload = match task_workload {
            TaskWorkload::Startup => {
                &mut self.startup
            }
            TaskWorkload::Main => {
                &mut self.main
            }
            TaskWorkload::Teardown => {
                &mut self.teardown
            }
        };
        workload.run_once(&mut self.container);
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

pub enum TaskWorkload {
    Startup,
    Main,
    Teardown,
}
