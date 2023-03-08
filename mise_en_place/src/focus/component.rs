use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Resource;

#[derive(Component)]
pub struct Focus {
    pub(crate) focused: bool,
}

impl Focus {
    pub(crate) fn new() -> Self {
        Self { focused: false }
    }
    pub fn focus(&mut self) {
        self.focused = true;
    }
    pub fn blur(&mut self) {
        self.focused = false;
    }
    pub fn focused(&self) -> bool {
        self.focused
    }
}

#[derive(Resource)]
pub struct FocusedEntity {
    pub entity: Option<Entity>,
}

impl FocusedEntity {
    pub(crate) fn new(entity: Option<Entity>) -> Self {
        Self { entity }
    }
}
