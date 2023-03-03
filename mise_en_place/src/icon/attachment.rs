use bevy_ecs::prelude::{IntoSystemDescriptor, StageLabel, SystemStage};

use crate::engen::{Attach, Engen};
use crate::engen::{BackendStages, BackEndStartupStages, FrontEndStages, FrontEndStartupStages};
use crate::icon::backend_system::{process_differences, read_add_requests, setup};
use crate::icon::frontend_system::{
    area_cache_check, calc_area, color_cache_check, color_invert_cache_check, depth_cache_check,
    frontend_setup, icon_key_cache_check, initialization, position_cache_check,
    secondary_color_cache_check,
};
use crate::icon::IconRenderer;

pub struct IconAttachment;

#[derive(StageLabel)]
pub enum IconStages {
    CalcArea,
}

impl Attach for IconAttachment {
    fn attach(engen: &mut Engen) {
        engen.add_renderer::<IconRenderer>();
        engen
            .backend
            .startup
            .add_system_to_stage(BackEndStartupStages::Setup, setup);
        engen
            .backend
            .main
            .add_system_to_stage(BackendStages::Prepare, read_add_requests.label("add mesh"));
        engen.backend.main.add_system_to_stage(
            BackendStages::Prepare,
            process_differences.after("add mesh"),
        );
        engen
            .frontend
            .startup
            .add_system_to_stage(FrontEndStartupStages::Startup, frontend_setup);
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::PostProcessPreparation,
            calc_area,
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, initialization);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, position_cache_check);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, color_invert_cache_check);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, area_cache_check);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, depth_cache_check);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, color_cache_check);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, secondary_color_cache_check);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, icon_key_cache_check);
    }
}
