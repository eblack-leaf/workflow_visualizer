use bevy_ecs::prelude::{
    Component, Entity, IntoSystemDescriptor, Query, Res, Resource, SystemLabel,
};

use crate::clickable::ClickSystems;
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

pub(crate) fn set_focused(
    mut focus_listeners: Query<(Entity, &mut Focus)>,
    focused_entity_res: Res<FocusedEntity>,
) {
    if focused_entity_res.is_changed() {
        if let Some(f_entity) = focused_entity_res.entity {
            for (entity, mut listener) in focus_listeners.iter_mut() {
                if f_entity == entity {
                    listener.focus();
                } else {
                    if listener.focused() {
                        listener.blur();
                    }
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

pub struct FocusAttachment;

#[derive(SystemLabel)]
pub enum FocusSystems {
    SetFocused,
}

impl Attach for FocusAttachment {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .container
            .insert_resource(FocusedEntity::new(None));
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Prepare,
            set_focused
                .label(FocusSystems::SetFocused)
                .after(ClickSystems::RegisterClick),
        );
    }
}
