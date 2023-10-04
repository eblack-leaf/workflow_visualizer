use crate::button::system;
use crate::{Attach, SyncPoint, Visualizer};
use bevy_ecs::prelude::IntoSystemConfigs;

pub(crate) struct ButtonAttachment;

impl Attach for ButtonAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            system::spawn.in_set(SyncPoint::Spawn),
            system::place.in_set(SyncPoint::Spawn).after(system::spawn),
            system::scale_change
                .in_set(SyncPoint::SecondaryEffects)
                .after(crate::snap_grid::reapply),
            system::border_change.in_set(SyncPoint::Reconfigure),
            system::color_invert.in_set(SyncPoint::Reconfigure),
            system::secondary_despawn
                .in_set(SyncPoint::PostProcessPreparation)
                .before(crate::despawn),
            system::color_forward.in_set(SyncPoint::SecondaryEffects),
            system::forward_disable.in_set(SyncPoint::SecondaryEffects),
            system::remove_disabled.in_set(SyncPoint::SecondaryEffects),
        ));
    }
}
