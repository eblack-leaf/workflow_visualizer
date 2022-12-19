use crate::canvas::Canvas;
use crate::viewport::Viewport;
use crate::{Gfx, Job};
use std::sync::{Arc, Mutex};
use wgpu::{SurfaceError, SurfaceTexture};

pub trait Render {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, viewport: &'a Viewport);
    fn extract(&mut self, job: Job);
    fn prepare(&mut self, canvas: &Canvas);
}
pub(crate) fn render(gfx: &mut Gfx) {
    if let Some(surface_texture) = surface_texture(gfx.canvas.as_ref().unwrap()).take() {
        let mut command_encoder = gfx.canvas.as_ref().unwrap().device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("command encoder"),
            },
        );
        let surface_texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        {
            let depth_texture_view = gfx
                .viewport
                .as_ref()
                .unwrap()
                .depth_texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(gfx.theme.background.into()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(gfx.viewport.as_ref().unwrap().cpu.far_layer()),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            gfx.render_implementors
                .iter_mut()
                .for_each(|mut implementor| {
                    implementor.prepare(gfx.canvas.as_ref().unwrap());
                    implementor.render(&mut render_pass, gfx.viewport.as_ref().unwrap());
                });
        }
        gfx.canvas
            .as_ref()
            .unwrap()
            .queue
            .submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
    }
}
pub fn surface_texture(canvas: &Canvas) -> Option<SurfaceTexture> {
    let surface_texture = match canvas.surface.get_current_texture() {
        Ok(surface_texture) => Some(surface_texture),
        Err(err) => match err {
            SurfaceError::Timeout => None,
            SurfaceError::Outdated => {
                canvas
                    .surface
                    .configure(&canvas.device, &canvas.surface_configuration);
                Some(
                    canvas
                        .surface
                        .get_current_texture()
                        .expect("configuring did not solve surface outdated"),
                )
            }
            SurfaceError::Lost => {
                canvas
                    .surface
                    .configure(&canvas.device, &canvas.surface_configuration);
                Some(
                    canvas
                        .surface
                        .get_current_texture()
                        .expect("configuring did not solve surface lost"),
                )
            }
            SurfaceError::OutOfMemory => {
                panic!("gpu out of memory");
            }
        },
    };
    surface_texture
}
