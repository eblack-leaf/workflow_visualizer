use std::collections::HashMap;

use workflow_visualizer::{
    Attach, bevy_ecs, BundledIcon, Button, ButtonDespawn, ButtonType, Color, Disabled, Grid,
    GridPoint, IconBitmap, IconBitmapRequest, Line, Panel, PanelType, RawMarker,
    ResponsiveGridView, ResponsivePathView, ResponsiveUnit, Sender, SyncPoint, Text,
    TextScaleAlignment, TextValue, TextWrapStyle, TouchTrigger, Visualizer,
};
use workflow_visualizer::bevy_ecs::prelude::{
    Changed, Commands, Component, DetectChanges, Entity, EventReader, IntoSystemConfig, NonSend,
    Query, Res, ResMut, Resource,
};

use crate::Engen;
use crate::workflow::{Action, TokenName, TokenOtp};

pub struct EntryAttachment;

impl Attach for EntryAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.add_event::<ReceivedTokens>();
        visualizer.add_event::<ReadOtp>();
        visualizer.spawn(IconBitmapRequest::from((
            "edit",
            IconBitmap::bundled(BundledIcon::Edit),
        )));
        visualizer.job.task(Visualizer::TASK_STARTUP).add_systems((
            request_tokens.in_set(SyncPoint::PostInitialization),
            setup_paging.in_set(SyncPoint::PostInitialization),
            setup_entry_list_placements.in_set(SyncPoint::PostInitialization),
            setup_entry_scale.in_set(SyncPoint::PostInitialization),
            setup_removed_entry_indices.in_set(SyncPoint::PostInitialization),
            setup_total_entries.in_set(SyncPoint::PostInitialization),
            setup_entry_list.in_set(SyncPoint::PostInitialization),
            setup_bottom_panel_buttons.in_set(SyncPoint::PostResolve),
        ));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            entry_list_placements
                .in_set(SyncPoint::Preparation)
                .after(entry_list_layout),
            page_range.in_set(SyncPoint::Spawn).after(page_change),
            set_max_page.in_set(SyncPoint::Spawn).before(page_change),
            page_change.in_set(SyncPoint::Spawn),
            display_name.in_set(SyncPoint::Reconfigure),
            read_otp.in_set(SyncPoint::PostInitialization),
            display_otp.in_set(SyncPoint::Reconfigure),
            process_bottom_panel_buttons.in_set(SyncPoint::Process),
            place_bottom_panel_buttons.in_set(SyncPoint::Spawn),
            position.in_set(SyncPoint::Spawn).after(enable_by_index_change),
            enable.in_set(SyncPoint::Spawn).after(page_range),
            enable_by_index_change
                .in_set(SyncPoint::Spawn)
                .before(enable),
            removed_indices
                .in_set(SyncPoint::Spawn)
                .before(set_max_page),
            receive_tokens.in_set(SyncPoint::PostInitialization),
            entry_list_layout.in_set(SyncPoint::Preparation),
        ));
    }
}

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

pub(crate) fn page_range(
    entries_per: Res<EntriesPerPage>,
    mut range: ResMut<PageRange>,
    mut entries: Query<(&mut EntryEnabled, &mut EntryListPosition, &EntryIndex)>,
    page: Res<Page>,
) {
    if entries_per.is_changed() || page.is_changed() {
        range.set(page.0, entries_per.0);
        for (mut enabled, mut pos, index) in entries.iter_mut() {
            enable_entry(&range, &mut enabled, index, &mut pos);
        }
    }
}

#[derive(Resource, Copy, Clone)]
pub(crate) struct PageMax(pub(crate) u32);

pub(crate) fn set_max_page(
    mut page_max: ResMut<PageMax>,
    total_entries: Res<TotalEntries>,
    entries_per: Res<EntriesPerPage>,
) {
    if total_entries.is_changed() {
        page_max.0 = (total_entries.0 as f32 / entries_per.0 as f32).floor() as u32;
    }
}

#[derive(Resource)]
pub(crate) struct Page(pub(crate) u32);

#[derive(Resource)]
pub(crate) struct PageLeft(pub(crate) bool);

#[derive(Resource)]
pub(crate) struct PageRight(pub(crate) bool);

#[derive(Resource)]
pub(crate) struct PageRange(pub(crate) u32, pub(crate) u32);

