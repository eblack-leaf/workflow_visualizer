use workflow_visualizer::bevy_ecs::prelude::{Changed, Commands, Component, Entity, Query};
use workflow_visualizer::{Disabled, TextValue};
use crate::workflow::{TokenName, TokenOtp};
#[derive(Component)]
pub(crate) struct Entry {
    pub(crate) name: Entity,
    pub(crate) otp: Entity,
    pub(crate) line: Entity,
    pub(crate) main_panel: Entity,
    pub(crate) edit_panel: Entity,
    pub(crate) delete_panel: Entity,
    pub(crate) generate_button: Entity,
    pub(crate) edit_button: Entity,
    pub(crate) delete_button: Entity,
}
// where this entry is in the list
#[derive(Component)]
pub(crate) struct EntryListPosition(pub(crate) usize);
pub(crate) fn position(entries: Query<(&Entry, &EntryListPosition), Changed<EntryListPosition>>, mut cmd: Commands) {
    for (entry, list_position) in entries.iter() {
        // get placements for list position then add
    }
}
// Trigger to disable all entry elements
#[derive(Component)]
pub(crate) struct EntryEnabled(pub(crate) bool);
pub(crate) fn enable(entries: Query<(&Entry, &EntryEnabled), Changed<EntryEnabled>>, mut cmd: Commands) {
    for (entry, enabled) in entries.iter() {
        if !enabled.0 {
            cmd.entity(entry.name).insert(Disabled{});
            cmd.entity(entry.otp).insert(Disabled{});
            cmd.entity(entry.line).insert(Disabled{});
            cmd.entity(entry.main_panel).insert(Disabled{});
            cmd.entity(entry.edit_panel).insert(Disabled{});
            cmd.entity(entry.delete_panel).insert(Disabled{});
            cmd.entity(entry.generate_button).insert(Disabled{});
            cmd.entity(entry.edit_button).insert(Disabled{});
            cmd.entity(entry.delete_button).insert(Disabled{});
        } else {
            cmd.entity(entry.name).remove::<Disabled>();
            cmd.entity(entry.otp).remove::<Disabled>();
            cmd.entity(entry.line).remove::<Disabled>();
            cmd.entity(entry.main_panel).remove::<Disabled>();
            cmd.entity(entry.edit_panel).remove::<Disabled>();
            cmd.entity(entry.delete_panel).remove::<Disabled>();
            cmd.entity(entry.generate_button).remove::<Disabled>();
            cmd.entity(entry.edit_button).remove::<Disabled>();
            cmd.entity(entry.delete_button).remove::<Disabled>();
        }
    }
}
#[derive(Component)]
pub(crate) struct EntryName(pub(crate) TokenName);
pub(crate) fn display_name(entries: Query<(&Entry, &EntryName), Changed<EntryName>>, mut text: Query<&mut TextValue>) {
    for (entry, entry_name) in entries.iter() {
        if let Ok(text) = text.get_mut(entry.name) {
            text.0 = entry_name.0.0.clone();
        }
    }
}
#[derive(Component)]
pub(crate) struct EntryOtp(pub(crate) TokenOtp);
pub(crate) fn display_otp(entries: Query<(&Entry, &EntryOtp), Changed<EntryOtp>>, mut text: Query<&mut TextValue>) {
    for (entry, entry_otp) in entries.iter() {
        if let Ok(text) = text.get_mut(entry.otp) {
            text.0 = entry_otp.0.0.clone();
        }
    }
}
