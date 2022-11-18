use bevy_ecs::prelude::Res;
use wgpu::{SurfaceError, SurfaceTexture};

use crate::depth_texture::DepthTexture;
use crate::text_refactor;
use crate::text_refactor::{Rasterization, TextRenderer};
use crate::theme::Theme;
use crate::viewport::{Viewport, ViewportBinding};

pub fn get_surface_texture(
    surface: &wgpu::Surface,
    device: &wgpu::Device,
    surface_configuration: &wgpu::SurfaceConfiguration,
) -> Option<wgpu::SurfaceTexture> {
    match surface.get_current_texture() {
        Ok(surface_texture) => Some(surface_texture),
        Err(err) => match err {
            SurfaceError::Timeout => {
                return None;
            }
            SurfaceError::Outdated => {
                surface.configure(&device, &surface_configuration);
                return Some(
                    surface
                        .get_current_texture()
                        .expect("configuring did not solve surface outdated"),
                );
            }
            SurfaceError::Lost => {
                surface.configure(&device, &surface_configuration);
                return Some(
                    surface
                        .get_current_texture()
                        .expect("configuring did not solve surface lost"),
                );
            }
            SurfaceError::OutOfMemory => {
                panic!("gpu out of memory");
            }
        },
    }
}

pub fn render(
    surface: Res<wgpu::Surface>,
    device: Res<wgpu::Device>,
    queue: Res<wgpu::Queue>,
    viewport: Res<Viewport>,
    viewport_binding: Res<ViewportBinding>,
    text_renderer: Res<TextRenderer>,
    rasterization: Res<Rasterization>,
    depth_texture: Res<DepthTexture>,
    instances: Res<text_refactor::Instances>,
    surface_configuration: Res<wgpu::SurfaceConfiguration>,
    theme: Res<Theme>,
) {
    if let Some(surface_texture) = get_surface_texture(&surface, &device, &surface_configuration) {
        let surface_texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("command encoder"),
        });
        {
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
                    view: &depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(viewport.far_layer()),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            text_renderer.render(
                &mut render_pass,
                &rasterization,
                &viewport_binding,
                &instances,
            );
        }
        queue.submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
    }
}
