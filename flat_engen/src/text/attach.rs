use bevy_ecs::prelude::{IntoSystemDescriptor, SystemStage};

use crate::task::Stage;
use crate::text::compute_system::{
    bounds_diff, calc_area, calc_scale_from_alignment, color_diff, depth_diff,
    discard_out_of_bounds, letter_diff, manage_render_groups, place, position_diff,
    pull_differences, setup as frontend_setup,
};
use crate::text::render_system::{
    create_render_groups, render_group_differences, reset_extraction, resize_receiver,
    setup as backend_setup,
};
use crate::text::renderer::TextRenderer;
use crate::{Attach, Engen};

impl Attach for TextRenderer {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .startup
            .schedule
            .add_system_to_stage(Stage::Before, frontend_setup);
        engen.frontend.main.schedule.add_stage_before(
            Stage::Before,
            "calc scale alignment",
            SystemStage::single(calc_scale_from_alignment),
        );
        engen.frontend.main.schedule.add_stage_after(
            Stage::Before,
            "calc area",
            SystemStage::single(calc_area),
        );
        engen
            .frontend
            .main
            .schedule
            .add_system_to_stage(Stage::After, manage_render_groups.before("place"));
        engen
            .frontend
            .main
            .schedule
            .add_system_to_stage(Stage::After, bounds_diff);
        engen
            .frontend
            .main
            .schedule
            .add_system_to_stage(Stage::After, color_diff);
        engen
            .frontend
            .main
            .schedule
            .add_system_to_stage(Stage::After, depth_diff);
        engen
            .frontend
            .main
            .schedule
            .add_system_to_stage(Stage::After, position_diff);
        engen
            .frontend
            .main
            .schedule
            .add_system_to_stage(Stage::After, place.label("place"));
        engen.frontend.main.schedule.add_system_to_stage(
            Stage::After,
            discard_out_of_bounds.label("out of bounds").after("place"),
        );
        engen
            .frontend
            .main
            .schedule
            .add_system_to_stage(Stage::After, letter_diff.after("out of bounds"));
        engen
            .frontend
            .main
            .schedule
            .add_system_to_stage(Stage::Last, pull_differences);
        // render side
        engen
            .backend
            .startup
            .schedule
            .add_system_to_stage(Stage::Before, backend_setup);
        engen
            .backend
            .main
            .schedule
            .add_system_to_stage(Stage::First, create_render_groups);
        engen
            .backend
            .main
            .schedule
            .add_system_to_stage(Stage::Before, render_group_differences);
        engen
            .backend
            .main
            .schedule
            .add_system_to_stage(Stage::Before, resize_receiver);
        engen
            .backend
            .main
            .schedule
            .add_system_to_stage(Stage::Last, reset_extraction);
    }
}
