use bevy_ecs::prelude::{Component, DetectChanges, Entity, IntoSystemConfig, Query, Res, Resource};

use crate::touch::read_touch_events;
use crate::{Attach, SyncPoint, Visualizer};

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
pub(crate) fn set_focused(
    mut focus_listeners: Query<(Entity, &mut Focus)>,
    focused_entity_res: Res<FocusedEntity>,
) {
    if focused_entity_res.is_changed() {
        if let Some(f_entity) = focused_entity_res.entity {
            for (entity, mut listener) in focus_listeners.iter_mut() {
                if f_entity == entity {
                    listener.focus();
                } else if listener.focused() {
                    listener.blur();
                }
            }
        } else {
            for (_entity, mut listener) in focus_listeners.iter_mut() {
                if listener.focused() {
                    listener.blur();
                }
            }
        }
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
pub struct FocusAttachment;

impl Attach for FocusAttachment {
    fn attach(engen: &mut Visualizer) {
        engen
            .job
            .container
            .insert_resource(FocusedEntity::new(None));
        engen
            .job
            .task(Visualizer::TASK_MAIN)
            .add_systems((set_focused
                .in_set(SyncPoint::Config)
                .after(read_touch_events),));
    }
}
