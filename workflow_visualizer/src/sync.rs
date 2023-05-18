use bevy_ecs::prelude::{IntoSystemConfig, IntoSystemSetConfigs, SystemSet};
use bevy_ecs::schedule::apply_system_buffers;

use crate::{JobSyncPoint, Visualizer};

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum SyncPoint {
    Event,
    Initialization,
    Config,
    Preparation,
    Spawn,
    Reconfigure,
    ResolveVisibility,
    Resolve,
    PushDiff,
    Finish,
}
#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum UserSpaceSyncPoint {
    Process,
    Resolve,
}
pub(crate) fn set_sync_points(engen: &mut Visualizer) {
    engen.job.task(Visualizer::TASK_STARTUP).configure_sets(
        (
            SyncPoint::Initialization,
            SyncPoint::Preparation,
            SyncPoint::Resolve,
            SyncPoint::Finish,
        )
            .chain(),
    );
    engen.job.task(Visualizer::TASK_MAIN).configure_sets(
        (
            JobSyncPoint::Idle,
            SyncPoint::Event,
            SyncPoint::Initialization,
            SyncPoint::Preparation,
            UserSpaceSyncPoint::Process,
            SyncPoint::Spawn,
            SyncPoint::Reconfigure,
            SyncPoint::ResolveVisibility,
            SyncPoint::Resolve,
            UserSpaceSyncPoint::Resolve,
            SyncPoint::PushDiff,
            SyncPoint::Finish,
        )
            .chain(),
    );
    engen.job.task(Visualizer::TASK_MAIN).add_systems((
        apply_system_buffers
            .after(SyncPoint::Spawn)
            .before(SyncPoint::Reconfigure),
        apply_system_buffers
            .after(SyncPoint::Reconfigure)
            .before(SyncPoint::ResolveVisibility),
    ));
    engen
        .job
        .task(Visualizer::TASK_RENDER_STARTUP)
        .configure_sets(
            (
                SyncPoint::Initialization,
                SyncPoint::Preparation,
                SyncPoint::Resolve,
                SyncPoint::Finish,
            )
                .chain(),
        );
    engen
        .job
        .task(Visualizer::TASK_RENDER_STARTUP)
        .add_systems((apply_system_buffers
            .after(SyncPoint::Initialization)
            .before(SyncPoint::Preparation),));
    engen.job.task(Visualizer::TASK_RENDER_MAIN).configure_sets(
        (
            JobSyncPoint::Idle,
            SyncPoint::Initialization,
            SyncPoint::Preparation,
            SyncPoint::Resolve,
            SyncPoint::Finish,
        )
            .chain(),
    );
}
