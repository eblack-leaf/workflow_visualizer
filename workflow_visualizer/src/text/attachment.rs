use bevy_ecs::prelude::IntoSystemConfig;

use crate::{Attach, spawn, SyncPoint, Visualizer};
use crate::text::component::Text;
use crate::text::renderer;
use crate::text::renderer::TextRenderer;
use crate::text::system::{
    color_diff, create_render_groups, filter, layer_diff, letter_differential, manage, place,
    position_diff, pull_differences, render_group_differences, resolve_draw_section_on_resize,
    scale_change, setup, visible_section_diff,
};

/// Attachment for enabling the `TextRenderer`
pub struct TextAttachment;
impl Attach for TextAttachment {
    fn attach(engen: &mut Visualizer) {
        engen.register_renderer::<TextRenderer>();
        engen
            .job
            .task(Visualizer::TASK_STARTUP)
            .add_systems((setup.in_set(SyncPoint::Initialization),));
        engen.job.task(Visualizer::TASK_MAIN).add_systems((
            spawn::<Text>.in_set(SyncPoint::Spawn),
            scale_change.in_set(SyncPoint::Reconfigure),
            place.in_set(SyncPoint::Resolve),
            manage.in_set(SyncPoint::Resolve),
            filter.in_set(SyncPoint::Resolve).after(place),
            letter_differential.in_set(SyncPoint::Resolve).after(filter),
            position_diff.in_set(SyncPoint::PushDiff),
            visible_section_diff.in_set(SyncPoint::PushDiff),
            color_diff.in_set(SyncPoint::PushDiff),
            layer_diff.in_set(SyncPoint::PushDiff),
            pull_differences.in_set(SyncPoint::Finish),
        ));
        engen
            .job
            .task(Visualizer::TASK_RENDER_STARTUP)
            .add_systems((renderer::setup.in_set(SyncPoint::Preparation),));
        engen.job.task(Visualizer::TASK_RENDER_MAIN).add_systems((
            create_render_groups.in_set(SyncPoint::Preparation),
            render_group_differences.in_set(SyncPoint::Resolve),
            resolve_draw_section_on_resize.in_set(SyncPoint::Resolve),
        ));
    }
}
