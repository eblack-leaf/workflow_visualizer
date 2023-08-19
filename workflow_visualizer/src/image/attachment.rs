use crate::image::renderer::{load_images, setup_renderer};
use crate::{Attach, SyncPoint, Visualizer};
use bevy_ecs::prelude::IntoSystemConfigs;

pub(crate) struct ImageAttachment;

impl Attach for ImageAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer
            .job
            .task(Visualizer::TASK_RENDER_STARTUP)
            .add_systems((
                setup_renderer.in_set(SyncPoint::PostInitialization),
                load_images.in_set(SyncPoint::Resolve),
            ));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems(());
    }
}
