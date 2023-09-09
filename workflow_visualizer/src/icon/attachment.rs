use bevy_ecs::prelude::{Entity, IntoSystemConfigs};

use crate::icon::bitmap::{cleanup_requests, IconBitmapRequestManager};
use crate::icon::renderer::IconRenderer;
use crate::icon::system::{
    area_diff, calc_area, icon_id_diff, layer_diff, management, position_diff,
    positive_space_color_diff, read_differences,
};
use crate::{Attach, IconBitmapRequest, SyncPoint, Visualizer};

pub(crate) struct IconAttachment;

impl Attach for IconAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            calc_area.in_set(SyncPoint::Reconfigure),
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
            .add_systems((read_differences.in_set(SyncPoint::Preparation),));
    }
}
pub(crate) struct IconRendererAttachment;
impl Attach for IconRendererAttachment {
    fn attach(visualizer: &mut Visualizer) {
        let mut manager = IconBitmapRequestManager::default();
        for (_entity, request) in visualizer
            .job
            .container
            .query::<(Entity, &IconBitmapRequest)>()
            .iter(&visualizer.job.container)
        {
            manager.add(request.clone());
        }
        visualizer.job.container.insert_resource(manager);
        visualizer.register_renderer::<IconRenderer>();
        visualizer
            .job
            .container
            .remove_resource::<IconBitmapRequestManager>();
        visualizer
            .job
            .task(Visualizer::TASK_STARTUP)
            .add_systems((cleanup_requests.in_set(SyncPoint::PostInitialization),));
    }
}
