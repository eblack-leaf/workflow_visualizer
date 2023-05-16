use crate::{Job, Viewport};
use bevy_ecs::prelude::Resource;

pub enum RenderPhase {
    Opaque,
    Alpha(u32),
}
pub(crate) fn invoke_render<'a, Renderer: Render + Resource>(
    backend: &'a Job,
    render_pass_handle: &mut RenderPassHandle<'a>,
) {
    let viewport = backend
        .container
        .get_resource::<Viewport>()
        .expect("no viewport attached");
    backend
        .container
        .get_resource::<Renderer>()
        .expect("no render attachment")
        .render(render_pass_handle, viewport);
}

pub struct RenderPassHandle<'a>(pub wgpu::RenderPass<'a>);

pub(crate) type RenderFns = Vec<Box<for<'a> fn(&'a Job, &mut RenderPassHandle<'a>)>>;

pub trait Render {
    fn phase() -> RenderPhase;
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport);
}
