use std::collections::HashMap;

use bevy_ecs::prelude::{Entity, IntoSystemConfig, ResMut, Resource, Schedule, SystemSet, World};
use bevy_ecs::schedule::ExecutorKind;

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

#[derive(Resource)]
pub struct EntityStore {
    pub store: HashMap<&'static str, Entity>,
}

impl EntityStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

pub struct Job {
    pub execution_state: ExecutionState,
    pub container: Container,
    pub startup: Task,
    pub main: Task,
    pub teardown: Task,
}

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum JobBucket {
    Idle,
}

impl Job {
    pub fn store_entity(&mut self, id: &'static str, entity: Entity) {
        self.container
            .get_resource_mut::<EntityStore>()
            .expect("no entity store")
            .store
            .insert(id, entity);
    }
    pub(crate) fn new() -> Self {
        Self {
            execution_state: ExecutionState::Active,
            container: {
                let mut container = Container::new();
                container.insert_resource(Exit::new());
                container.insert_resource(Idle::new());
                container.insert_resource(EntityStore::new());
                container
            },
            startup: Task::default(),
            main: {
                let mut task = Task::default();
                task.add_system(attempt_to_idle.in_set(JobBucket::Idle));
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
        task.set_executor_kind(ExecutorKind::SingleThreaded)
            .run(&mut self.container);
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
