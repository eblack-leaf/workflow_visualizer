use bevy_ecs::prelude::Resource;

use crate::gfx::{GfxSurface, GfxSurfaceConfiguration};
use crate::viewport::Viewport;
use crate::{Engen, Job, Theme};

pub enum RenderPhase {
    Opaque,
    Alpha,
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

pub(crate) fn render(engen: &mut Engen) {
    let gfx_surface = engen
        .backend
        .container
        .get_resource::<GfxSurface>()
        .expect("no gfx surface attached");
    let gfx_surface_configuration = engen
        .backend
        .container
        .get_resource::<GfxSurfaceConfiguration>()
        .expect("no gfx surface configuration");
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
    if let Some(surface_texture) = gfx_surface.surface_texture(gfx_surface_configuration) {
        let mut command_encoder =
            gfx_surface
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
                        stencil_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(0u32),
                            store: true,
                        }),
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
        gfx_surface
            .queue
            .submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
    }
}
