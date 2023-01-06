use crate::canvas::Viewport;
use crate::task::Task;
use wgpu::RenderPass;

pub trait Extract {
    fn extract(&mut self, compute: &Task);
}
pub trait Render {
    fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>, viewport: &'a Viewport);
}
