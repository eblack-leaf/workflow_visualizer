use bevy_ecs::prelude::{StageLabel, SystemStage};

use crate::{gfx, Job};
use crate::engen::Container;

#[derive(StageLabel)]
pub enum FrontEndStartupStages {
    Startup,
    Initialization,
}

#[derive(StageLabel)]
pub enum FrontEndStages {
    First,
    Resize,
    PreProcess,
    PreProcessResolve,
    Process,
    CoordAdjust,
    VisibilityPreparation,
    ResolveVisibility,
    PostProcess,
    Last,
}

#[derive(StageLabel)]
pub enum BackEndStartupStages {
    Startup,
    Setup,
    PostSetup,
}

#[derive(StageLabel)]
pub enum BackendStages {
    Initialize,
    GfxSurfaceResize,
    Resize,
    Prepare,
    Last,
}

pub(crate) fn staged_frontend() -> Job {
    let mut job = Job::new();
    job.startup
        .add_stage(FrontEndStartupStages::Startup, SystemStage::parallel());
    job.startup.add_stage(
        FrontEndStartupStages::Initialization,
        SystemStage::parallel(),
    );
    job.main
        .add_stage(FrontEndStages::First, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::Resize, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::PreProcess, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::PreProcessResolve, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::Process, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::CoordAdjust, SystemStage::parallel());
    job.main.add_stage(
        FrontEndStages::VisibilityPreparation,
        SystemStage::parallel(),
    );
    job.main
        .add_stage(FrontEndStages::ResolveVisibility, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::PostProcess, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::Last, SystemStage::parallel());
    job.main.add_stage_after(
        FrontEndStages::Last,
        "clear trackers",
        SystemStage::single(Container::clear_trackers),
    );
    job
}

pub(crate) fn staged_backend() -> Job {
    let mut job = Job::new();
    job.startup
        .add_stage(BackEndStartupStages::Startup, SystemStage::parallel());
    job.startup
        .add_stage(BackEndStartupStages::Setup, SystemStage::parallel());
    job.startup
        .add_stage(BackEndStartupStages::PostSetup, SystemStage::parallel());
    job.main
        .add_stage(BackendStages::Initialize, SystemStage::parallel());
    job.main.add_stage(
        BackendStages::GfxSurfaceResize,
        SystemStage::single(gfx::resize),
    );
    job.main
        .add_stage(BackendStages::Resize, SystemStage::parallel());
    job.main
        .add_stage(BackendStages::Prepare, SystemStage::parallel());
    job.main
        .add_stage(BackendStages::Last, SystemStage::parallel());
    job.main.add_stage_after(
        BackendStages::Last,
        "clear trackers",
        SystemStage::single(Container::clear_trackers),
    );
    job
}