impl PageRange {
    pub(crate) fn set(&mut self, page: u32, entries_per: u32) {
        self.0 = page * (entries_per - 1);
        self.1 = self.0 + (entries_per - 1);
    }
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
        let normal = index.0 % diff;
        normal
    }
}

pub(crate) fn setup_paging(mut cmd: Commands) {
    let page_max = PageMax(0);
    let page = Page(0);
    let page_left = PageLeft(false);
    let page_right = PageRight(false);
    let page_range = PageRange(0, 0);
    cmd.insert_resource(page_max);
    cmd.insert_resource(page);
    cmd.insert_resource(page_left);
    cmd.insert_resource(page_right);
    cmd.insert_resource(page_range);
}

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

#[derive(Component)]
pub(crate) struct EntryName(pub(crate) TokenName);

pub(crate) fn display_name(
    entries: Query<(&Entry, &EntryName), Changed<EntryName>>,
    mut text: Query<&mut TextValue>,
) {
    for (entry, entry_name) in entries.iter() {
        if let Ok(mut text) = text.get_mut(entry.name) {
            text.0 = entry_name.0.0.clone();
        }
    }
}

#[derive(Component)]
pub(crate) struct EntryOtp(pub(crate) Option<TokenOtp>);

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

#[derive(Resource)]
pub(crate) struct AddButton(pub(crate) Entity);

pub(crate) fn setup_bottom_panel_buttons(mut cmd: Commands, entry_scale: Res<EntryScale>) {
    let entity = cmd
        .spawn(Button::new(
            ButtonType::Press,
            4,
            Color::RED_ORANGE,
            Color::OFF_BLACK,
            "edit",
            "",
            0,
            entry_scale.button_icon_scale,
        ))
        .id();
    let add_button = AddButton(entity);
    cmd.insert_resource(add_button);
    let entity = cmd
        .spawn(Button::new(
            ButtonType::Press,
            4,
            Color::OFF_WHITE,
            Color::OFF_BLACK,
            "edit",
            "",
            0,
            entry_scale.button_icon_scale,
        ))
        .id();
    let other = cmd
        .spawn(Button::new(
            ButtonType::Press,
            4,
            Color::OFF_WHITE,
            Color::OFF_BLACK,
            "edit",
            "",
            0,
            entry_scale.button_icon_scale,
        ))
        .id();
    cmd.insert_resource(PageLeftButton(entity));
    cmd.insert_resource(PageRightButton(other));
}

pub(crate) fn place_bottom_panel_buttons(
    add_button: Res<AddButton>,
    entry_list_layout: Res<EntryListLayout>,
    page_left_button: Res<PageLeftButton>,
    page_right_button: Res<PageRightButton>,
    mut cmd: Commands,
) {
    if entry_list_layout.is_changed() {
        let horizontal_start =
            entry_list_layout.horizontal_markers.0 / 2 - ENTRY_LIST_CONTENT_HEIGHT / 2;
        let horizontal = (
            1.near().raw_offset(horizontal_start),
            1.near().raw_offset(horizontal_start + ENTRY_LIST_HEIGHT),
        );
        let vertical_start = entry_list_layout.vertical_markers.0 + ENTRY_LIST_PADDING;
        let vertical = (
            1.near().raw_offset(vertical_start),
            1.near().raw_offset(vertical_start + ENTRY_LIST_HEIGHT),
        );
        let view = (horizontal, vertical);
        cmd.entity(add_button.0)
            .insert(ResponsiveGridView::all_same(view));
        let page_left_horizontal = (
            horizontal
                .0
                .raw_offset(-ENTRY_LIST_PADDING - ENTRY_LIST_HEIGHT),
            horizontal.0.raw_offset(-ENTRY_LIST_PADDING),
        );
        let page_right_horizontal = (
            horizontal.1.raw_offset(ENTRY_LIST_PADDING),
            horizontal
                .1
                .raw_offset(ENTRY_LIST_PADDING + ENTRY_LIST_HEIGHT),
        );
        let page_left_view = (page_left_horizontal, vertical);
        let page_right_view = (page_right_horizontal, vertical);
        cmd.entity(page_left_button.0)
            .insert(ResponsiveGridView::all_same(page_left_view));
        cmd.entity(page_right_button.0)
            .insert(ResponsiveGridView::all_same(page_right_view));
    }
}

#[derive(Resource)]
pub(crate) struct PageLeftButton(pub(crate) Entity);

