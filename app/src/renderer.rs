use crate::depth_texture::DepthTexture;
use crate::viewport::Viewport;
use bevy_ecs::prelude::Res;
use wgpu::{SurfaceError, SurfaceTexture};

pub struct Renderer {}
impl Renderer {
    pub fn acquire_surface_texture(
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
                    surface.configure(&device, &config);
                    return Some(
                        surface
                            .get_current_texture()
                            .expect("configuring did not solve problem"),
                    );
                }
                SurfaceError::Lost => {
                    surface.configure(&device, &config);
                    return Some(
                        surface
                            .get_current_texture()
                            .expect("configuring did not solve problem"),
                    );
                }
                SurfaceError::OutOfMemory => {
                    panic!("gpu out of memory");
                }
            },
        }
    }
    pub fn acquire_surface_texture_view(surface_texture: &wgpu::SurfaceTexture) -> wgpu::TextureView {
        return surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
    }
    pub fn acquire_command_encoder(device: &wgpu::Device) -> wgpu::CommandEncoder {
        return device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("command encoder"),
        });
    }
    pub fn acquire_render_pass<'a>(command_encoder: &mut wgpu::CommandEncoder, surface_texture_view: &wgpu::TextureView, depth_texture_view: &wgpu::TextureView, viewport_depth: f32) -> wgpu::RenderPass<'a> {
        return command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &surface_texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(viewport.depth()),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });
    }
}
pub fn render(
    surface: Res<wgpu::Surface>,
    device: Res<wgpu::Device>,
    queue: Res<wgpu::Queue>,
    viewport: Res<Viewport>,
    depth_texture: Res<DepthTexture>,
    config: Res<wgpu::SurfaceConfiguration>,
) {
    if let Some(surface_texture) = Renderer::acquire_surface_texture(&surface, &device, &config) {
        let surface_texture_view = Renderer::acquire_surface_texture_view(&surface_texture);
        let mut command_encoder = Renderer::acquire_command_encoder(&device);
        {
            let mut render_pass = Renderer::acquire_render_pass(&mut command_encoder,
            &surface_texture_view, &depth_texture.view, viewport.far_layer());
            // do work
        }
        queue.submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
    }
}
