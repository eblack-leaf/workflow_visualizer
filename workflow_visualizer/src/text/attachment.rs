use bevy_ecs::prelude::IntoSystemConfig;

use crate::engen::{Attach, Engen};
use crate::text::backend_system::{
    create_render_groups, render_group_differences, reset_extraction, resize_receiver,
};
use crate::text::frontend_system::{
    bounds_diff, calc_area, calc_bound_from_guide, calc_line_structure, calc_scale_from_alignment,
    depth_diff, discard_out_of_bounds, letter_diff, manage_render_groups, place, position_diff,
    pull_differences, setup as frontend_setup, update_content, visible_area_diff,
};
use crate::text::renderer;
use crate::text::renderer::TextRenderer;
use crate::{spawn, TextBundle};

pub struct TextAttachment;

impl Attach for TextAttachment {
    fn attach(engen: &mut Engen) {
        engen.add_renderer::<TextRenderer>();
        engen.frontend.startup.add_system(frontend_setup);
        engen.frontend.main.add_system(spawn::<TextBundle>);
        engen.frontend.main.add_system(calc_bound_from_guide);
        engen.frontend.main.add_system(calc_scale_from_alignment);
        engen.frontend.main.add_system(update_content);
        engen.frontend.main.add_system(place);
        engen.frontend.main.add_system(calc_area);
        engen.frontend.main.add_system(manage_render_groups);
        engen.frontend.main.add_system(bounds_diff);
        engen.frontend.main.add_system(depth_diff);
        engen.frontend.main.add_system(position_diff);
        engen.frontend.main.add_system(visible_area_diff);
        engen.frontend.main.add_system(discard_out_of_bounds);
        engen.frontend.main.add_system(letter_diff);
        engen.frontend.main.add_system(pull_differences);
        engen.frontend.main.add_system(calc_line_structure);
        engen.backend.startup.add_system(renderer::setup);
        engen.backend.main.add_system(create_render_groups);
        engen.backend.main.add_system(render_group_differences);
        engen.backend.main.add_system(resize_receiver);
        engen.backend.main.add_system(reset_extraction);
    }
}
