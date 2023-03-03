use bevy_ecs::prelude::{StageLabel, SystemStage};

use crate::engen::Container;
use crate::{gfx, Job};

#[derive(StageLabel)]
pub enum FrontEndStartupStages {
    Startup,
    Initialization,
    Last,
}

#[derive(StageLabel)]
pub enum FrontEndStages {
    First,
    Resize,
    PreProcess,
    PreProcessResolve,
    ProcessAndSpawn,
    AnimationStart,
    AnimationUpdate,
    AnimationResolved,
    CoordAdjust,
    VisibilityPreparation,
    ResolveVisibility,
    Resolve,
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
    job.startup
        .add_stage(FrontEndStartupStages::Last, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::First, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::Resize, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::PreProcess, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::PreProcessResolve, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::ProcessAndSpawn, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::AnimationStart, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::AnimationUpdate, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::AnimationResolved, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::CoordAdjust, SystemStage::parallel());
    job.main.add_stage(
        FrontEndStages::VisibilityPreparation,
        SystemStage::parallel(),
    );
    job.main
        .add_stage(FrontEndStages::ResolveVisibility, SystemStage::parallel());
    job.main
        .add_stage(FrontEndStages::Resolve, SystemStage::parallel());
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