#[derive(Resource)]
pub(crate) struct PageRightButton(pub(crate) Entity);
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
) {
    if entry_list_layout.is_changed() {
        placements.0.insert(
            "info-panel-offset",
            (entry_list_layout.horizontal_markers.0 - 2 * ENTRY_LIST_HEIGHT - 2).into(),
        );
        placements.0.insert(
            "edit-panel-offset",
            (entry_list_layout.horizontal_markers.0 - ENTRY_LIST_HEIGHT - 1).into(),
        );
        let midpoint =
            (placements.0.get("info-panel-offset").unwrap().0 as f32 / 2f32).ceil() as i32;
        placements.0.insert("info-panel-midpoint", midpoint.into());
        placements
            .0
            .insert("name-near-horizontal", (ENTRY_LIST_PADDING).into());
        let nfh = placements.0.get("info-panel-midpoint").unwrap().0 - ENTRY_LIST_PADDING * 2;
        placements.0.insert("name-far-horizontal", nfh.into());
        let onh = placements.0.get("info-panel-midpoint").unwrap().0 + ENTRY_LIST_PADDING * 2;
        placements.0.insert("otp-near-horizontal", onh.into());
        let ofh = placements.0.get("info-panel-offset").unwrap().0
            - ENTRY_LIST_PADDING * 4
            - ENTRY_LIST_CONTENT_HEIGHT;
        placements.0.insert("otp-far-horizontal", ofh.into());
        let gbfh = placements.0.get("info-panel-offset").unwrap().0 - ENTRY_LIST_PADDING;
        placements
            .0
            .insert("generate-button-far-horizontal", gbfh.into());
        let gbnh = placements
            .0
            .get("generate-button-far-horizontal")
            .unwrap()
            .0
            - ENTRY_LIST_CONTENT_HEIGHT;
        placements
            .0
            .insert("generate-button-near-horizontal", gbnh.into());
        let ebnh = placements.0.get("info-panel-offset").unwrap().0 + ENTRY_LIST_PADDING;
        placements
            .0
            .insert("edit-button-near-horizontal", ebnh.into());
        let ebfh =
            placements.0.get("edit-button-near-horizontal").unwrap().0 + ENTRY_LIST_CONTENT_HEIGHT;
        placements
            .0
            .insert("edit-button-far-horizontal", ebfh.into());
        let bdnh = placements.0.get("edit-panel-offset").unwrap().0 + ENTRY_LIST_PADDING;
        placements
            .0
            .insert("delete-button-near-horizontal", bdnh.into());
        let dbfh = placements.0.get("delete-button-near-horizontal").unwrap().0
            + ENTRY_LIST_CONTENT_HEIGHT;
        placements
            .0
            .insert("delete-button-far-horizontal", dbfh.into());
        let lx = placements.0.get("info-panel-midpoint").unwrap().0;
        placements.0.insert("line-x", lx.into());
        placements
            .0
            .insert("line-y-top", (ENTRY_LIST_PADDING).into());
        placements.0.insert(
            "line-y-bottom",
            (ENTRY_LIST_HEIGHT - ENTRY_LIST_PADDING).into(),
        );
    }
}

