use std::collections::HashMap;

use bevy_ecs::prelude::{Entity, IntoSystemConfig, ResMut, Resource, Schedule, SystemSet, World};
use bevy_ecs::schedule::ExecutorKind;
use compact_str::CompactString;

pub type Container = World;
pub type Task = Schedule;
#[derive(Eq, PartialEq, Hash)]
pub struct TaskLabel(pub &'static str);

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
    pub store: HashMap<EntityName, Entity>,
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
    pub tasks: HashMap<TaskLabel, Task>,
}

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum JobSyncPoint {
    Idle,
}
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct EntityName(pub CompactString);
impl EntityName {
    pub fn new(name: String) -> Self {
        EntityName(CompactString::new(name))
    }
}
impl Job {
    pub fn store_entity(&mut self, id: EntityName, entity: Entity) {
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
            tasks: HashMap::new(),
        }
    }
    pub fn task(&mut self, task_label: TaskLabel) -> &mut Task {
        self.tasks.get_mut(&task_label).expect("no task")
    }
    pub(crate) fn exec(&mut self, task_label: TaskLabel) {
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
