use std::collections::HashMap;

use bevy_ecs::prelude::{
    Component, DetectChanges, Entity, IntoSystemConfig, Local, Query, Res, Resource,
};
use tracing::trace;

use crate::diagnostics::{Diagnostics, DiagnosticsHandle, Record};
use crate::touch::read_touch_events;
use crate::virtual_keyboard::{VirtualKeyboardAdapter, VirtualKeyboardType};
use crate::visualizer::{Attach, Visualizer};
use crate::SyncPoint;

/// Used to set the Focus of an element
#[derive(Component)]
pub struct Focus {
    pub(crate) focused: bool,
}

impl Focus {
    pub fn new() -> Self {
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

impl Default for Focus {
    fn default() -> Self {
        Focus::new()
    }
}

#[derive(Default)]
pub(crate) struct FocusRecorder {
    times_focused: HashMap<Entity, usize>,
    loops_focused: HashMap<Entity, usize>,
}

impl FocusRecorder {
    fn record_time_focus(&mut self, entity: Entity) {
        if let Some(count) = self.times_focused.get_mut(&entity) {
            *count += 1;
        } else {
            self.times_focused.insert(entity, 1);
        }
    }
    fn record_loop_focused(&mut self, entity: Entity) {
        if let Some(count) = self.loops_focused.get_mut(&entity) {
            *count += 1;
        } else {
            self.loops_focused.insert(entity, 1);
        }
    }
}

impl Record for FocusRecorder {
    fn record(&self, core_record: String) -> String {
        format!(
            "{:?}:@times_focused:{:?}:@loops_focused:{:?}",
            core_record, self.times_focused, self.loops_focused
        )
    }
}

/// used to opt into VirtualKeyboard open functionality
#[derive(Component, Copy, Clone, Default)]
pub struct FocusInputListener {}

pub(crate) fn set_focused(
    mut focus_listeners: Query<(Entity, &mut Focus)>,
    focus_input_listeners: Query<(Entity, &FocusInputListener)>,
    focused_entity_res: Res<FocusedEntity>,
    virtual_keyboard: Res<VirtualKeyboardAdapter>,
    #[cfg(feature = "diagnostics")] mut diagnostics: Local<DiagnosticsHandle<FocusRecorder>>,
) {
    if focused_entity_res.is_changed() {
        if let Some(f_entity) = focused_entity_res.entity {
            #[cfg(feature = "diagnostics")]
            {
                diagnostics.ext.record_time_focus(f_entity);
                diagnostics.ext.record_loop_focused(f_entity);
            }
            if focus_input_listeners.get(f_entity).is_ok() {
                virtual_keyboard.open(VirtualKeyboardType::Keyboard);
            } else {
                virtual_keyboard.close();
            }
            for (entity, mut listener) in focus_listeners.iter_mut() {
                if f_entity == entity {
                    listener.focus();
                } else if listener.focused() {
                    listener.blur();
                }
            }
        } else {
            virtual_keyboard.close();
            for (_entity, mut listener) in focus_listeners.iter_mut() {
                if listener.focused() {
                    listener.blur();
                }
            }
        }
    } else {
        #[cfg(feature = "diagnostics")]
        if let Some(entity) = focused_entity_res.entity {
            diagnostics.ext.record_loop_focused(entity);
        }
    }
    #[cfg(feature = "diagnostics")]
    trace!("{:?}", diagnostics.record());
}
/// which entity is currently focused
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
                .in_set(SyncPoint::Preparation)
                .after(read_touch_events),));
    }
}
