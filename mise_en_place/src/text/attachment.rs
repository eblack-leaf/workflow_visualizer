use bevy_ecs::prelude::{IntoSystemDescriptor, SystemLabel};

use crate::engen::{Attach, Engen};
use crate::engen::{BackEndStartupStages, BackendStages, FrontEndStages, FrontEndStartupStages};
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

#[derive(SystemLabel)]
pub enum TextSystems {
    CreateRenderGroups,
    RenderGroupDiff,
}

pub struct TextAttachment;

impl Attach for TextAttachment {
    fn attach(engen: &mut Engen) {
        engen.add_renderer::<TextRenderer>();
        engen
            .frontend
            .startup
            .add_system_to_stage(FrontEndStartupStages::Startup, frontend_setup);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Spawn, spawn::<TextBundle>);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::CoordPrepare, intercept_area_adjust);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::ResolvePrepare, calc_bound_from_guide);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::ResolvePrepare, calc_scale_from_alignment);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::ResolvePrepare, update_content);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, place.label("place"));
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, calc_area.after("place"));
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::PushDiffs,
            manage_render_groups.before("out of bounds"),
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PushDiffs, bounds_diff);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PushDiffs, depth_diff);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PushDiffs, position_diff);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PushDiffs, visible_area_diff);
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::PushDiffs,
            discard_out_of_bounds.label("out of bounds"),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::PushDiffs,
            letter_diff.label("letter diff").after("out of bounds"),
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Finish, pull_differences);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Finish, calc_line_structure);
        engen
            .backend
            .startup
            .add_system_to_stage(BackEndStartupStages::Setup, renderer::setup);
        engen.backend.main.add_system_to_stage(
            BackendStages::Prepare,
            create_render_groups.label(TextSystems::CreateRenderGroups),
        );
        engen.backend.main.add_system_to_stage(
            BackendStages::Prepare,
            render_group_differences
                .label(TextSystems::RenderGroupDiff)
                .after(TextSystems::CreateRenderGroups),
        );
        engen.backend.main.add_system_to_stage(
            BackendStages::Prepare,
            resize_receiver.after(TextSystems::RenderGroupDiff),
        );
        engen
            .backend
            .main
            .add_system_to_stage(BackendStages::Last, reset_extraction);
    }
}
