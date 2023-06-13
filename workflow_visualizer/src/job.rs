use std::collections::HashMap;

use bevy_ecs::prelude::{Entity, IntoSystemConfig, ResMut, Resource, Schedule, SystemSet, World};
use bevy_ecs::schedule::ExecutorKind;
use compact_str::CompactString;

/// Wrapper around a bevy_ecs::World
pub type Container = World;
/// Wrapper around a bevy_ecs::Schedule
pub type Task = Schedule;

/// Label for a Task
#[derive(Eq, PartialEq, Hash)]
pub struct TaskLabel(pub &'static str);

/// State of a Job
#[derive(PartialEq)]
pub enum ExecutionState {
    Active,
    Suspended,
}

/// Idle hook
#[derive(Copy, Clone, Resource)]
pub struct Idle {
    pub can_idle: bool,
}

impl Idle {
    pub fn new() -> Self {
        Self { can_idle: false }
    }
}

/// System for attempting to idle at the beginning of each loop
pub fn attempt_to_idle(mut idle: ResMut<Idle>) {
    idle.can_idle = true;
}

/// Exit hook
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

/// Store for entity that maps to EntityName
#[derive(Resource)]
pub struct EntityStore {
    pub store: HashMap<EntityName, Entity>,
}

impl EntityStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

/// Extensible container + task runner
pub struct Job {
    pub execution_state: ExecutionState,
    pub container: Container,
    pub tasks: HashMap<TaskLabel, Task>,
}

/// SyncPoint for Job Idle behaviour
#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum JobSyncPoint {
    Idle,
}

/// Name for identifying an Entity
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct EntityName(pub CompactString);
impl EntityName {
    pub fn new<S: Into<String>>(name: S) -> Self {
        EntityName(CompactString::new(name.into()))
    }
}
impl From<&'static str> for EntityName {
    fn from(value: &'static str) -> Self {
        Self::new(value.to_string())
    }
}
impl From<String> for EntityName {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
impl Job {
    pub fn store_entity<T: Into<EntityName>>(&mut self, id: T, entity: Entity) {
        self.container
            .get_resource_mut::<EntityStore>()
            .expect("no entity store")
            .store
            .insert(id.into(), entity);
    }
    pub fn get_entity<T: Into<EntityName>>(&mut self, id: T) {
        todo!()
    }
    pub(crate) fn new() -> Self {
        Self {
            execution_state: ExecutionState::Suspended,
            container: {
                let mut container = Container::new();
                container.insert_resource(Exit::new());
                container.insert_resource(Idle::new());
                container.insert_resource(EntityStore::new());
                container
            },
            tasks: HashMap::new(),
        }
    }
    pub fn task(&mut self, task_label: TaskLabel) -> &mut Task {
        self.tasks.get_mut(&task_label).expect("no task")
    }
    pub fn exec(&mut self, task_label: TaskLabel) {
        if let Some(task) = self.tasks.get_mut(&task_label) {
            task.set_executor_kind(ExecutorKind::MultiThreaded)
                .run(&mut self.container);
        }
    }
    pub fn suspend(&mut self) {
        self.execution_state = ExecutionState::Suspended;
    }
    pub fn resume(&mut self) {
        self.execution_state = ExecutionState::Active;
    }
    pub fn suspended(&self) -> bool {
        self.execution_state == ExecutionState::Suspended
    }
    pub fn resumed(&self) -> bool {
        self.execution_state == ExecutionState::Active
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
