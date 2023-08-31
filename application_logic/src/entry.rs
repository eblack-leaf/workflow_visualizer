use workflow_visualizer::bevy_ecs::prelude::{
    Changed, Commands, Component, Entity, Event, EventReader, NonSend, Query, Res, ResMut,
};
use workflow_visualizer::{
    bevy_ecs, Button, ButtonBorder, ButtonDespawn, ButtonType, Color, Line, Panel, PanelType,
    Sender, Text, TextValue, TextWrapStyle, Triggered,
};

use crate::enable::EntryEnabled;
use crate::entry_list::{EntryIndex, EntryScale, RemovedEntryIndices, TotalEntries};
use crate::positioning::EntryListPosition;
use crate::workflow::{Action, TokenName, TokenOtp};
use crate::Engen;

pub(crate) fn request_tokens(sender: NonSend<Sender<Engen>>) {
    sender.send(Action::RequestTokenNames);
}
#[derive(Component)]
pub(crate) struct Entry {
    pub(crate) name: Entity,
    pub(crate) otp: Entity,
    pub(crate) line: Entity,
    pub(crate) info_panel: Entity,
    pub(crate) edit_panel: Entity,
    pub(crate) delete_panel: Entity,
    pub(crate) generate_button: Entity,
    pub(crate) edit_button: Entity,
    pub(crate) delete_button: Entity,
}

#[derive(Component)]
pub(crate) struct EntryName(pub(crate) TokenName);

pub(crate) fn display_name(
    entries: Query<(&Entry, &EntryName), Changed<EntryName>>,
    mut text: Query<&mut TextValue>,
) {
    for (entry, entry_name) in entries.iter() {
        if let Ok(mut text) = text.get_mut(entry.name) {
            text.0 = entry_name.0 .0.clone();
        }
    }
}

#[derive(Component)]
pub(crate) struct EntryOtp(pub(crate) Option<TokenOtp>);
#[derive(Event)]
pub(crate) struct ReadOtp(pub(crate) TokenName, pub(crate) TokenOtp);
pub(crate) fn read_otp(
    mut entries: Query<(&EntryName, &mut EntryOtp)>,
    mut events: EventReader<ReadOtp>,
) {
    for event in events.iter() {
        for (name, mut otp) in entries.iter_mut() {
            if event.0 == name.0 {
                otp.0.replace(event.1.clone());
            }
        }
    }
}
#[derive(Event)]
pub(crate) struct EntryAddToken(pub(crate) TokenName);

pub(crate) fn receive_add_token(
    mut events: EventReader<EntryAddToken>,
    mut cmd: Commands,
    entry_scale: Res<EntryScale>,
    mut total_entries: ResMut<TotalEntries>,
) {
    for event in events.iter() {
        let entry = create_entry(&mut cmd, &entry_scale);
        cmd.spawn((
            entry,
            EntryName(event.0.clone()),
            EntryOtp(None),
            EntryEnabled(false),
            EntryIndex(total_entries.0),
            EntryListPosition(None),
        ));
        total_entries.0 += 1;
    }
}
#[derive(Event)]
pub(crate) struct EntryRemoveToken(pub(crate) TokenName);

pub(crate) fn receive_remove_token(
    mut events: EventReader<EntryRemoveToken>,
    mut removed_indices: ResMut<RemovedEntryIndices>,
    entries: Query<(Entity, &Entry, &EntryName, &EntryIndex)>,
    mut cmd: Commands,
) {
    for event in events.iter() {
        for (entity, entry, entry_name, entry_index) in entries.iter() {
            if entry_name.0 == event.0 {
                delete_entry(&mut cmd, entity, &entry);
                removed_indices.0.push(entry_index.0);
            }
        }
    }
}
pub(crate) fn display_otp(
    entries: Query<(&Entry, &EntryOtp), Changed<EntryOtp>>,
    mut text: Query<&mut TextValue>,
) {
    for (entry, entry_otp) in entries.iter() {
        if let Some(otp_val) = entry_otp.0.as_ref() {
            if let Ok(mut text) = text.get_mut(entry.otp) {
                text.0 = otp_val.0.clone();
            }
        }
    }
}

