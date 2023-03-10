use bevy_ecs::prelude::{
    apply_system_buffers, IntoSystemConfig, IntoSystemSetConfig, IntoSystemSetConfigs, SystemSet,
};

use crate::engen::job::JobBucket;
use crate::engen::Container;
use crate::{gfx, Job};

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum FrontEndStartupBuckets {
    Startup,
    Initialization,
    Last,
}

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum FrontEndBuckets {
    First,
    Resize,
    Prepare,
    Process,
    PostProcess,
    Spawn,
    AnimationStart,
    AnimationUpdate,
    AnimationResolved,
    CoordPrepare,
    CoordAdjust,
    ResolvePrepare,
    ResolveStart,
    Resolve,
    VisibilityPreparation,
    ResolveVisibility,
    PushDiffs,
    Finish,
    Last,
}

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum BackEndStartupBuckets {
    Startup,
    StartupClear,
    Prepare,
    Resolve,
}

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum BackendBuckets {
    Initialize,
    GfxSurfaceResize,
    Resize,
    Prepare,
    Last,
}

pub(crate) fn staged_frontend() -> Job {
    let mut job = Job::new();
    job.startup.configure_sets(
        (
            FrontEndStartupBuckets::Startup,
            FrontEndStartupBuckets::Initialization,
            FrontEndStartupBuckets::Last,
        )
            .chain(),
    );
    job.main.configure_sets(
        (
            FrontEndBuckets::First.after(JobBucket::Idle),
            FrontEndBuckets::Resize,
            FrontEndBuckets::Prepare,
            FrontEndBuckets::Process,
            FrontEndBuckets::PostProcess,
            FrontEndBuckets::Spawn,
            FrontEndBuckets::AnimationStart,
            FrontEndBuckets::AnimationUpdate,
            FrontEndBuckets::AnimationResolved,
        )
            .chain(),
    );
    job.main.configure_sets(
        (
            FrontEndBuckets::CoordPrepare,
            FrontEndBuckets::CoordAdjust,
            FrontEndBuckets::ResolvePrepare,
            FrontEndBuckets::ResolveStart,
            FrontEndBuckets::Resolve.before(FrontEndBuckets::VisibilityPreparation),
            FrontEndBuckets::VisibilityPreparation.before(FrontEndBuckets::ResolveVisibility),
            FrontEndBuckets::ResolveVisibility.before(FrontEndBuckets::PushDiffs),
            FrontEndBuckets::PushDiffs.before(FrontEndBuckets::Finish),
            FrontEndBuckets::Finish.before(FrontEndBuckets::Last),
            FrontEndBuckets::Last,
        )
            .chain(),
    );
    job.main
        .add_systems((Container::clear_trackers.in_set(FrontEndBuckets::Last),));
    job
}

pub(crate) fn staged_backend() -> Job {
    let mut job = Job::new();
    job.startup.configure_sets(
        (
            BackEndStartupBuckets::Startup.before(BackEndStartupBuckets::Prepare),
            BackEndStartupBuckets::StartupClear,
            BackEndStartupBuckets::Prepare.before(BackEndStartupBuckets::Resolve),
            BackEndStartupBuckets::Resolve,
        )
            .chain(),
    );
    job.startup
        .add_system(apply_system_buffers.in_set(BackEndStartupBuckets::StartupClear));
    job.main.configure_sets(
        (
            BackendBuckets::Initialize
                .after(JobBucket::Idle)
                .before(BackendBuckets::GfxSurfaceResize),
            BackendBuckets::GfxSurfaceResize.before(BackendBuckets::Resize),
            BackendBuckets::Resize.before(BackendBuckets::Prepare),
            BackendBuckets::Prepare.before(BackendBuckets::Last),
            BackendBuckets::Last,
        )
            .chain(),
    );
    job.main.add_systems((
        gfx::resize.in_set(BackendBuckets::GfxSurfaceResize),
        Container::clear_trackers.in_set(BackendBuckets::Last),
    ));
    job
}
