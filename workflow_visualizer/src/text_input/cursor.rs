use crate::TextGridLocation;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;

#[derive(Component)]
pub struct Cursor {
    pub location: TextGridLocation,
    pub cached_location: Option<TextGridLocation>,
}

impl Cursor {
    pub(crate) fn new() -> Self {
        Self {
            location: TextGridLocation::new(0, 0),
            cached_location: None,
        }
    }
}

#[derive(Component)]
pub(crate) struct CursorIcon {
    pub(crate) entity: Entity,
}

impl CursorIcon {
    pub(crate) fn new(entity: Entity) -> Self {
        Self { entity }
    }
}
