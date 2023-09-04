use crate::entry::Entry;
use crate::entry_list::EntryIndex;
use crate::paging::PageRange;
use crate::positioning::EntryListPosition;
use workflow_visualizer::bevy_ecs;
use workflow_visualizer::bevy_ecs::component::Component;
use workflow_visualizer::bevy_ecs::prelude::{Changed, Commands, Query};
use workflow_visualizer::Disabled;

// Trigger to disable all entry elements
#[derive(Component)]
pub(crate) struct EntryEnabled(pub(crate) bool);

pub(crate) fn enable(
    entries: Query<(&Entry, &EntryEnabled), Changed<EntryEnabled>>,
    mut cmd: Commands,
) {
    for (entry, enabled) in entries.iter() {
        if !enabled.0 {
            cmd.entity(entry.name).insert(Disabled::default());
            cmd.entity(entry.otp).insert(Disabled::default());
            cmd.entity(entry.line).insert(Disabled::default());
            cmd.entity(entry.info_panel).insert(Disabled::default());
            cmd.entity(entry.edit_panel).insert(Disabled::default());
            cmd.entity(entry.delete_panel).insert(Disabled::default());
            cmd.entity(entry.generate_button)
                .insert(Disabled::default());
            cmd.entity(entry.edit_button).insert(Disabled::default());
            cmd.entity(entry.delete_button).insert(Disabled::default());
        } else {
            cmd.entity(entry.name).remove::<Disabled>();
            cmd.entity(entry.otp).remove::<Disabled>();
            cmd.entity(entry.line).remove::<Disabled>();
            cmd.entity(entry.info_panel).remove::<Disabled>();
            cmd.entity(entry.edit_panel).remove::<Disabled>();
            cmd.entity(entry.delete_panel).remove::<Disabled>();
            cmd.entity(entry.generate_button).remove::<Disabled>();
            cmd.entity(entry.edit_button).remove::<Disabled>();
            cmd.entity(entry.delete_button).remove::<Disabled>();
        }
    }
}

pub(crate) fn enable_entry(
    page_range: &PageRange,
    enabled: &mut EntryEnabled,
    index: &EntryIndex,
    list_position: &mut EntryListPosition,
) {
    if page_range.contains(*index) {
        enabled.0 = true;
        list_position.0.replace(page_range.normalized(*index));
    } else {
        enabled.0 = false;
        let _ = list_position.0.take();
    }
}
