use crate::entry::Entry;
use crate::entry_list::{EntryListLayout, ListDimensions};
use std::collections::HashMap;
use workflow_visualizer::bevy_ecs;
use workflow_visualizer::bevy_ecs::change_detection::{DetectChanges, Res, ResMut};
use workflow_visualizer::bevy_ecs::component::Component;
use workflow_visualizer::bevy_ecs::prelude::{Changed, Commands, Query, Resource};
use workflow_visualizer::{GridPoint, RawMarker, ResponsiveGridView, ResponsivePathView};
#[derive(Component)]
pub(crate) struct EntryListPosition(pub(crate) Option<u32>);

#[derive(Resource)]
pub(crate) struct EntryListPlacements(pub(crate) HashMap<&'static str, RawMarker>);

impl EntryListPlacements {
    pub(crate) fn get(&self, str: &'static str) -> RawMarker {
        self.0.get(str).copied().unwrap()
    }
}

pub(crate) fn setup_entry_list_placements(mut cmd: Commands) {
    cmd.insert_resource(EntryListPlacements(HashMap::new()));
}

pub(crate) fn entry_list_placements(
    mut placements: ResMut<EntryListPlacements>,
    entry_list_layout: Res<EntryListLayout>,
    list_dimensions: Res<ListDimensions>,
) {
    if entry_list_layout.is_changed() {
        placements.0.insert(
            "info-panel-offset",
            (entry_list_layout.horizontal_markers.0 - 2 * list_dimensions.entry.0).into(),
        );
        placements.0.insert(
            "edit-panel-offset",
            (entry_list_layout.horizontal_markers.0 - list_dimensions.entry.0).into(),
        );
        let midpoint =
            (placements.0.get("info-panel-offset").unwrap().0 as f32 / 2f32).ceil() as i32;
        placements.0.insert("info-panel-midpoint", midpoint.into());
        placements
            .0
            .insert("name-near-horizontal", (list_dimensions.padding).into());
        let nfh =
            placements.0.get("info-panel-midpoint").unwrap().0 - list_dimensions.padding.0 * 2;
        placements.0.insert("name-far-horizontal", nfh.into());
        let onh =
            placements.0.get("info-panel-midpoint").unwrap().0 + list_dimensions.padding.0 * 2;
        placements.0.insert("otp-near-horizontal", onh.into());
        let ofh = placements.0.get("info-panel-offset").unwrap().0
            - list_dimensions.padding.0 * 2
            - list_dimensions.content.0;
        placements.0.insert("otp-far-horizontal", ofh.into());
        let gbfh = placements.0.get("info-panel-offset").unwrap().0 - list_dimensions.padding.0;
        placements
            .0
            .insert("generate-button-far-horizontal", gbfh.into());
        let gbnh = placements
            .0
            .get("generate-button-far-horizontal")
            .unwrap()
            .0
            - list_dimensions.content.0;
        placements
            .0
            .insert("generate-button-near-horizontal", gbnh.into());
        let ebnh = placements.0.get("info-panel-offset").unwrap().0 + list_dimensions.padding.0;
        placements
            .0
            .insert("edit-button-near-horizontal", ebnh.into());
        let ebfh =
            placements.0.get("edit-button-near-horizontal").unwrap().0 + list_dimensions.content.0;
        placements
            .0
            .insert("edit-button-far-horizontal", ebfh.into());
        let bdnh = placements.0.get("edit-panel-offset").unwrap().0 + list_dimensions.padding.0;
        placements
            .0
            .insert("delete-button-near-horizontal", bdnh.into());
        let dbfh = placements.0.get("delete-button-near-horizontal").unwrap().0
            + list_dimensions.content.0;
        placements
            .0
            .insert("delete-button-far-horizontal", dbfh.into());
        let lx = placements.0.get("info-panel-midpoint").unwrap().0;
        placements.0.insert("line-x", lx.into());
        placements
            .0
            .insert("line-y-top", (list_dimensions.padding).into());
        placements.0.insert(
            "line-y-bottom",
            (list_dimensions.entry.0 - list_dimensions.padding.0).into(),
        );
    }
}

pub(crate) fn position(
    entries: Query<(&Entry, &EntryListPosition), Changed<EntryListPosition>>,
    entry_list_placements: Res<EntryListPlacements>,
    entry_list_layout: Res<EntryListLayout>,
    list_dimensions: Res<ListDimensions>,
    mut cmd: Commands,
) {
    for (entry, list_position) in entries.iter() {
        if let Some(pos) = list_position.0 {
            let anchor = GridPoint::from((
                entry_list_layout.anchor.x,
                entry_list_layout
                    .anchor
                    .y
                    .raw_offset((list_dimensions.entry + list_dimensions.padding).0 * pos as i32),
            ));
            cmd.entity(entry.name).insert(ResponsiveGridView::all_same((
                (
                    anchor
                        .x
                        .raw_offset(entry_list_placements.get("name-near-horizontal")),
                    anchor
                        .x
                        .raw_offset(entry_list_placements.get("name-far-horizontal")),
                ),
                (
                    anchor.y.raw_offset(list_dimensions.padding),
                    anchor
                        .y
                        .raw_offset(list_dimensions.padding + list_dimensions.content),
                ),
            )));
            cmd.entity(entry.otp).insert(ResponsiveGridView::all_same((
                (
                    anchor
                        .x
                        .raw_offset(entry_list_placements.get("otp-near-horizontal")),
                    anchor
                        .x
                        .raw_offset(entry_list_placements.get("otp-far-horizontal")),
                ),
                (
                    anchor.y.raw_offset(list_dimensions.padding),
                    anchor
                        .y
                        .raw_offset(list_dimensions.padding + list_dimensions.content),
                ),
            )));
            cmd.entity(entry.info_panel)
                .insert(ResponsiveGridView::all_same((
                    (
                        anchor.x,
                        anchor
                            .x
                            .raw_offset(entry_list_placements.get("info-panel-offset")),
                    ),
                    (anchor.y, anchor.y.raw_offset(list_dimensions.entry)),
                )));
            cmd.entity(entry.edit_panel)
                .insert(ResponsiveGridView::all_same((
                    (
                        anchor
                            .x
                            .raw_offset(entry_list_placements.get("info-panel-offset").0 - 1),
                        anchor
                            .x
                            .raw_offset(entry_list_placements.get("edit-panel-offset")),
                    ),
                    (anchor.y, anchor.y.raw_offset(list_dimensions.entry)),
                )));
            cmd.entity(entry.delete_panel)
                .insert(ResponsiveGridView::all_same((
                    (
                        anchor
                            .x
                            .raw_offset(entry_list_placements.get("edit-panel-offset").0 - 1),
                        anchor.x.raw_offset(entry_list_layout.horizontal_markers),
                    ),
                    (anchor.y, anchor.y.raw_offset(list_dimensions.entry)),
                )));
            cmd.entity(entry.generate_button)
                .insert(ResponsiveGridView::all_same((
                    (
                        anchor.x.raw_offset(
                            entry_list_placements.get("generate-button-near-horizontal"),
                        ),
                        anchor.x.raw_offset(
                            entry_list_placements.get("generate-button-far-horizontal"),
                        ),
                    ),
                    (
                        anchor.y.raw_offset(list_dimensions.padding),
                        anchor
                            .y
                            .raw_offset(list_dimensions.padding + list_dimensions.content),
                    ),
                )));
            cmd.entity(entry.edit_button)
                .insert(ResponsiveGridView::all_same((
                    (
                        anchor
                            .x
                            .raw_offset(entry_list_placements.get("edit-button-near-horizontal")),
                        anchor
                            .x
                            .raw_offset(entry_list_placements.get("edit-button-far-horizontal")),
                    ),
                    (
                        anchor.y.raw_offset(list_dimensions.padding),
                        anchor
                            .y
                            .raw_offset(list_dimensions.padding + list_dimensions.content),
                    ),
                )));
            cmd.entity(entry.delete_button)
                .insert(ResponsiveGridView::all_same((
                    (
                        anchor
                            .x
                            .raw_offset(entry_list_placements.get("delete-button-near-horizontal")),
                        anchor
                            .x
                            .raw_offset(entry_list_placements.get("delete-button-far-horizontal")),
                    ),
                    (
                        anchor.y.raw_offset(list_dimensions.padding),
                        anchor
                            .y
                            .raw_offset(list_dimensions.padding + list_dimensions.content),
                    ),
                )));
            cmd.entity(entry.line)
                .insert(ResponsivePathView::all_same(vec![
                    (
                        anchor.x.raw_offset(entry_list_placements.get("line-x")),
                        anchor.y.raw_offset(entry_list_placements.get("line-y-top")),
                    ),
                    (
                        anchor.x.raw_offset(entry_list_placements.get("line-x")),
                        anchor
                            .y
                            .raw_offset(entry_list_placements.get("line-y-bottom")),
                    ),
                ]));
        }
    }
}
