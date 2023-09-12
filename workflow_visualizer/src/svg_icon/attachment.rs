use crate::svg_icon::interface::{
    area_diff, color_diff, layer_diff, management, position_diff, scale_change, svg_diff,
};
use crate::svg_icon::renderer::{cull_entities, load_svg_buffers, read_differences, SvgRenderer};
use crate::{Attach, SyncPoint, Visualizer};
use bevy_ecs::prelude::IntoSystemConfigs;

pub(crate) struct SvgIconAttachment;
impl Attach for SvgIconAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.register_renderer::<SvgRenderer>();
        visualizer
            .task(Visualizer::TASK_STARTUP)
            .add_systems((load_svg_buffers.in_set(SyncPoint::PostInitialization),));
        visualizer.task(Visualizer::TASK_RENDER_MAIN).add_systems((
            cull_entities.in_set(SyncPoint::PostInitialization),
            read_differences.in_set(SyncPoint::Resolve),
        ));
        visualizer.task(Visualizer::TASK_MAIN).add_systems((
            scale_change.in_set(SyncPoint::Reconfigure),
            management.in_set(SyncPoint::Resolve),
            svg_diff.in_set(SyncPoint::PushDiff),
            position_diff.in_set(SyncPoint::PushDiff),
            area_diff.in_set(SyncPoint::PushDiff),
            layer_diff.in_set(SyncPoint::PushDiff),
            color_diff.in_set(SyncPoint::PushDiff),
        ));
    }
}
