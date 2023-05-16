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
    engen.job.startup.configure_sets(
        (
            SyncPoint::Initialization,
            SyncPoint::Preparation,
            SyncPoint::Resolve,
            SyncPoint::Finish,
        )
            .chain(),
    );
    engen.job.main.configure_sets(
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
    engen.job.main.add_systems((
        apply_system_buffers
            .after(SyncPoint::Spawn)
            .before(SyncPoint::Reconfigure),
        apply_system_buffers
            .after(SyncPoint::Reconfigure)
            .before(SyncPoint::ResolveVisibility),
    ));
    engen.render_initialization.configure_sets(
        (
            SyncPoint::Initialization,
            SyncPoint::Preparation,
            SyncPoint::Resolve,
            SyncPoint::Finish,
        )
            .chain(),
    );
    engen
        .render_initialization
        .add_systems((apply_system_buffers
            .after(SyncPoint::Initialization)
            .before(SyncPoint::Preparation),));
    engen.render_preparation.configure_sets(
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
