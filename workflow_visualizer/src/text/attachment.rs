use bevy_ecs::prelude::IntoSystemConfig;

use crate::engen::{Attach, Engen};
use crate::text::backend_system::{
    create_render_groups, render_group_differences, reset_extraction, resize_receiver,
};
use crate::text::frontend_system::{
    bounds_diff, calc_bound_from_guide, calc_line_structure, calc_scale_from_alignment, depth_diff,
    discard_out_of_bounds, letter_diff, manage_render_groups, place, position_diff,
    pull_differences, setup as frontend_setup, update_content, visible_area_diff,
};
use crate::text::renderer;
use crate::text::renderer::TextRenderer;
use crate::viewport::viewport_attach;

use crate::{spawn, SyncPoint, Text};

pub struct TextAttachment;

impl Attach for TextAttachment {
    fn attach(engen: &mut Engen) {
        engen.add_renderer::<TextRenderer>();
        engen.frontend.startup.add_system(frontend_setup);
        engen
            .frontend
            .main
            .add_system(spawn::<Text>.in_set(SyncPoint::Spawn));
        engen
            .frontend
            .main
            .add_system(calc_bound_from_guide.in_set(SyncPoint::Reconfigure));
        engen
            .frontend
            .main
            .add_system(calc_scale_from_alignment.in_set(SyncPoint::Reconfigure));
        engen
            .frontend
            .main
            .add_system(update_content.in_set(SyncPoint::Reconfigure));
        engen
            .frontend
            .main
            .add_system(place.in_set(SyncPoint::Resolve));
        engen
            .frontend
            .main
            .add_system(manage_render_groups.in_set(SyncPoint::Resolve));
        engen
            .frontend
            .main
            .add_system(bounds_diff.in_set(SyncPoint::PushDiff));
        engen
            .frontend
            .main
            .add_system(depth_diff.in_set(SyncPoint::PushDiff));
        engen
            .frontend
            .main
            .add_system(position_diff.in_set(SyncPoint::PushDiff));
        engen
            .frontend
            .main
            .add_system(visible_area_diff.in_set(SyncPoint::PushDiff));
        engen.frontend.main.add_system(
            discard_out_of_bounds
                .in_set(SyncPoint::Resolve)
                .after(place),
        );
        engen.frontend.main.add_system(
            letter_diff
                .in_set(SyncPoint::Resolve)
                .after(discard_out_of_bounds),
        );
        engen
            .frontend
            .main
            .add_system(pull_differences.in_set(SyncPoint::Finish));
        engen
            .frontend
            .main
            .add_system(calc_line_structure.in_set(SyncPoint::Resolve));
        engen.backend.startup.add_system(
            renderer::setup
                .in_set(SyncPoint::Preparation)
                .after(viewport_attach),
        );
        engen
            .backend
            .main
            .add_system(create_render_groups.in_set(SyncPoint::Preparation));
        engen
            .backend
            .main
            .add_system(render_group_differences.in_set(SyncPoint::Resolve));
        engen
            .backend
            .main
            .add_system(resize_receiver.in_set(SyncPoint::Resolve));
        engen
            .backend
            .main
            .add_system(reset_extraction.in_set(SyncPoint::Finish));
    }
}
