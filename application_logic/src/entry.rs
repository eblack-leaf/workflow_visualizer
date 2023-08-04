use workflow_visualizer::{Attach, bevy_ecs, BundledIcon, Button, ButtonDespawn, ButtonType, Color, IconBitmap, IconBitmapRequest, Line, Panel, PanelType, Text, TextScaleAlignment, TextWrapStyle, Visualizer};
use workflow_visualizer::{Disabled, TextValue};
use workflow_visualizer::bevy_ecs::prelude::{
    Changed, Commands, Component, DetectChanges, Entity, EventReader, Query, Res, ResMut,
    Resource,
};

use crate::workflow::{TokenName, TokenOtp};

pub struct EntryAttachment;

impl Attach for EntryAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.add_event::<ReceivedTokens>();
        visualizer.spawn(IconBitmapRequest::from(("edit", IconBitmap::bundled(BundledIcon::Edit))));
    }
}
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

#[derive(Resource)]
pub(crate) struct EntryScale {
    pub(crate) button_icon_scale: u32,
    pub(crate) text_scale: u32,
}

#[derive(Component, Copy, Clone)]
pub(crate) struct EntryIndex(pub(crate) u32);

pub(crate) fn enable_by_index_change(
    mut entries: Query<
        (&mut EntryEnabled, &EntryIndex, &mut EntryListPosition),
        Changed<EntryIndex>,
    >,
    page_range: Res<PageRange>,
) {
    for (mut enabled, index, mut list_position) in entries.iter_mut() {
        if page_range.contains(*index) {
            enabled.0 = true;
            list_position.0.replace(page_range.normalized(*index));
        }
    }
}

#[derive(Resource)]
pub(crate) struct RemovedEntryIndices(pub(crate) Vec<u32>);

pub(crate) fn removed_indices(
    mut indices: ResMut<RemovedEntryIndices>,
    entries: Query<(&mut EntryIndex)>,
) {
    indices.0.sort();
    indices.0.reverse();
    for removed_index in indices.0.drain(..) {
        for entry_index in entries.iter() {
            if entry_index.0 > removed_index {
                *entry_index.0 -= 1;
            }
        }
    }
}

pub(crate) struct ReceivedTokens(pub(crate) Vec<TokenName>);

pub(crate) fn receive_tokens(
    mut events: EventReader<ReceivedTokens>,
    mut cmd: Commands,
    existing_entries: Query<(Entity, &Entry)>,
    entry_scale: Res<EntryScale>,
) {
    let mut new_tokens = vec![];
    for event in events.iter() {
        new_tokens = event.0.clone();
    }
    if !new_tokens.is_empty() {
        for (entity, entry) in existing_entries.iter() {
            delete_entry(&mut cmd, entity, entry);
        }
        let mut index = 0;
        for token in new_tokens {
            let entry = create_entry(&mut cmd, &entry_scale);
            cmd.spawn((
                entry,
                EntryName(token),
                EntryOtp(None),
                EntryEnabled(false),
                EntryIndex(index),
                EntryListPosition(None),
            ));
            index += 1;
        }
    }
}

fn delete_entry(cmd: &mut Commands, entity: Entity, entry: &Entry) {
    cmd.entity(entry.name).despawn();
    cmd.entity(entry.otp).despawn();
    cmd.entity(entry.line).despawn();
    cmd.entity(entry.main_panel).despawn();
    cmd.entity(entry.edit_panel).despawn();
    cmd.entity(entry.delete_panel).despawn();
    cmd.entity(entry.generate_button)
        .insert(ButtonDespawn::default());
    cmd.entity(entry.edit_button)
        .insert(ButtonDespawn::default());
    cmd.entity(entry.delete_button)
        .insert(ButtonDespawn::default());
    cmd.entity(entity).despawn();
}

fn create_entry(cmd: &mut Commands, entry_scale: &EntryScale) -> Entry {
    let main_panel = cmd
        .spawn(Panel::new(
            PanelType::Panel,
            5,
            Color::from(Color::MEDIUM_GREEN).with_alpha(0f32),
            Color::from(Color::MEDIUM_GREEN).with_alpha(0f32),
        ))
        .id();
    let edit_panel = cmd
        .spawn(Panel::new(
            PanelType::Panel,
            6,
            Color::from(Color::MEDIUM_RED_ORANGE).with_alpha(0f32),
            Color::from(Color::MEDIUM_RED_ORANGE).with_alpha(0f32),
        ))
        .id();
    let delete_panel = cmd
        .spawn(Panel::new(
            PanelType::Panel,
            7,
            Color::from(Color::MEDIUM_RED).with_alpha(0f32),
            Color::from(Color::MEDIUM_RED).with_alpha(0f32),
        ))
        .id();
    let name = cmd
        .spawn(Text::new(
            4,
            "",
            TextScaleAlignment::Custom(entry_scale.text_scale),
            Color::from(Color::OFF_WHITE).with_alpha(0f32),
            TextWrapStyle::letter(),
        ))
        .id();
    let otp = cmd
        .spawn(Text::new(
            4,
            "------",
            TextScaleAlignment::Custom(entry_scale.text_scale),
            Color::from(Color::OFF_WHITE).with_alpha(0f32),
            TextWrapStyle::letter(),
        ))
        .id();
    let line = cmd
        .spawn(Line::new(4, Color::from(Color::OFF_WHITE).with_alpha(0f32)))
        .id();
    let generate_button = cmd
        .spawn(Button::new(
            ButtonType::Press,
            4,
            Color::from(Color::LIGHT_GREEN).with_alpha(0f32),
            Color::from(Color::DARK_GREEN).with_alpha(0f32),
            "edit",
            "",
            15,
            entry_scale.button_icon_scale,
        ))
        .id();
    let edit_button = cmd
        .spawn(Button::new(
            ButtonType::Press,
            4,
            Color::from(Color::LIGHT_RED_ORANGE).with_alpha(0f32),
            Color::from(Color::DARK_RED_ORANGE).with_alpha(0f32),
            "edit",
            "",
            15,
            entry_scale.button_icon_scale,
        ))
        .id();
    let delete_button = cmd
        .spawn(Button::new(
            ButtonType::Press,
            4,
            Color::from(Color::LIGHT_RED).with_alpha(0f32),
            Color::from(Color::DARK_RED).with_alpha(0f32),
            "edit",
            "",
            15,
            entry_scale.button_icon_scale,
        ))
        .id();
    let entry = Entry {
        name,
        otp,
        line,
        main_panel,
        edit_panel,
        delete_panel,
        generate_button,
        edit_button,
        delete_button,
    };
    entry
}

