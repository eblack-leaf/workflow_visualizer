use std::collections::HashMap;
use std::marker::PhantomData;

use bevy_ecs::prelude::{
    Component, Entity, IntoSystemConfigs, ResMut, Resource, Schedule, SystemSet, World,
};
use bevy_ecs::schedule::ExecutorKind;
use compact_str::CompactString;
#[derive(Component, Copy, Clone)]
pub struct Tag<T> {
    _phantom: PhantomData<T>,
}
impl<T> Tag<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
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
impl Job {
    pub(crate) fn new() -> Self {
        Self {
            execution_state: ExecutionState::Suspended,
            container: {
                let mut container = Container::new();
                container.insert_resource(Exit::new());
                container.insert_resource(Idle::new());
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
