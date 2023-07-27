use workflow_visualizer::{
    bevy_ecs, BundledIcon, GridView, IconBitmap, IconBitmapRequest,
};
use workflow_visualizer::{
    Attach, Grid, GridPoint, PlacementReference, RawMarker, ResponsiveUnit, SyncPoint, TextScale,
    UserSpaceSyncPoint, Visualizer,
};
use workflow_visualizer::bevy_ecs::prelude::{Entity, IntoSystemConfig, Resource};

use crate::system;
use crate::workflow::{TokenName, TokenOtp};

#[derive(Resource)]
pub(crate) struct AddButton(pub(crate) Entity);

#[derive(Resource)]
pub(crate) struct SlotPool(pub(crate) Vec<TokenName>);

#[derive(Resource)]
pub(crate) struct SlotFills(pub(crate) Vec<TokenName>);
#[derive(Resource)]
pub(crate) struct SlotFillsCache(pub(crate) Vec<TokenName>);
#[derive(Resource)]
pub(crate) struct Slots(pub(crate) Vec<Slot>);

#[derive(Resource)]
pub(crate) struct SlotPaging(pub(crate) u32);

impl SlotPaging {
    pub(crate) fn range(&self, slots_per_page: usize) -> (usize, usize) {
        let start = self.0 as usize * slots_per_page;
        let end = start + slots_per_page - 1;
        (start, end)
    }
}
pub(crate) struct OtpRead {
    pub(crate) name: TokenName,
    pub(crate) otp: TokenOtp,
}
impl Attach for Slots {
    fn attach(visualizer: &mut Visualizer) {
        visualizer
            .job
            .task(Visualizer::TASK_STARTUP)
            .add_systems((system::setup.in_set(UserSpaceSyncPoint::Initialization), ));
        visualizer.add_event::<SlotFillEvent>();
        visualizer.add_event::<OtpRead>();
        visualizer.spawn(IconBitmapRequest::from((
            "edit",
            IconBitmap::bundled(BundledIcon::Edit),
        )));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            system::update_blueprint.in_set(SyncPoint::Preparation),
            system::read_fill_event.in_set(SyncPoint::Preparation),
            system::fill_slots.in_set(UserSpaceSyncPoint::Process),
            system::process
                .in_set(UserSpaceSyncPoint::Process)
                .after(system::fill_slots),
            system::read_otp.in_set(SyncPoint::Reconfigure),
        ));
    }
}

pub(crate) struct Slot {
    pub(crate) name_text: Entity,
    pub(crate) otp_text: Entity,
    pub(crate) generate_button: Entity,
    pub(crate) delete_button: Entity,
    pub(crate) info_line: Entity,
    pub(crate) edit_button: Entity,
    pub(crate) info_panel: Entity,
    pub(crate) edit_panel: Entity,
    pub(crate) delete_panel: Entity,
}

#[derive(Resource)]
pub(crate) struct SlotBlueprint {
    pub(crate) slots_per_page: usize,
    pub(crate) anchor: GridPoint,
    pub(crate) slot_offset_markers: RawMarker,
    pub(crate) add_button_view: GridView,
    pub(crate) info_text_scale: TextScale,
    pub(crate) button_icon_scale: u32,
    pub(crate) slot_height: RawMarker,
    pub(crate) total_width_markers: RawMarker,
    pub(crate) total_vertical_markers: RawMarker,
    info_panel_offset: i32,
    edit_panel_offset: i32,
    line_x: i32,
    line_y_top: i32,
    line_y_bottom: i32,
    name_far_horizontal: i32,
    name_near_horizontal: i32,
    otp_near_horizontal: i32,
    otp_far_horizontal: i32,
    slot_content_height: i32,
    segment_padding: i32,
    button_content_height: i32,
    generate_button_near_horizontal: i32,
    generate_button_far_horizontal: i32,
    edit_button_near_horizontal: i32,
    edit_button_far_horizontal: i32,
    delete_button_near_horizontal: i32,
    delete_button_far_horizontal: i32,
    button_markers: i32,
}

