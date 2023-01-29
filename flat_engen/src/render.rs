use bevy_ecs::prelude::Resource;

use crate::canvas::Canvas;
use crate::viewport::Viewport;
use crate::{Engen, Task, Theme};

pub enum RenderPhase {
    Opaque,
    Alpha,
}

pub(crate) fn invoke_render<'a, RenderAttachment: Render + Resource>(
    backend: &'a Task,
    render_pass_handle: &mut RenderPassHandle<'a>,
) {
    let viewport = backend
        .container
        .get_resource::<Viewport>()
        .expect("no viewport attached");
    backend
        .container
        .get_resource::<RenderAttachment>()
        .expect("no render attachment")
        .render(render_pass_handle, viewport);
}

pub struct RenderPassHandle<'a>(pub wgpu::RenderPass<'a>);

pub(crate) type RenderFns = Vec<Box<for<'a> fn(&'a Task, &mut RenderPassHandle<'a>)>>;

pub trait Render {
    fn phase() -> RenderPhase;
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport);
}

pub(crate) fn extract(engen: &mut Engen) {
    for invoke in engen.extract_fns.iter_mut() {
        invoke(&mut engen.frontend, &mut engen.backend);
    }
}

pub(crate) fn render(engen: &mut Engen) {
    let canvas = engen
        .backend
        .container
        .get_resource::<Canvas>()
        .expect("no canvas attached");
    let theme = engen
        .backend
        .container
        .get_resource::<Theme>()
        .expect("no theme attached");
    let viewport = engen
        .backend
        .container
        .get_resource::<Viewport>()
        .expect("no viewport attached");
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
            let depth_texture_view = viewport
                .depth_texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut render_pass_handle = RenderPassHandle(command_encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
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
                            load: wgpu::LoadOp::Clear(viewport.cpu.far_layer()),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                },
            ));
            for invoke in engen.render_fns.0.iter_mut() {
                invoke(&engen.backend, &mut render_pass_handle);
            }
            for invoke in engen.render_fns.1.iter_mut() {
                invoke(&engen.backend, &mut render_pass_handle);
            }
        }
        canvas
            .queue
            .submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
    }
}