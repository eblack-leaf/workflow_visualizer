use bevy_ecs::prelude::{Schedule, World};

pub type Container = World;
pub type Workload = Schedule;
pub struct Task {
    pub container: Container,
    pub setup: Workload,
    pub main: Workload,
}
impl Task {
    pub fn new() -> Self {
        Self {
            container: Container::new(),
            setup: Workload::default(),
            main: Workload::default(),
        }
    }
}
