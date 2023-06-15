use bevy_ecs::prelude::IntoSystemConfig;

use crate::{Attach, Icon, spawn, SyncPoint, Visualizer};
use crate::icon::renderer::{IconRenderer, setup};
use crate::icon::system::{
    area_diff, calc_area, icon_id_diff, layer_diff, management, position_diff,
    positive_space_color_diff, read_differences,
};

pub(crate) struct IconAttachment;

impl Attach for IconAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.register_renderer::<IconRenderer>();
        visualizer
            .job
            .task(Visualizer::TASK_RENDER_STARTUP)
            .add_systems((setup.in_set(SyncPoint::Initialization), ));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            spawn::<Icon>.in_set(SyncPoint::Spawn),
            calc_area.in_set(SyncPoint::Reconfigure),
            calc_area.in_set(SyncPoint::Config),
            management.in_set(SyncPoint::Resolve),
            position_diff.in_set(SyncPoint::PushDiff),
            area_diff.in_set(SyncPoint::PushDiff),
            layer_diff.in_set(SyncPoint::PushDiff),
            positive_space_color_diff.in_set(SyncPoint::PushDiff),
            icon_id_diff.in_set(SyncPoint::PushDiff),
        ));
        visualizer
            .job
            .task(Visualizer::TASK_RENDER_MAIN)
            .add_systems((read_differences.in_set(SyncPoint::Preparation), ));
    }
}
