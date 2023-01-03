use crate::canvas::{Canvas, Viewport};
use crate::theme::Theme;
use crate::{Launcher, Task};
use std::collections::HashMap;
use wgpu::RenderPass;
#[derive(Eq, Hash, PartialEq)]
pub struct Id(pub &'static str);
pub enum RenderPhase {
    Opaque,
    Alpha,
}
pub trait Render {
    fn phase() -> RenderPhase
    where
        Self: Sized;
    fn id() -> Id
    where
        Self: Sized;
    fn extract(&mut self, compute: &mut Task);
    fn prepare(&mut self, canvas: &Canvas);
    fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>, viewport: &'a Viewport);
    fn instrument(&self, app: &mut Task);
    fn renderer(canvas: &Canvas) -> Self
    where
        Self: Sized;
}
pub type Renderer = Box<dyn Render>;
pub type RendererStorage = HashMap<Id, Renderer>;
pub struct Renderers {
    pub opaque: RendererStorage,
    pub alpha: RendererStorage,
}
impl Renderers {
    pub fn new() -> Self {
        Self {
            opaque: RendererStorage::new(),
            alpha: RendererStorage::new(),
        }
    }
}
pub(crate) fn extract(renderers: &mut Renderers, compute: &mut Task) {
    for (_id, renderer) in renderers.opaque.iter_mut() {
        renderer.extract(compute);
    }
    for (_id, renderer) in renderers.alpha.iter_mut() {
        renderer.extract(compute);
    }
}
pub(crate) fn prepare(renderers: &mut Renderers, canvas: &Canvas) {
    for (_id, renderer) in renderers.opaque.iter_mut() {
        renderer.prepare(canvas);
    }
    for (_id, renderer) in renderers.alpha.iter_mut() {
        renderer.prepare(canvas);
    }
}
pub(crate) fn render(renderers: &mut Renderers, canvas: &Canvas, theme: &Theme) {
    if let Some(surface_texture) = canvas.surface_texture() {
        let mut command_encoder =
            canvas
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("command encoder"),
                });
        let surface_texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        {
            let depth_texture_view = canvas
                .viewport
                .depth_texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(theme.background.into()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(canvas.viewport.cpu.far_layer()),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            for (_id, renderer) in renderers.opaque.iter() {
                renderer.render(&mut render_pass, &canvas.viewport);
            }
            for (_id, renderer) in renderers.alpha.iter() {
                renderer.render(&mut render_pass, &canvas.viewport);
            }
        }
        canvas
            .queue
            .submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
    }
}
