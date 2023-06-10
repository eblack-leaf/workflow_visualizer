use bevy_ecs::prelude::IntoSystemConfig;

use crate::line::renderer::{setup, LineRenderer};
use crate::line::system::{
    calc_section, create_render_group, push_color, push_layer, push_uniforms, scale_path,
};
use crate::path::view_changed;
use crate::{Attach, SyncPoint, Visualizer};

pub(crate) struct LineAttachment;

impl Attach for LineAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.register_renderer::<LineRenderer>();
        visualizer
            .job
            .task(Visualizer::TASK_RENDER_STARTUP)
            .add_systems((setup.in_set(SyncPoint::Initialization),));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            calc_section
                .in_set(SyncPoint::Reconfigure)
                .after(view_changed),
            scale_path.in_set(SyncPoint::Resolve),
            push_layer.in_set(SyncPoint::PushDiff),
            push_color.in_set(SyncPoint::PushDiff),
        ));
        visualizer
            .job
            .task(Visualizer::TASK_RENDER_MAIN)
            .add_systems((
                create_render_group.in_set(SyncPoint::Preparation),
                push_uniforms.in_set(SyncPoint::Resolve),
            ));
    }
}
