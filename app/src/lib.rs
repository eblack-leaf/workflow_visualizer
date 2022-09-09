mod canvas;

use bevy_ecs::prelude::{Schedule, World};
use winit::event_loop::EventLoop;

pub type Workload = Schedule;
pub type Container = World;
pub type AppComponents = ();
pub struct WakeMessage {}
pub enum ExecutionState {
    Active,
    Suspended,
    Uninitialized,
}
pub struct App {
    pub execution_state: ExecutionState,
    pub components: AppComponents,
}
impl App {
    pub fn new() -> Self {
        Self {
            execution_state: ExecutionState::Uninitialized,
            components: (),
        }
    }
    pub fn activate_execution_state(&mut self) {
        self.execution_state = ExecutionState::Active
    }
    pub async fn run<T>(&mut self, event_loop: EventLoop<T>) {

    }
}
