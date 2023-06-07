use bevy_ecs::prelude::{IntoSystemConfig, IntoSystemSetConfigs, SystemSet};
use bevy_ecs::schedule::apply_system_buffers;

use crate::visualizer::Visualizer;
use crate::JobSyncPoint;

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum SyncPoint {
    Event,
    Initialization,
    Config,
    PreProcessVisibility,
    Preparation,
    Spawn,
    Reconfigure,
    PostProcessVisibility,
    Resolve,
    PushDiff,
    Finish,
}
#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum UserSpaceSyncPoint {
    Initialization,
    Process,
    Resolve,
}
pub(crate) fn set_sync_points(visualizer: &mut Visualizer) {
    visualizer
        .job
        .task(Visualizer::TASK_STARTUP)
        .configure_sets(
            (
                SyncPoint::Initialization,
                UserSpaceSyncPoint::Initialization,
                SyncPoint::Preparation,
                SyncPoint::Resolve,
                UserSpaceSyncPoint::Resolve,
                SyncPoint::Finish,
            )
                .chain(),
        );
    visualizer.job.task(Visualizer::TASK_MAIN).configure_sets(
        (
            JobSyncPoint::Idle,
            SyncPoint::Event,
            SyncPoint::Initialization,
            UserSpaceSyncPoint::Initialization,
            SyncPoint::Config,
            SyncPoint::PreProcessVisibility,
            SyncPoint::Preparation,
            UserSpaceSyncPoint::Process,
            SyncPoint::Spawn,
            SyncPoint::Reconfigure,
            SyncPoint::PostProcessVisibility,
            SyncPoint::Resolve,
            UserSpaceSyncPoint::Resolve,
            SyncPoint::PushDiff,
            SyncPoint::Finish,
        )
            .chain(),
    );
    visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
        apply_system_buffers
            .after(SyncPoint::Spawn)
            .before(SyncPoint::Reconfigure),
        apply_system_buffers
            .after(SyncPoint::Reconfigure)
            .before(SyncPoint::PostProcessVisibility),
    ));
    visualizer
        .job
        .task(Visualizer::TASK_RENDER_STARTUP)
        .configure_sets(
            (
                SyncPoint::Initialization,
                UserSpaceSyncPoint::Initialization,
                SyncPoint::Preparation,
                SyncPoint::Resolve,
                UserSpaceSyncPoint::Resolve,
                SyncPoint::Finish,
            )
                .chain(),
        );
    visualizer
        .job
        .task(Visualizer::TASK_RENDER_STARTUP)
        .add_systems((apply_system_buffers
            .after(SyncPoint::Initialization)
            .before(SyncPoint::Preparation),));
    visualizer
        .job
        .task(Visualizer::TASK_RENDER_MAIN)
        .configure_sets(
            (
                JobSyncPoint::Idle,
                SyncPoint::Initialization,
                UserSpaceSyncPoint::Initialization,
                SyncPoint::Preparation,
                SyncPoint::Resolve,
                UserSpaceSyncPoint::Resolve,
                SyncPoint::Finish,
            )
                .chain(),
        );
}
