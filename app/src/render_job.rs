use crate::job::{Job, Workload};
use crate::text::rasterize;
use crate::theme::Theme;
use crate::window::Resize;
use crate::{depth_texture, renderer, text, viewport, window, Signal};
use bevy_ecs::prelude::SystemStage;
fn startup() -> Workload {
    let mut workload = Workload::default();
    workload.add_stage(
        "setup",
        SystemStage::parallel()
            .with_system(viewport::setup)
            .with_system(depth_texture::setup),
    );
    workload.add_stage(
        "text",
        SystemStage::parallel()
            .with_system(rasterize::rasterization::setup)
            .with_system(text::attribute::setup)
            .with_system(text::font::setup)
            .with_system(text::vertex_buffer::setup),
    );
    workload.add_stage(
        "text2",
        SystemStage::parallel().with_system(text::rasterize::binding::setup),
    );
    workload.add_stage(
        "text3",
        SystemStage::parallel().with_system(text::pipeline::setup),
    );
    workload
}
fn exec() -> Workload {
    let mut workload = Workload::default();
    workload.add_stage("window_resize", SystemStage::single(window::resize));
    workload.add_stage("render", SystemStage::single(renderer::render));
    workload
}
pub fn render_job() -> Job {
    {
        let mut job = Job::new();
        job.container.insert_resource(Theme::default());
        job.container.insert_resource(Signal::<Resize>::new(None));
        job.startup = startup();
        job.exec = exec();
        job
    }
}
