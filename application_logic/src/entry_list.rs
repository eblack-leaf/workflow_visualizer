use workflow_visualizer::{bevy_ecs, Grid, GridPoint, RawMarker, ResponsiveUnit, ScaleFactor};
use workflow_visualizer::bevy_ecs::change_detection::{DetectChanges, Res, ResMut};
use workflow_visualizer::bevy_ecs::component::Component;
use workflow_visualizer::bevy_ecs::entity::Entity;
use workflow_visualizer::bevy_ecs::event::EventReader;
use workflow_visualizer::bevy_ecs::prelude::{Changed, Commands, Query, Resource};

use crate::{enable, entry};
use crate::enable::EntryEnabled;
use crate::entry::{Entry, EntryName, EntryOtp};
use crate::paging::PageRange;
use crate::positioning::EntryListPosition;
use crate::workflow::TokenName;

#[derive(Resource)]
pub(crate) struct RemovedEntryIndices(pub(crate) Vec<u32>);

pub(crate) fn setup_removed_entry_indices(mut cmd: Commands) {
    cmd.insert_resource(RemovedEntryIndices(vec![]));
}

pub(crate) fn removed_indices(
    mut indices: ResMut<RemovedEntryIndices>,
    mut entries: Query<&mut EntryIndex>,
    mut total_entries: ResMut<TotalEntries>,
) {
    indices.0.sort();
    indices.0.reverse();
    for removed_index in indices.0.drain(..) {
        total_entries.0 -= 1;
        for mut entry_index in entries.iter_mut() {
            if entry_index.0 > removed_index {
                entry_index.0 -= 1;
            }
        }
    }
}

pub(crate) struct ReceivedTokens(pub(crate) Vec<TokenName>);

#[derive(Resource, Copy, Clone)]
pub(crate) struct TotalEntries(pub(crate) u32);

pub(crate) fn setup_total_entries(mut cmd: Commands) {
    cmd.insert_resource(TotalEntries(0));
}

pub(crate) fn receive_tokens(
    mut events: EventReader<ReceivedTokens>,
    mut cmd: Commands,
    existing_entries: Query<(Entity, &Entry)>,
    entry_scale: Res<EntryScale>,
    mut total_entries: ResMut<TotalEntries>,
) {
    let mut new_tokens = vec![];
    for event in events.iter() {
        new_tokens = event.0.clone();
    }
    if !new_tokens.is_empty() {
        for (entity, entry) in existing_entries.iter() {
            entry::delete_entry(&mut cmd, entity, entry);
        }
        total_entries.0 = 0;
        let mut index = 0;
        for token in new_tokens {
            let entry = entry::create_entry(&mut cmd, &entry_scale);
            cmd.spawn((
                entry,
                EntryName(token),
                EntryOtp(None),
                EntryEnabled(false),
                EntryIndex(index),
                EntryListPosition(None),
            ));
            total_entries.0 += 1;
            index += 1;
        }
    }
}

#[derive(Resource)]
pub(crate) struct EntryListLayout {
    pub(crate) horizontal_markers: RawMarker,
    pub(crate) vertical_markers: RawMarker,
    pub(crate) anchor: GridPoint,
}

#[derive(Resource)]
pub(crate) struct EntriesPerPage(pub(crate) u32);

#[derive(Resource, Copy, Clone, Default)]
pub(crate) struct ListDimensions {
    pub(crate) entry: RawMarker,
    pub(crate) padding: RawMarker,
    pub(crate) content: RawMarker,
}

pub(crate) fn dimension_change(
    mut dimensions: ResMut<ListDimensions>,
    // mut entry_scale: ResMut<EntryScale>,
    scale_factor: Res<ScaleFactor>,
) {
    if scale_factor.is_changed() {
        // let entry = (10f64 * scale_factor.factor()).floor() as i32;
        let entry = 12;
        dimensions.entry = entry.into();
        // let padding = (2f64 * scale_factor.factor()).floor() as i32;
        let padding = 2;
        dimensions.padding = padding.into();
        // let content = ((entry - 2 * padding) as f64 * scale_factor.factor()).floor() as i32;
        let content = entry - 2 * padding;
        dimensions.content = content.into();
    }
}

pub(crate) fn setup_entry_list(mut cmd: Commands) {
    let entry_list_layout = EntryListLayout {
        horizontal_markers: 0.into(),
        vertical_markers: 0.into(),
        anchor: GridPoint::from((1.near(), 1.near())),
    };
    let entries_per_page = EntriesPerPage(1);
    cmd.insert_resource(entry_list_layout);
    cmd.insert_resource(entries_per_page);
}

pub(crate) fn entry_list_layout(
    grid: Res<Grid>,
    mut entries_per: ResMut<EntriesPerPage>,
    mut entry_list_layout: ResMut<EntryListLayout>,
    list_dimensions: Res<ListDimensions>,
) {
    if grid.is_changed() || list_dimensions.is_changed() {
        let begin = grid.calc_horizontal_location(1.near());
        let end = grid.calc_horizontal_location(4.far());
        let horizontal_markers = end.0 - begin.0;
        let vertical_markers = grid.vertical_markers()
            - 2 * grid.markers_per_gutter()
            - list_dimensions.entry.0
            - 2 * list_dimensions.padding.0;
        entry_list_layout.horizontal_markers = horizontal_markers.max(1).into();
        entry_list_layout.vertical_markers = vertical_markers.max(1).into();
        entries_per.0 =
            (vertical_markers / (list_dimensions.entry.0 + list_dimensions.padding.0)) as u32;
        entries_per.0 = entries_per.0.max(1);
    }
}

#[derive(Resource)]
pub(crate) struct EntryScale {
    pub(crate) button_icon_scale: u32,
    pub(crate) text_scale: u32,
}

pub(crate) fn setup_entry_scale(mut cmd: Commands) {
    let entry_scale = EntryScale {
        button_icon_scale: 16,
        text_scale: 18,
    };
    cmd.insert_resource(entry_scale);
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
        enable::enable_entry(&page_range, &mut enabled, index, &mut list_position);
    }
}
