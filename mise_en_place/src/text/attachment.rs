use bevy_ecs::prelude::IntoSystemConfig;

use crate::engen::{Attach, Engen};
use crate::engen::{
    BackEndStartupBuckets, BackendBuckets, FrontEndBuckets, FrontEndStartupBuckets,
};
use crate::text::backend_system::{
    create_render_groups, render_group_differences, reset_extraction, resize_receiver,
};
use crate::text::frontend_system::{
    bounds_diff, calc_area, calc_bound_from_guide, calc_line_structure, calc_scale_from_alignment,
    depth_diff, discard_out_of_bounds, intercept_area_adjust, letter_diff, manage_render_groups,
    place, position_diff, pull_differences, setup as frontend_setup, update_content,
    visible_area_diff,
};
use crate::text::renderer;
use crate::text::renderer::TextRenderer;
use crate::{spawn, TextBundle};

pub struct TextAttachment;

impl Attach for TextAttachment {
    fn attach(engen: &mut Engen) {
        engen.add_renderer::<TextRenderer>();
        engen
            .frontend
            .startup
            .add_system(frontend_setup.in_set(FrontEndStartupBuckets::Startup));
        engen
            .frontend
            .main
            .add_system(spawn::<TextBundle>.in_set(FrontEndBuckets::Spawn));
        engen
            .frontend
            .main
            .add_system(intercept_area_adjust.in_set(FrontEndBuckets::CoordPrepare));
        engen
            .frontend
            .main
            .add_system(calc_bound_from_guide.in_set(FrontEndBuckets::ResolvePrepare));
        engen
            .frontend
            .main
            .add_system(calc_scale_from_alignment.in_set(FrontEndBuckets::ResolvePrepare));
        engen
            .frontend
            .main
            .add_system(update_content.in_set(FrontEndBuckets::ResolvePrepare));
        engen
            .frontend
            .main
            .add_system(place.in_set(FrontEndBuckets::ResolveStart));
        engen
            .frontend
            .main
            .add_system(calc_area.in_set(FrontEndBuckets::Resolve));
        engen.frontend.main.add_system(
            manage_render_groups
                .in_set(FrontEndBuckets::PushDiffs)
                .before(discard_out_of_bounds),
        );
        engen
            .frontend
            .main
            .add_system(bounds_diff.in_set(FrontEndBuckets::PushDiffs));
        engen
            .frontend
            .main
            .add_system(depth_diff.in_set(FrontEndBuckets::PushDiffs));
        engen
            .frontend
            .main
            .add_system(position_diff.in_set(FrontEndBuckets::PushDiffs));
        engen
            .frontend
            .main
            .add_system(visible_area_diff.in_set(FrontEndBuckets::PushDiffs));
        engen
            .frontend
            .main
            .add_system(discard_out_of_bounds.in_set(FrontEndBuckets::PushDiffs));
        engen.frontend.main.add_system(
            letter_diff
                .after(discard_out_of_bounds)
                .in_set(FrontEndBuckets::PushDiffs),
        );
        engen
            .frontend
            .main
            .add_system(pull_differences.in_set(FrontEndBuckets::Finish));
        engen
            .frontend
            .main
            .add_system(calc_line_structure.in_set(FrontEndBuckets::Finish));
        engen
            .backend
            .startup
            .add_system(renderer::setup.in_set(BackEndStartupBuckets::Prepare));
        engen
            .backend
            .main
            .add_system(create_render_groups.in_set(BackendBuckets::Prepare));
        engen.backend.main.add_system(
            render_group_differences
                .in_set(BackendBuckets::Prepare)
                .after(create_render_groups),
        );
        engen.backend.main.add_system(
            resize_receiver
                .after(render_group_differences)
                .in_set(BackendBuckets::Prepare),
        );
        engen
            .backend
            .main
            .add_system(reset_extraction.in_set(BackendBuckets::Last));
    }
}
