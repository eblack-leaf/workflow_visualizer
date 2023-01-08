use bevy_ecs::prelude::Resource;
use crate::canvas::Viewport;
use crate::render::{Render, RenderPassHandle, RenderPhase};
use crate::{Attach, Engen, Task};
#[derive(Resource)]
pub struct TextRenderer {}
impl Attach for TextRenderer {
    fn attach(engen: &mut Engen) {
        todo!()
    }
}
impl Render for TextRenderer {
    fn extract(compute: &Task, render: &mut Task)
    where
        Self: Sized,
    {
        todo!()
    }

    fn phase() -> RenderPhase {
        todo!()
    }

    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        todo!()
    }
}
