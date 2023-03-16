use crate::{Engen, JobSyncPoint};
use bevy_ecs::prelude::{IntoSystemConfig, IntoSystemSetConfigs, SystemSet};
use bevy_ecs::schedule::apply_system_buffers;

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
pub(crate) fn set_sync_points(engen: &mut Engen) {
    engen.frontend.startup.configure_sets(
        (
            SyncPoint::Initialization,
            SyncPoint::Preparation,
            SyncPoint::Resolve,
            SyncPoint::Finish,
        )
            .chain(),
    );
    engen.frontend.main.configure_sets(
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
    engen.frontend.main.add_system(
        apply_system_buffers
            .after(SyncPoint::Spawn)
            .before(SyncPoint::Reconfigure),
    );
    engen.frontend.main.add_system(
        apply_system_buffers
            .after(SyncPoint::Reconfigure)
            .before(SyncPoint::ResolveVisibility),
    );
    engen.backend.startup.configure_sets(
        (
            SyncPoint::Initialization,
            SyncPoint::Preparation,
            SyncPoint::Resolve,
            SyncPoint::Finish,
        )
            .chain(),
    );
    engen.backend.startup.add_system(
        apply_system_buffers
            .after(SyncPoint::Initialization)
            .before(SyncPoint::Preparation),
    );
    engen.backend.main.configure_sets(
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
