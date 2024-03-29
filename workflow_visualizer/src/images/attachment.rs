use bevy_ecs::prelude::IntoSystemConfigs;

use crate::images::interface::{
    apply_aspect_animations, area_diff, aspect_ratio_aligned_dimension, extract, fade_diff,
    icon_color_diff, layer_diff, management, name_diff, pos_diff, set_from_scale, Extraction,
};
use crate::images::render_group::read_extraction;
use crate::images::renderer::{
    apply_animations, load_images, ImageLoaded, ImageOrientations, ImageRenderer,
};
use crate::{AspectRatioAlignedDimension, Attach, ImageFade, ImageSizes, SyncPoint, Visualizer};

pub(crate) struct ImageAttachment;

impl Attach for ImageAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.register_renderer::<ImageRenderer>();
        visualizer.register_animation::<ImageFade>();
        visualizer.register_animation::<AspectRatioAlignedDimension>();
        visualizer
            .job
            .container
            .insert_resource(Extraction::default());
        visualizer
            .job
            .container
            .insert_resource(ImageOrientations::default());
        visualizer
            .job
            .container
            .insert_resource(ImageSizes::default());
        visualizer.add_event::<ImageLoaded>();
        visualizer
            .job
            .task(Visualizer::TASK_STARTUP)
            .add_systems((load_images.in_set(SyncPoint::Initialization),));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            // load_images.in_set(SyncPoint::Initialization),
            apply_animations.in_set(SyncPoint::Animation),
            apply_aspect_animations.in_set(SyncPoint::Animation),
            set_from_scale.in_set(SyncPoint::Reconfigure),
            aspect_ratio_aligned_dimension.in_set(SyncPoint::Reconfigure),
            management.in_set(SyncPoint::Resolve),
            icon_color_diff.in_set(SyncPoint::PushDiff),
            pos_diff.in_set(SyncPoint::PushDiff),
            area_diff.in_set(SyncPoint::PushDiff),
            layer_diff.in_set(SyncPoint::PushDiff),
            name_diff.in_set(SyncPoint::PushDiff),
            fade_diff.in_set(SyncPoint::PushDiff),
            extract.in_set(SyncPoint::Finish),
        ));
        visualizer
            .job
            .task(Visualizer::TASK_RENDER_MAIN)
            .add_systems((read_extraction.in_set(SyncPoint::Preparation),));
    }
}