pub(crate) fn delete_entry(cmd: &mut Commands, entity: Entity, entry: &Entry) {
    cmd.entity(entry.name).despawn();
    cmd.entity(entry.otp).despawn();
    cmd.entity(entry.line).despawn();
    cmd.entity(entry.info_panel).despawn();
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

pub(crate) fn create_entry(cmd: &mut Commands, entry_scale: &EntryScale) -> Entry {
    let info_panel = cmd
        .spawn(Panel::new(
            PanelType::Flat,
            5,
            Color::from(Color::MEDIUM_GREEN).with_alpha(1f32),
            Color::from(Color::MEDIUM_GREEN).with_alpha(1f32),
        ))
        .id();
    let edit_panel = cmd
        .spawn(Panel::new(
            PanelType::Flat,
            6,
            Color::from(Color::MEDIUM_RED_ORANGE).with_alpha(1f32),
            Color::from(Color::MEDIUM_RED_ORANGE).with_alpha(1f32),
        ))
        .id();
    let delete_panel = cmd
        .spawn(Panel::new(
            PanelType::Flat,
            7,
            Color::from(Color::MEDIUM_RED).with_alpha(1f32),
            Color::from(Color::MEDIUM_RED).with_alpha(1f32),
        ))
        .id();
    let name = cmd
        .spawn(Text::new(
            4,
            "",
            entry_scale.text_scale,
            Color::from(Color::OFF_WHITE).with_alpha(1f32),
            TextWrapStyle::letter(),
        ))
        .id();
    let otp = cmd
        .spawn(Text::new(
            4,
            "------",
            entry_scale.text_scale,
            Color::from(Color::OFF_WHITE).with_alpha(1f32),
            TextWrapStyle::letter(),
        ))
        .id();
    let line = cmd
        .spawn(Line::new(4, Color::from(Color::OFF_WHITE).with_alpha(1f32)))
        .id();
    let generate_button = cmd
        .spawn(Button::new(
            ButtonType::Press,
            4,
            Color::from(Color::LIGHT_GREEN).with_alpha(1f32),
            Color::from(Color::DARK_GREEN).with_alpha(1f32),
            "generate",
            "",
            15,
            entry_scale.button_icon_scale,
            ButtonBorder::None,
        ))
        .id();
    let edit_button = cmd
        .spawn(Button::new(
            ButtonType::Press,
            4,
            Color::from(Color::LIGHT_RED_ORANGE).with_alpha(1f32),
            Color::from(Color::DARK_RED_ORANGE).with_alpha(1f32),
            "edit",
            "",
            15,
            entry_scale.button_icon_scale,
            ButtonBorder::None,
        ))
        .id();
    let delete_button = cmd
        .spawn(Button::new(
            ButtonType::Press,
            4,
            Color::from(Color::LIGHT_RED).with_alpha(1f32),
            Color::from(Color::DARK_RED).with_alpha(1f32),
            "delete",
            "",
            15,
            entry_scale.button_icon_scale,
            ButtonBorder::None,
        ))
        .id();

    Entry {
        name,
        otp,
        line,
        info_panel,
        edit_panel,
        delete_panel,
        generate_button,
        edit_button,
        delete_button,
    }
}

pub(crate) fn process_entry_buttons(
    triggers: Query<&Triggered>,
    entries: Query<(&Entry, &EntryName)>,
    sender: NonSend<Sender<Engen>>,
) {
    for (entry, entry_name) in entries.iter() {
        if let Ok(trigger) = triggers.get(entry.generate_button) {
            if trigger.active() {
                sender.send(Action::GenerateOtp(entry_name.0.clone()));
            }
        }
        if let Ok(trigger) = triggers.get(entry.edit_button) {
            if trigger.active() {
                // trigger edit animation
            }
        }
        if let Ok(trigger) = triggers.get(entry.delete_button) {
            if trigger.active() {
                sender.send(Action::RemoveToken(entry_name.0.clone()));
            }
        }
    }
}
