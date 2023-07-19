use workflow_visualizer::{Attach, Grid, RawMarker, TextScale, UserSpaceSyncPoint, Visualizer};
use workflow_visualizer::bevy_ecs;
use workflow_visualizer::bevy_ecs::prelude::{Entity, IntoSystemConfig, Resource};

use crate::system;
use crate::workflow::TokenName;

pub(crate) struct Slot {
    pub(crate) name_text: Entity,
    pub(crate) otp_text: Entity,
    pub(crate) generate_button: Entity,
    pub(crate) delete_button: Entity,
    pub(crate) token_name: TokenName,
}
pub(crate) struct SlotBlueprint {

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
        let info_content_text_scale = TextScale(24u32);// programmatically pull from mapping using from_height(info_content_height_px)
        let button_content_height = slot_content_height - 2;
        let button_content_height_px = RawMarker(button_content_height).to_pixel();
        let button_text_scale = TextScale(18u32);// programmatically pull from mapping using from_height(button_content_height_px)
        let total_vertical_markers = grid.vertical_markers() - grid.markers_per_gutter() * 2 - button_markers - 2 * segment_padding;
        let slot_padding = 2;
        let slot_offset = slot_padding + slot_height;
        let mut num_slots = total_vertical_markers / (slot_offset);
        if total_vertical_markers % (slot_offset) >= slot_height {
            num_slots += 1;
        }
        Self {

        }
    }
}
#[derive(Resource)]
pub(crate) struct SlotController {
    pub(crate) slots: Vec<Slot>,
    pub(crate) blueprint: SlotBlueprint,
}

impl SlotController {
    pub(crate) fn new(grid: &Grid) -> Self {
        Self {
            slots: vec![],
            blueprint: SlotBlueprint::new(grid),
        }
    }
    pub(crate) fn fill_slot<N: Into<TokenName>>(&mut self, name: N) {
        // use blueprint to place elements and return PlacementReference of views for elements
    }
}
impl Attach for SlotController {
    fn attach(visualizer: &mut Visualizer) {
        visualizer
            .job
            .task(Visualizer::TASK_STARTUP)
            .add_systems((system::setup.in_set(UserSpaceSyncPoint::Initialization), ));
        visualizer
            .job
            .task(Visualizer::TASK_MAIN)
            .add_systems((system::send_event.in_set(UserSpaceSyncPoint::Process), ));
    }
}