pub(crate) fn position(
    entries: Query<(&Entry, &EntryListPosition), Changed<EntryListPosition>>,
    entry_list_placements: Res<EntryListPlacements>,
    entry_list_layout: Res<EntryListLayout>,
    mut cmd: Commands,
) {
    for (entry, list_position) in entries.iter() {
        if let Some(pos) = list_position.0 {
            let anchor = GridPoint::from((
                entry_list_layout.anchor.x,
                entry_list_layout
                    .anchor
                    .y
                    .raw_offset((ENTRY_LIST_HEIGHT + ENTRY_LIST_PADDING) * pos as i32),
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
                    anchor.y.raw_offset(ENTRY_LIST_PADDING),
                    anchor
                        .y
                        .raw_offset(ENTRY_LIST_PADDING + ENTRY_LIST_CONTENT_HEIGHT),
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
                    anchor.y.raw_offset(ENTRY_LIST_PADDING),
                    anchor
                        .y
                        .raw_offset(ENTRY_LIST_PADDING + ENTRY_LIST_CONTENT_HEIGHT),
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
                    (anchor.y, anchor.y.raw_offset(ENTRY_LIST_HEIGHT)),
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
                    (anchor.y, anchor.y.raw_offset(ENTRY_LIST_HEIGHT)),
                )));
            cmd.entity(entry.delete_panel)
                .insert(ResponsiveGridView::all_same((
                    (
                        anchor
                            .x
                            .raw_offset(entry_list_placements.get("edit-panel-offset").0 - 1),
                        anchor.x.raw_offset(entry_list_layout.horizontal_markers),
                    ),
                    (anchor.y, anchor.y.raw_offset(ENTRY_LIST_HEIGHT)),
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
                        anchor.y.raw_offset(ENTRY_LIST_PADDING),
                        anchor
                            .y
                            .raw_offset(ENTRY_LIST_PADDING + ENTRY_LIST_CONTENT_HEIGHT),
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
                        anchor.y.raw_offset(ENTRY_LIST_PADDING),
                        anchor
                            .y
                            .raw_offset(ENTRY_LIST_PADDING + ENTRY_LIST_CONTENT_HEIGHT),
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
                        anchor.y.raw_offset(ENTRY_LIST_PADDING),
                        anchor
                            .y
                            .raw_offset(ENTRY_LIST_PADDING + ENTRY_LIST_CONTENT_HEIGHT),
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
            TextScaleAlignment::Custom(entry_scale.text_scale),
            Color::from(Color::OFF_WHITE).with_alpha(1f32),
            TextWrapStyle::letter(),
        ))
        .id();
    let otp = cmd
        .spawn(Text::new(
            4,
            "------",
            TextScaleAlignment::Custom(entry_scale.text_scale),
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
            Color::from(Color::LIGHT_RED_ORANGE).with_alpha(1f32),
            Color::from(Color::DARK_RED_ORANGE).with_alpha(1f32),
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
            Color::from(Color::LIGHT_RED).with_alpha(1f32),
            Color::from(Color::DARK_RED).with_alpha(1f32),
            "edit",
            "",
            15,
            entry_scale.button_icon_scale,
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
        enable_entry(&page_range, &mut enabled, index, &mut list_position);
    }
}

fn enable_entry(
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
            delete_entry(&mut cmd, entity, entry);
        }
        total_entries.0 = 0;
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
            total_entries.0 += 1;
            index += 1;
        }
    }
}
pub(crate) const ENTRY_LIST_HEIGHT: i32 = 10;
pub(crate) const ENTRY_LIST_PADDING: i32 = 2;
pub(crate) const ENTRY_LIST_CONTENT_HEIGHT: i32 = ENTRY_LIST_HEIGHT - 2 * ENTRY_LIST_PADDING;
#[derive(Resource)]
pub(crate) struct EntryListLayout {
    pub(crate) horizontal_markers: RawMarker,
    pub(crate) vertical_markers: RawMarker,
    pub(crate) anchor: GridPoint,
}
#[derive(Resource)]
pub(crate) struct EntriesPerPage(pub(crate) u32);

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
) {
    if grid.is_changed() {
        let begin = grid.calc_horizontal_location(1.near());
        let end = grid.calc_horizontal_location(4.far());
        let horizontal_markers = end.0 - begin.0;
        let vertical_markers = grid.vertical_markers()
            - 2 * grid.markers_per_gutter()
            - ENTRY_LIST_HEIGHT
            - 2 * ENTRY_LIST_PADDING;
        entry_list_layout.horizontal_markers = horizontal_markers.max(1).into();
        entry_list_layout.vertical_markers = vertical_markers.max(1).into();
        entries_per.0 = (vertical_markers / (ENTRY_LIST_HEIGHT + ENTRY_LIST_PADDING)) as u32;
        entries_per.0 = entries_per.0.max(1);
    }
}

pub(crate) fn process_bottom_panel_buttons(
    mut page_right: ResMut<PageRight>,
    mut page_left: ResMut<PageLeft>,
    add: Res<AddButton>,
    page_left_button: Res<PageLeftButton>,
    page_right_button: Res<PageRightButton>,
    buttons: Query<&TouchTrigger>,
) {
    if let Ok(trigger) = buttons.get(add.0) {
        if trigger.triggered() {
            // add logic
        }
    }
    if let Ok(trigger) = buttons.get(page_left_button.0) {
        if trigger.triggered() {
            page_left.0 = true;
        }
    }
    if let Ok(trigger) = buttons.get(page_right_button.0) {
        if trigger.triggered() {
            page_right.0 = true;
        }
    }
}
