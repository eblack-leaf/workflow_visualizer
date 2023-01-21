use bevy_ecs::prelude::IntoSystemDescriptor;

use crate::{Attach, Engen};
use crate::clean_text::cache::{bounds_diff, letter_diff};
use crate::clean_text::extraction::pull_differences;
use crate::clean_text::place::{discard_out_of_bounds, place};
use crate::clean_text::render_group::{color_diff, depth_diff, manage_render_groups, position_diff};
use crate::clean_text::renderer::Renderer;
use crate::task::Stage;

impl Attach for Renderer {
    fn attach(engen: &mut Engen) {
        engen.compute.main.schedule.add_system_to_stage(Stage::After, manage_render_groups.before("place"));
        engen.compute.main.schedule.add_system_to_stage(Stage::After, bounds_diff);
        engen.compute.main.schedule.add_system_to_stage(Stage::After, color_diff);
        engen.compute.main.schedule.add_system_to_stage(Stage::After, depth_diff);
        engen.compute.main.schedule.add_system_to_stage(Stage::After, position_diff);
        engen.compute.main.schedule.add_system_to_stage(Stage::After, place.label("place"));
        engen.compute.main.schedule.add_system_to_stage(Stage::After, discard_out_of_bounds.label("out of bounds").after("place"));
        engen.compute.main.schedule.add_system_to_stage(Stage::After, letter_diff.after("out of bounds"));
        engen.compute.main.schedule.add_system_to_stage(Stage::Last, pull_differences);
    }
}