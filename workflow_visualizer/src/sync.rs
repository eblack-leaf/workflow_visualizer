use bevy_ecs::prelude::{apply_deferred, IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};

use crate::visualizer::Visualizer;
use crate::JobSyncPoint;

/// Synchronization Points for bucketing systems in a task
#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum SyncPoint {
    Event,
    Initialization,
    PostInitialization,
    Animation,
    Config,
    PreProcessVisibility,
    Preparation,
    Process,
    Spawn,
    PostProcessPreparation,
    SecondaryEffects,
    Reconfigure,
    PostProcessVisibility,
    Resolve,
    PostResolve,
    PushDiff,
    Finish,
}
pub(crate) fn set_sync_points(visualizer: &mut Visualizer) {
    visualizer.task(Visualizer::TASK_STARTUP).configure_sets(
        (
            SyncPoint::Event,
            SyncPoint::Initialization,
            SyncPoint::PostInitialization,
            SyncPoint::Preparation,
            SyncPoint::Resolve,
            SyncPoint::PostResolve,
            SyncPoint::Finish,
        )
            .chain(),
    );
    visualizer.task(Visualizer::TASK_STARTUP).add_systems((
        apply_deferred
            .after(SyncPoint::Initialization)
            .before(SyncPoint::PostInitialization),
        apply_deferred
            .after(SyncPoint::PostInitialization)
            .before(SyncPoint::Preparation),
        apply_deferred
            .after(SyncPoint::Preparation)
            .before(SyncPoint::Resolve),
        apply_deferred
            .after(SyncPoint::Resolve)
            .before(SyncPoint::PostResolve),
    ));
    visualizer.task(Visualizer::TASK_MAIN).configure_sets(
        (
            JobSyncPoint::Idle,
            SyncPoint::Event,
            SyncPoint::Initialization,
            SyncPoint::PostInitialization,
            SyncPoint::Animation,
            SyncPoint::Config,
            SyncPoint::PreProcessVisibility,
            SyncPoint::Preparation,
            SyncPoint::Process,
            SyncPoint::Spawn,
            SyncPoint::PostProcessPreparation,
            SyncPoint::SecondaryEffects,
            SyncPoint::Reconfigure,
            SyncPoint::PostProcessVisibility,
        )
            .chain(),
    );
    visualizer.job.task(Visualizer::TASK_MAIN).configure_sets(
        (
            SyncPoint::Resolve,
            SyncPoint::PostResolve,
            SyncPoint::PushDiff,
            SyncPoint::Finish,
        )
            .chain()
            .after(SyncPoint::PostProcessVisibility),
    );
    visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
        apply_deferred
            .after(SyncPoint::Spawn)
            .before(SyncPoint::PostProcessPreparation),
        apply_deferred
            .after(SyncPoint::PostProcessPreparation)
            .before(SyncPoint::SecondaryEffects),
        apply_deferred
            .after(SyncPoint::Reconfigure)
            .before(SyncPoint::PostProcessVisibility),
        apply_deferred
            .after(SyncPoint::Process)
            .before(SyncPoint::Spawn),
    ));
    visualizer
        .job
        .task(Visualizer::TASK_RENDER_MAIN)
        .configure_sets(
            (
                JobSyncPoint::Idle,
                SyncPoint::Initialization,
                SyncPoint::PostInitialization,
                SyncPoint::Preparation,
                SyncPoint::Resolve,
                SyncPoint::PostResolve,
                SyncPoint::Finish,
            )
                .chain(),
        );
}
