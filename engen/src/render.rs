use crate::canvas::{Canvas, Viewport};
use crate::launcher::Renderers;
use crate::theme::Theme;
use crate::{App, Launcher};
use bevy_ecs::prelude::{NonSendMut, Res};
use std::collections::HashMap;
use wgpu::RenderPass;
#[derive(Eq, Hash, PartialEq)]
pub struct Id(pub &'static str);
pub trait Render {
    fn id() -> Id
    where
        Self: Sized;
    fn extract(&mut self, compute: &mut App);
    fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>, viewport: &'a Viewport);
    fn attach(self, launcher: &mut Launcher);
}
pub fn render(canvas: Res<Canvas>, theme: Res<Theme>, renderers: NonSendMut<Renderers>) {
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
            for (_id, renderer) in renderers.renderers.iter() {
                renderer.render(&mut render_pass, &canvas.viewport);
            }
        }
        canvas
            .queue
            .submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
    }
}
