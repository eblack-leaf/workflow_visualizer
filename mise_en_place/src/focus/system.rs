use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Query, Res};

use crate::focus::component::{Focus, FocusedEntity};

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
