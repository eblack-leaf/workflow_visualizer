use bevy_ecs::prelude::{IntoSystemDescriptor, StageLabel, SystemLabel, SystemStage};

use crate::{Attach, BackendStages, BackEndStartupStages, FrontEndStages, FrontEndStartupStages, Stove};
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

#[derive(StageLabel)]
pub enum TextStages {
    CalcTextScale,
    CalcArea,
    TextFrontEnd,
    TextBackEnd,
}

impl Attach for TextRenderer {
    fn attach(stove: &mut Stove) {
        stove
            .frontend
            .startup
            .add_system_to_stage(FrontEndStartupStages::Startup, frontend_setup);
        stove.frontend.main.add_stage_before(
            FrontEndStages::VisibilityPreparation,
            TextStages::CalcTextScale,
            SystemStage::single(calc_scale_from_alignment),
        );
        stove.frontend.main.add_stage_after(
            TextStages::CalcTextScale,
            TextStages::CalcArea,
            SystemStage::single(calc_area),
        );
        stove.frontend.main.add_stage_after(FrontEndStages::ResolveVisibility, TextStages::TextFrontEnd, SystemStage::parallel());
        stove
            .frontend
            .main
            .add_system_to_stage(TextStages::TextFrontEnd, manage_render_groups.before("place"));
        stove
            .frontend
            .main
            .add_system_to_stage(TextStages::TextFrontEnd, bounds_diff);
        stove
            .frontend
            .main
            .add_system_to_stage(TextStages::TextFrontEnd, color_diff);
        stove
            .frontend
            .main
            .add_system_to_stage(TextStages::TextFrontEnd, depth_diff);
        stove
            .frontend
            .main
            .add_system_to_stage(TextStages::TextFrontEnd, position_diff);
        stove
            .frontend
            .main
            .add_system_to_stage(TextStages::TextFrontEnd, place.label("place"));
        stove.frontend.main.add_system_to_stage(
            TextStages::TextFrontEnd,
            discard_out_of_bounds.label("out of bounds").after("place"),
        );
        stove
            .frontend
            .main
            .add_system_to_stage(TextStages::TextFrontEnd, letter_diff.after("out of bounds"));
        stove
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Last, pull_differences);
        // render side
        stove
            .backend
            .startup
            .add_system_to_stage(BackEndStartupStages::Setup, backend_setup);
        stove
            .backend
            .main
            .add_system_to_stage(BackendStages::Prepare, create_render_groups.label(TextSystems::CreateRenderGroups));
        stove
            .backend
            .main
            .add_system_to_stage(BackendStages::Prepare, render_group_differences.label(TextSystems::RenderGroupDiff).after(TextSystems::CreateRenderGroups));
        stove
            .backend
            .main
            .add_system_to_stage(BackendStages::Prepare, resize_receiver.after(TextSystems::RenderGroupDiff));
        stove
            .backend
            .main
            .add_system_to_stage(BackendStages::Last, reset_extraction);
    }
}

#[derive(SystemLabel)]
enum TextSystems {
    CreateRenderGroups,
    RenderGroupDiff,
}
