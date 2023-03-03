use bevy_ecs::prelude::{IntoSystemDescriptor, StageLabel, SystemLabel, SystemStage};

use crate::engen::{Attach, Engen};
use crate::engen::{BackendStages, BackEndStartupStages, FrontEndStages, FrontEndStartupStages};
use crate::text::backend_system::{
    create_render_groups, render_group_differences, reset_extraction, resize_receiver,
};
use crate::text::frontend_system::{
    bounds_diff, calc_area, calc_bound_from_guide, calc_scale_from_alignment, depth_diff,
    discard_out_of_bounds, intercept_area_adjust, letter_diff, manage_render_groups, place,
    position_diff, pull_differences, setup as frontend_setup, visible_area_diff,
};
use crate::text::renderer;
use crate::text::renderer::TextRenderer;

#[derive(SystemLabel)]
pub enum TextSystems {
    CreateRenderGroups,
    RenderGroupDiff,
}

#[derive(StageLabel)]
pub enum TextStages {
    PlacementPreparation,
}

pub struct TextAttachment;

impl Attach for TextAttachment {
    fn attach(engen: &mut Engen) {
        engen.add_renderer::<TextRenderer>();
        engen
            .frontend
            .startup
            .add_system_to_stage(FrontEndStartupStages::Startup, frontend_setup);
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::CoordHook, intercept_area_adjust,
        );
        engen.frontend.main.add_stage_before(
            FrontEndStages::PostProcessPreparation,
            TextStages::PlacementPreparation,
            SystemStage::parallel()
                .with_system(calc_bound_from_guide)
                .with_system(calc_scale_from_alignment),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::PostProcessPreparation,
            place.label("place"),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::PostProcessPreparation, calc_area.after("place"),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Resolve,
            manage_render_groups.before("out of bounds"),
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, bounds_diff);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, depth_diff);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, position_diff);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, visible_area_diff);
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Resolve,
            discard_out_of_bounds.label("out of bounds"),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Resolve,
            letter_diff.label("letter diff").after("out of bounds"),
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Finish, pull_differences);
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
