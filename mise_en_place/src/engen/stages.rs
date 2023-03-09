use bevy_ecs::prelude::{apply_system_buffers, IntoSystemConfig, IntoSystemSetConfig, SystemSet};

use crate::{gfx, Job};
use crate::engen::Container;
use crate::engen::job::JobBucket;

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
    job.startup.configure_sets((
        FrontEndStartupBuckets::Startup,
        FrontEndStartupBuckets::Initialization.after(FrontEndStartupBuckets::Startup),
        FrontEndStartupBuckets::Last.after(FrontEndStartupBuckets::Initialization),
    ));
    job.startup.add_systems(
        (
            apply_system_buffers.before(FrontEndStartupBuckets::Initialization),
            apply_system_buffers.before(FrontEndStartupBuckets::Last),
            apply_system_buffers.after(FrontEndStartupBuckets::Last),
        )
    );
    job.main.configure_sets((
        FrontEndBuckets::First.after(JobBucket::Idle),
        FrontEndBuckets::Resize.after(FrontEndBuckets::First),
        FrontEndBuckets::Prepare.after(FrontEndBuckets::Resize),
        FrontEndBuckets::Process.after(FrontEndBuckets::Prepare),
        FrontEndBuckets::PostProcess.after(FrontEndBuckets::Process),
        FrontEndBuckets::Spawn.after(FrontEndBuckets::PostProcess),
        FrontEndBuckets::AnimationStart.after(FrontEndBuckets::Spawn),
        FrontEndBuckets::AnimationUpdate.after(FrontEndBuckets::AnimationStart),
        FrontEndBuckets::AnimationResolved.after(FrontEndBuckets::AnimationUpdate),
    ));
    job.main.configure_sets((
        FrontEndBuckets::CoordPrepare.after(FrontEndBuckets::AnimationResolved),
        FrontEndBuckets::CoordAdjust.after(FrontEndBuckets::CoordPrepare),
        FrontEndBuckets::ResolvePrepare.after(FrontEndBuckets::CoordAdjust),
        FrontEndBuckets::ResolveStart.after(FrontEndBuckets::ResolvePrepare),
        FrontEndBuckets::Resolve.after(FrontEndBuckets::ResolveStart),
        FrontEndBuckets::VisibilityPreparation.after(FrontEndBuckets::Resolve),
        FrontEndBuckets::ResolveVisibility.after(FrontEndBuckets::VisibilityPreparation),
        FrontEndBuckets::PushDiffs.after(FrontEndBuckets::ResolveVisibility),
        FrontEndBuckets::Finish.after(FrontEndBuckets::PushDiffs),
        FrontEndBuckets::Last.after(FrontEndBuckets::Finish),
    ));
    job.main.add_systems((
        apply_system_buffers.before(FrontEndBuckets::First),
        apply_system_buffers.before(FrontEndBuckets::Resize),
        apply_system_buffers.before(FrontEndBuckets::Prepare),
        apply_system_buffers.before(FrontEndBuckets::Process),
        apply_system_buffers.before(FrontEndBuckets::PostProcess),
        apply_system_buffers.before(FrontEndBuckets::Spawn),
        apply_system_buffers.before(FrontEndBuckets::AnimationStart),
        apply_system_buffers.before(FrontEndBuckets::AnimationUpdate),
        apply_system_buffers.before(FrontEndBuckets::AnimationResolved),
    ));
    job.main.add_systems((
        apply_system_buffers.before(FrontEndBuckets::CoordPrepare),
        apply_system_buffers.before(FrontEndBuckets::CoordAdjust),
        apply_system_buffers.before(FrontEndBuckets::ResolvePrepare),
        apply_system_buffers.before(FrontEndBuckets::ResolveStart),
        apply_system_buffers.before(FrontEndBuckets::Resolve),
        apply_system_buffers.before(FrontEndBuckets::VisibilityPreparation),
        apply_system_buffers.before(FrontEndBuckets::ResolveVisibility),
        apply_system_buffers.before(FrontEndBuckets::PushDiffs),
        apply_system_buffers.before(FrontEndBuckets::Finish),
        apply_system_buffers.before(FrontEndBuckets::Last),
        Container::clear_trackers.in_set(FrontEndBuckets::Last),
    ));
    job
}

pub(crate) fn staged_backend() -> Job {
    let mut job = Job::new();
    job.startup.configure_sets((
        BackEndStartupBuckets::Startup,
        BackEndStartupBuckets::Prepare.after(BackEndStartupBuckets::Startup),
        BackEndStartupBuckets::Resolve.after(BackEndStartupBuckets::Prepare),
    ));
    job.startup
        .add_systems((
            apply_system_buffers.after(BackEndStartupBuckets::Startup),
            apply_system_buffers.before(BackEndStartupBuckets::Prepare),
            apply_system_buffers.before(BackEndStartupBuckets::Resolve),
        ));
    job.main.configure_sets((
        BackendBuckets::Initialize.after(JobBucket::Idle),
        BackendBuckets::GfxSurfaceResize.after(BackendBuckets::Initialize),
        BackendBuckets::Resize.after(BackendBuckets::GfxSurfaceResize),
        BackendBuckets::Prepare.after(BackendBuckets::Resize),
        BackendBuckets::Last.after(BackendBuckets::Prepare),
    ));
    job.main.add_systems((
        apply_system_buffers.before(BackendBuckets::Initialize),
        apply_system_buffers.before(BackendBuckets::GfxSurfaceResize),
        apply_system_buffers.before(BackendBuckets::Resize),
        apply_system_buffers.before(BackendBuckets::Prepare),
        apply_system_buffers.before(BackendBuckets::Last),
        gfx::resize.in_set(BackendBuckets::GfxSurfaceResize),
        Container::clear_trackers.in_set(BackendBuckets::Last),
    ));
    job
}
