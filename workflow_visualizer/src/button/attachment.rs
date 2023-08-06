use bevy_ecs::prelude::IntoSystemConfig;

use crate::{Attach, SyncPoint, Visualizer};
use crate::button::system;

pub(crate) struct ButtonAttachment;

impl Attach for ButtonAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            system::spawn.in_set(SyncPoint::Spawn),
            system::placement.in_set(SyncPoint::SecondaryEffects),
            system::color_invert.in_set(SyncPoint::Reconfigure),
            system::despawn.in_set(SyncPoint::Reconfigure),
            system::color_forward.in_set(SyncPoint::SecondaryEffects),
            system::forward_disable.in_set(SyncPoint::SecondaryEffects),
            system::remove_disabled.in_set(SyncPoint::SecondaryEffects),
        ));
    }
}
