use bevy_ecs::prelude::IntoSystemConfigs;

use crate::panel::renderer::PanelRenderer;
use crate::panel::system::{
    calc_content_area, color_diff, content_area_diff, layer_diff, management, panel_type_diff,
    position_diff, process_extraction, pull_differences,
};
use crate::panel::Extraction;
use crate::{Attach, SyncPoint, Visualizer};

pub struct PanelAttachment;

impl Attach for PanelAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.job.container.insert_resource(Extraction::new());
        visualizer.register_renderer::<PanelRenderer>();
        visualizer
            .job
            .task(Visualizer::TASK_RENDER_MAIN)
            .add_systems((process_extraction.in_set(SyncPoint::Preparation),));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            calc_content_area.in_set(SyncPoint::Reconfigure),
            management.in_set(SyncPoint::Resolve),
            panel_type_diff.in_set(SyncPoint::PushDiff),
            position_diff.in_set(SyncPoint::PushDiff),
            content_area_diff.in_set(SyncPoint::PushDiff),
            layer_diff.in_set(SyncPoint::PushDiff),
            color_diff.in_set(SyncPoint::PushDiff),
            pull_differences.in_set(SyncPoint::Finish),
        ));
    }
}
