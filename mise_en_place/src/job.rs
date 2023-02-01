use bevy_ecs::prelude::{ResMut, Resource, Schedule, SystemStage, World};

pub type Container = World;
pub type Task = Schedule;

pub(crate) enum TaskLabel {
    Startup,
    Main,
    Teardown,
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
    #[allow(unused)]
    pub fn request_exit(&mut self) {
        self.exit_requested = true;
    }
}

pub struct Job {
    pub execution_state: ExecutionState,
    pub container: Container,
    pub startup: Task,
    pub main: Task,
    pub teardown: Task,
}

impl Job {
    pub(crate) fn new() -> Self {
        Self {
            execution_state: ExecutionState::Active,
            container: {
                let mut container = Container::new();
                container.insert_resource(Exit::new());
                container.insert_resource(Idle::new());
                container
            },
            startup: Task::default(),
            main: {
                let mut task = Task::default();
                task.add_stage("idle", SystemStage::single(attempt_to_idle));
                task
            },
            teardown: Task::default(),
        }
    }
    pub(crate) fn exec(&mut self, task_label: TaskLabel) {
        let task = match task_label {
            TaskLabel::Startup => &mut self.startup,
            TaskLabel::Main => &mut self.main,
            TaskLabel::Teardown => &mut self.teardown,
        };
        task.run_once(&mut self.container);
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
