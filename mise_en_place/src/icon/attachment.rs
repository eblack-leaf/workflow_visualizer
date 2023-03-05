use bevy_ecs::prelude::{IntoSystemDescriptor, StageLabel};

use crate::{IconBundle, spawn};
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
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Spawn, spawn::<IconBundle>);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, calc_area);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PushDiffs, initialization);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PushDiffs, position_cache_check);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PushDiffs, color_invert_cache_check);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PushDiffs, area_cache_check);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PushDiffs, depth_cache_check);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PushDiffs, color_cache_check);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PushDiffs, secondary_color_cache_check);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PushDiffs, icon_key_cache_check);
    }
}