impl SlotBlueprint {
    pub(crate) fn new(grid: &Grid) -> Self {
        let begin_vertical = grid.calc_vertical_location(1.near());
        let begin_horizontal = grid.calc_horizontal_location(1.near());
        let end_horizontal = grid.calc_horizontal_location(4.far());
        let total_horizontal_markers = end_horizontal.0 - begin_horizontal.0;
        let segment_markers = (total_horizontal_markers as f32 * 0.10f32).ceil() as i32;
        let segment_padding = 1;
        let button_markers = segment_markers - 2 * segment_padding;
        let info_panel_offset = (total_horizontal_markers as f32 * 0.67f32).ceil() as i32;
        let edit_panel_offset = (total_horizontal_markers as f32 * 0.81f32).ceil() as i32;
        let info_panel_midpoint = (info_panel_offset as f32 / 2f32).ceil() as i32;
        let name_far_horizontal = info_panel_midpoint - segment_padding * 2;
        let name_near_horizontal = segment_padding;
        let otp_near_horizontal = info_panel_midpoint + segment_padding * 2;
        let otp_far_horizontal = info_panel_offset - segment_padding * 4 - button_markers;
        let generate_button_near_horizontal = otp_far_horizontal + segment_padding;
        let generate_button_far_horizontal = generate_button_near_horizontal + button_markers;
        let edit_button_near_horizontal = info_panel_offset + segment_padding;
        let edit_button_far_horizontal = edit_button_near_horizontal + button_markers;
        let delete_button_near_horizontal = edit_panel_offset + segment_padding;
        let delete_button_far_horizontal = delete_button_near_horizontal + button_markers;
        let line_x = info_panel_midpoint;
        let slot_height = 10;
        let slot_content_height = slot_height - 2;
        let line_y_top = segment_padding;
        let line_y_bottom = segment_padding + slot_content_height;
        let info_content_text_scale = TextScale(24u32); // programmatically pull from mapping using from_height(info_content_height_px)
        let button_content_height = slot_content_height - 2;
        let button_content_height_px = RawMarker(button_content_height).to_pixel();
        let button_text_scale = 18u32; // programmatically pull from mapping using from_height(button_content_height_px)
        let total_vertical_markers = grid.vertical_markers()
            - grid.markers_per_gutter() * 2
            - button_markers
            - 2 * segment_padding;
        let slot_padding = 2;
        let slot_offset = slot_padding + slot_height;
        let mut num_slots = total_vertical_markers / (slot_offset);
        if total_vertical_markers % (slot_offset) >= slot_height {
            num_slots += 1;
        }
        let add_button_vertical_start = total_vertical_markers + segment_padding;
        let add_button_view_vertical = (
            1.near().raw_offset(add_button_vertical_start),
            1.near()
                .raw_offset(add_button_vertical_start + button_markers),
        );
        let add_button_horizontal_start = total_horizontal_markers / 2 - button_markers / 2;
        let add_button_view_horizontal = (
            1.near().raw_offset(add_button_horizontal_start),
            1.near()
                .raw_offset(add_button_horizontal_start + button_markers),
        );
        Self {
            slots_per_page: num_slots as usize,
            anchor: (1.near(), 1.near()).into(),
            slot_offset_markers: slot_offset.into(),
            add_button_view: (add_button_view_horizontal, add_button_view_vertical).into(),
            info_text_scale: info_content_text_scale,
            button_icon_scale: button_text_scale,
            slot_height: RawMarker(slot_height),
            total_width_markers: RawMarker(total_horizontal_markers),
            total_vertical_markers: RawMarker(total_vertical_markers),
            info_panel_offset,
            edit_panel_offset,
            line_x,
            line_y_top,
            line_y_bottom,
            name_far_horizontal,
            name_near_horizontal,
            otp_near_horizontal,
            otp_far_horizontal,
            slot_content_height,
            segment_padding,
            button_content_height,
            generate_button_near_horizontal,
            generate_button_far_horizontal,
            edit_button_near_horizontal,
            edit_button_far_horizontal,
            delete_button_near_horizontal,
            delete_button_far_horizontal,
            button_markers,
        }
    }
    pub(crate) fn placements(&self, offset: usize) -> PlacementReference {
        let mut placement_reference = PlacementReference::new();
        let slot_left_top = (
            self.anchor.x,
            self.anchor
                .y
                .raw_offset(self.slot_offset_markers.0 * offset as i32),
        );
        // use dimensions to offset from slot_anchor
        placement_reference.add_view(
            "info-panel",
            (
                (
                    slot_left_top.0,
                    slot_left_top.0.raw_offset(self.info_panel_offset),
                ),
                (
                    slot_left_top.1,
                    slot_left_top.1.raw_offset(self.slot_height.0),
                ),
            ),
        );
        placement_reference.add_view(
            "edit-panel",
            (
                (
                    slot_left_top.0,
                    slot_left_top.0.raw_offset(self.edit_panel_offset),
                ),
                (
                    slot_left_top.1,
                    slot_left_top.1.raw_offset(self.slot_height.0),
                ),
            ),
        );
        placement_reference.add_view(
            "delete-panel",
            (
                (
                    slot_left_top.0,
                    slot_left_top.0.raw_offset(self.total_width_markers.0),
                ),
                (
                    slot_left_top.1,
                    slot_left_top.1.raw_offset(self.slot_height.0),
                ),
            ),
        );
        placement_reference.add_view(
            "name-text",
            (
                (
                    slot_left_top.0.raw_offset(self.name_near_horizontal),
                    slot_left_top.0.raw_offset(self.name_far_horizontal),
                ),
                (
                    slot_left_top.1.raw_offset(self.segment_padding),
                    slot_left_top
                        .1
                        .raw_offset(self.segment_padding + self.slot_content_height),
                ),
            ),
        );
        placement_reference.add_view(
            "otp-text",
            (
                (
                    slot_left_top.0.raw_offset(self.otp_near_horizontal),
                    slot_left_top.0.raw_offset(self.otp_far_horizontal),
                ),
                (
                    slot_left_top.1.raw_offset(self.segment_padding),
                    slot_left_top
                        .1
                        .raw_offset(self.segment_padding + self.slot_content_height),
                ),
            ),
        );
        placement_reference.add_view(
            "generate-button",
            (
                (
                    slot_left_top
                        .0
                        .raw_offset(self.generate_button_near_horizontal),
                    slot_left_top
                        .0
                        .raw_offset(self.generate_button_far_horizontal),
                ),
                (
                    slot_left_top.1.raw_offset(self.segment_padding),
                    slot_left_top
                        .1
                        .raw_offset(self.segment_padding + self.button_markers),
                ),
            ),
        );
        placement_reference.add_view(
            "edit-button",
            (
                (
                    slot_left_top.0.raw_offset(self.edit_button_near_horizontal),
                    slot_left_top.0.raw_offset(self.edit_button_far_horizontal),
                ),
                (
                    slot_left_top.1.raw_offset(self.segment_padding),
                    slot_left_top
                        .1
                        .raw_offset(self.segment_padding + self.button_markers),
                ),
            ),
        );
        placement_reference.add_view(
            "delete-button",
            (
                (
                    slot_left_top
                        .0
                        .raw_offset(self.delete_button_near_horizontal),
                    slot_left_top
                        .0
                        .raw_offset(self.delete_button_far_horizontal),
                ),
                (
                    slot_left_top.1.raw_offset(self.segment_padding),
                    slot_left_top
                        .1
                        .raw_offset(self.segment_padding + self.button_markers),
                ),
            ),
        );
        placement_reference.add_path_view(
            "info-line",
            vec![
                (
                    slot_left_top.0.raw_offset(self.line_x),
                    slot_left_top.1.raw_offset(self.line_y_top),
                ),
                (
                    (slot_left_top.0.raw_offset(self.line_x)),
                    slot_left_top.1.raw_offset(self.line_y_bottom),
                ),
            ],
        );
        placement_reference
    }
}

pub(crate) struct SlotFillEvent {
    pub(crate) tokens: Vec<TokenName>,
}

impl SlotFillEvent {
    pub(crate) fn new(tokens: Vec<TokenName>) -> Self {
        Self { tokens }
    }
}