#[derive(Resource, Copy, Clone)]
pub(crate) struct PageMax(pub(crate) u32);

#[derive(Resource)]
pub(crate) struct Page(pub(crate) u32);

#[derive(Resource)]
pub(crate) struct PageLeft(pub(crate) bool);

#[derive(Resource)]
pub(crate) struct PageRight(pub(crate) bool);

pub(crate) fn page_change(
    mut entry_list_page: ResMut<Page>,
    mut page_left: ResMut<PageLeft>,
    mut page_right: ResMut<PageRight>,
    page_max: Res<PageMax>,
) {
    if page_left.0 {
        if entry_list_page.0 > 0 {
            entry_list_page.0 -= 1;
        }
        page_left.0 = false;
    }
    if page_right.0 {
        if entry_list_page.0 < page_max.0 {
            entry_list_page.0 += 1;
        }
        page_right.0 = false;
    }
}

#[derive(Resource)]
pub(crate) struct EntriesPerPage(pub(crate) u32);

#[derive(Resource)]
pub(crate) struct PageRange(pub(crate) u32, pub(crate) u32);

impl PageRange {
    pub(crate) fn contains(&self, index: EntryIndex) -> bool {
        let mut found = false;
        for i in self.0..self.1 {
            if i == index.0 {
                found = true;
            }
        }
        found
    }
    pub(crate) fn normalized(&self, index: EntryIndex) -> u32 {
        let diff = self.1 - self.0;
        index.0 % diff
    }
}

pub(crate) fn page_range(
    entries_per: Res<EntriesPerPage>,
    mut range: ResMut<PageRange>,
    page: Res<Page>,
) {
    if entries_per.is_changed() || page.is_changed() {
        // un-position old range entries
        // set range
        // reposition entries
    }
}
// where this entry is in the list
#[derive(Component)]
pub(crate) struct EntryListPosition(pub(crate) Option<u32>);

pub(crate) fn position(
    entries: Query<(&Entry, &EntryListPosition), Changed<EntryListPosition>>,
    mut cmd: Commands,
) {
    for (entry, list_position) in entries.iter() {
        // get placements for list position then add
    }
}
// Trigger to disable all entry elements
#[derive(Component)]
pub(crate) struct EntryEnabled(pub(crate) bool);

pub(crate) fn enable(
    entries: Query<(&Entry, &EntryEnabled), Changed<EntryEnabled>>,
    mut cmd: Commands,
) {
    for (entry, enabled) in entries.iter() {
        if !enabled.0 {
            cmd.entity(entry.name).insert(Disabled {});
            cmd.entity(entry.otp).insert(Disabled {});
            cmd.entity(entry.line).insert(Disabled {});
            cmd.entity(entry.main_panel).insert(Disabled {});
            cmd.entity(entry.edit_panel).insert(Disabled {});
            cmd.entity(entry.delete_panel).insert(Disabled {});
            cmd.entity(entry.generate_button).insert(Disabled {});
            cmd.entity(entry.edit_button).insert(Disabled {});
            cmd.entity(entry.delete_button).insert(Disabled {});
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

pub(crate) fn display_name(
    entries: Query<(&Entry, &EntryName), Changed<EntryName>>,
    mut text: Query<&mut TextValue>,
) {
    for (entry, entry_name) in entries.iter() {
        if let Ok(text) = text.get_mut(entry.name) {
            text.0 = entry_name.0.0.clone();
        }
    }
}
#[derive(Component)]
pub(crate) struct EntryOtp(pub(crate) Option<TokenOtp>);

pub(crate) fn display_otp(
    entries: Query<(&Entry, &EntryOtp), Changed<EntryOtp>>,
    mut text: Query<&mut TextValue>,
) {
    for (entry, entry_otp) in entries.iter() {
        if let Some(otp_val) = entry_otp.0.as_ref() {
            if let Ok(text) = text.get_mut(entry.otp) {
                text.0 = otp_val.0.clone();
            }
        }
    }
}
