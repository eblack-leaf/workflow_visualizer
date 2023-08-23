use bevy_ecs::prelude::IntoSystemConfigs;

use crate::images::interface::{
    area_diff, aspect_ratio_aligned_dimension, extract, fade_diff, layer_diff, management,
    name_diff, pos_diff, Extraction,
};
use crate::images::render_group::read_extraction;
use crate::images::renderer::{load_images, ImageOrientations, ImageRenderer};
use crate::{Attach, SyncPoint, Visualizer};

pub(crate) struct ImageAttachment;

impl Attach for ImageAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.register_renderer::<ImageRenderer>();
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
            .task(Visualizer::TASK_RENDER_STARTUP)
            .add_systems((load_images.in_set(SyncPoint::Resolve),));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            aspect_ratio_aligned_dimension.in_set(SyncPoint::Reconfigure),
            management.in_set(SyncPoint::Resolve),
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
