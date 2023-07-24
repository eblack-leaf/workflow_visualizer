use crate::system;
use crate::workflow::TokenName;
use workflow_visualizer::bevy_ecs;
use workflow_visualizer::bevy_ecs::prelude::{Entity, Events, IntoSystemConfig, Resource};
use workflow_visualizer::{
    Attach, Grid, GridPoint, PlacementReference, RawMarker, ResponsiveUnit, SyncPoint, TextScale,
    UserSpaceSyncPoint, Visualizer,
};

#[derive(Resource)]
pub(crate) struct SlotPool(pub(crate) Vec<TokenName>);

#[derive(Resource)]
pub(crate) struct SlotFills(pub(crate) Vec<TokenName>);

#[derive(Resource)]
pub(crate) struct Slots(pub(crate) Vec<Slot>);

impl Attach for Slots {
    fn attach(visualizer: &mut Visualizer) {
        visualizer
            .job
            .task(Visualizer::TASK_STARTUP)
            .add_systems((system::setup.in_set(UserSpaceSyncPoint::Initialization),));
        visualizer.add_event::<SlotFillEvent>();
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            system::update_blueprint.in_set(SyncPoint::Preparation),
            system::read_fill_event.in_set(SyncPoint::Preparation),
            system::fill_slots.in_set(UserSpaceSyncPoint::Process),
            system::process
                .in_set(UserSpaceSyncPoint::Process)
                .after(system::fill_slots),
        ));
    }
}

pub(crate) struct Slot {
    pub(crate) name_text: Entity,
    pub(crate) otp_text: Entity,
    pub(crate) generate_button: Entity,
    pub(crate) delete_button: Entity,
}

#[derive(Resource)]
pub(crate) struct SlotBlueprint {
    pub(crate) slots_per_page: usize,
    pub(crate) anchor: GridPoint,
    pub(crate) slot_offset_markers: RawMarker,
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
        let main_markers = total_horizontal_markers - 2 * segment_markers;
        let info_area_markers = main_markers - 2 * segment_padding - button_markers;
        let info_area_midpoint = (info_area_markers as f32 / 2f32).ceil() as i32;
        let name_bounds = info_area_midpoint - 2 * segment_padding;
        let otp_info_bounds = (info_area_markers - info_area_midpoint) - 2 * segment_padding;
        let slot_height = 10;
        let slot_content_height = slot_height - 2;
        let info_content_height_px = RawMarker(slot_content_height).to_pixel();
        let info_content_text_scale = TextScale(24u32); // programmatically pull from mapping using from_height(info_content_height_px)
        let button_content_height = slot_content_height - 2;
        let button_content_height_px = RawMarker(button_content_height).to_pixel();
        let button_text_scale = TextScale(18u32); // programmatically pull from mapping using from_height(button_content_height_px)
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
        Self {
            slots_per_page: num_slots as usize,
            anchor: (1.near(), 1.near()).into(),
            slot_offset_markers: slot_offset.into(),
        }
    }
    pub(crate) fn placements(&self, offset: usize) -> PlacementReference {
        let mut placement_reference = PlacementReference::new();
        let slot_left_top = (
            self.anchor.x,
            self.anchor.y.raw_offset(self.slot_offset_markers.0),
        );
        // use dimensions to offset from slot_anchor
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
