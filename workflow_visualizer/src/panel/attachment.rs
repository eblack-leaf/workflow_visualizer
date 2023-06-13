use bevy_ecs::prelude::IntoSystemConfig;

use crate::{Attach, spawn, SyncPoint, Visualizer};
use crate::panel::{Extraction, Panel, renderer};
use crate::panel::renderer::PanelRenderer;
use crate::panel::system::{
    calc_content_area, color_diff, content_area_diff, layer_diff, management, panel_type_diff,
    position_diff, process_extraction, pull_differences,
};

pub struct PanelAttachment;

impl Attach for PanelAttachment {
    fn attach(engen: &mut Visualizer) {
        engen.job.container.insert_resource(Extraction::new());
        engen.register_renderer::<PanelRenderer>();
        engen
            .job
            .task(Visualizer::TASK_RENDER_STARTUP)
            .add_systems((renderer::setup.in_set(SyncPoint::Preparation), ));
        engen
            .job
            .task(Visualizer::TASK_RENDER_MAIN)
            .add_systems((process_extraction.in_set(SyncPoint::Preparation), ));
        engen.job.task(Visualizer::TASK_MAIN).add_systems((
            spawn::<Panel>.in_set(SyncPoint::Spawn),
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
