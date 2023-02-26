use bevy_ecs::prelude::Resource;

#[derive(Resource)]
pub struct Signal<T: Resource> {
    pub emission: Option<T>,
}

impl<T: Resource> Signal<T> {
    pub fn new() -> Self {
        Self {
            emission: None
        }
    }
    pub fn emit(&mut self, emit_value: T) {
        self.emission.replace(emit_value);
    }
    pub fn receive(&mut self) -> Option<T> {
        self.emission.take()
    }
}
