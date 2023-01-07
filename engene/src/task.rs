use bevy_ecs::prelude::{ResMut, Resource, Schedule, StageLabel, SystemStage, World};

pub type Container = World;
pub struct Workload {
    pub schedule: Schedule,
}
impl Workload {
    pub fn new() -> Self {
        Self {
            schedule: {
                let mut schedule = Schedule::default();
                schedule.add_stage(Stage::First, SystemStage::parallel());
                schedule.add_stage(Stage::Before, SystemStage::parallel());
                schedule.add_stage(Stage::During, SystemStage::parallel());
                schedule.add_stage(Stage::After, SystemStage::parallel());
                schedule.add_stage(Stage::Last, SystemStage::parallel());
                schedule
            },
        }
    }
}
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
#[derive(StageLabel)]
pub enum Stage {
    First,
    Before,
    During,
    After,
    Last,
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
            startup: Workload::new(),
            main: {
                let mut workload = Workload::new();
                workload
                    .schedule
                    .add_system_to_stage(Stage::First, attempt_to_idle);
                workload
            },
            teardown: Workload::new(),
        }
    }
    pub fn exec(&mut self, workload_id: WorkloadId) {
        let workload = match workload_id {
            WorkloadId::Startup => &mut self.startup,
            WorkloadId::Main => &mut self.main,
            WorkloadId::Teardown => &mut self.teardown,
        };
        workload.schedule.run_once(&mut self.container);
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

pub enum WorkloadId {
    Startup,
    Main,
    Teardown,
}
