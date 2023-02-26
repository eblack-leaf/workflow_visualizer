use bevy_ecs::prelude::{Component, Entity, Query, ResMut, Resource};

use crate::signal::Signal;
use crate::{Attach, Engen, FrontEndStages};

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

// post process after setters have been run in clickables
pub(crate) fn set_focused(
    mut focused_entity: ResMut<Signal<FocusedEntity>>,
    mut focus_listeners: Query<(Entity, &mut Focus)>,
) {
    let focused = focused_entity.receive();
    if let Some(f_entity) = focused {
        if let Some(ent) = f_entity.entity {
            for (entity, mut listener) in focus_listeners.iter_mut() {
                if ent == entity {
                    listener.focus();
                } else {
                    if listener.focused() {
                        listener.blur();
                    }
                }
            }
        } else {
            for (_, mut listener) in focus_listeners.iter_mut() {
                if listener.focused() {
                    listener.blur();
                }
            }
        }
    }
}

pub struct FocusPlugin;

impl Attach for FocusPlugin {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .container
            .insert_resource(Signal::<FocusedEntity>::new());
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PreProcessResolve, set_focused);
    }
}
