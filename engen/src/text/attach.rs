use bevy_ecs::prelude::IntoSystemDescriptor;

use crate::{Attach, Engen};
use crate::task::Stage;
use crate::text::compute_system::{
    bounds_diff, color_diff, depth_diff, discard_out_of_bounds, letter_diff, manage_render_groups,
    place, position_diff, pull_differences, setup as compute_setup,
};
use crate::text::render_system::{
    create_render_groups, render_group_differences, reset_extraction, setup as render_setup,
};
use crate::text::renderer::TextRenderer;

impl Attach for TextRenderer {
    fn attach(engen: &mut Engen) {
        engen
            .compute
            .startup
            .schedule
            .add_system_to_stage(Stage::Before, compute_setup);
        engen
            .compute
            .main
            .schedule
            .add_system_to_stage(Stage::After, manage_render_groups.before("place"));
        engen
            .compute
            .main
            .schedule
            .add_system_to_stage(Stage::After, bounds_diff);
        engen
            .compute
            .main
            .schedule
            .add_system_to_stage(Stage::After, color_diff);
        engen
            .compute
            .main
            .schedule
            .add_system_to_stage(Stage::After, depth_diff);
        engen
            .compute
            .main
            .schedule
            .add_system_to_stage(Stage::After, position_diff);
        engen
            .compute
            .main
            .schedule
            .add_system_to_stage(Stage::After, place.label("place"));
        engen.compute.main.schedule.add_system_to_stage(
            Stage::After,
            discard_out_of_bounds.label("out of bounds").after("place"),
        );
        engen
            .compute
            .main
            .schedule
            .add_system_to_stage(Stage::After, letter_diff.after("out of bounds"));
        engen
            .compute
            .main
            .schedule
            .add_system_to_stage(Stage::Last, pull_differences);
        // render side
        engen
            .render
            .startup
            .schedule
            .add_system_to_stage(Stage::Before, render_setup);
        engen
            .render
            .main
            .schedule
            .add_system_to_stage(Stage::First, create_render_groups);
        engen
            .render
            .main
            .schedule
            .add_system_to_stage(Stage::Before, render_group_differences);
        engen
            .render
            .main
            .schedule
            .add_system_to_stage(Stage::Last, reset_extraction);
    }
}
