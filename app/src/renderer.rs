use std::sync::Arc;

use bevy_ecs::prelude::{Commands, NonSendMut, Res, ResMut};
use wgpu::{SurfaceError, SurfaceTexture};

use crate::depth_texture::DepthTexture;
use crate::text::vertex_buffer::VertexBuffer;
use crate::theme::Theme;
use crate::viewport;
use crate::{text, Area, Color, Depth, Position};

pub struct CurrentSurfaceTexture {
    pub surface_texture: Option<wgpu::SurfaceTexture>,
}

pub fn get_surface_texture(
    surface: Res<wgpu::Surface>,
    device: Res<wgpu::Device>,
    surface_configuration: Res<wgpu::SurfaceConfiguration>,
    mut cmd: Commands,
) {
    let surface_texture = match surface.get_current_texture() {
        Ok(surface_texture) => Some(surface_texture),
        Err(err) => match err {
            SurfaceError::Timeout => None,
            SurfaceError::Outdated => {
                surface.configure(&device, &surface_configuration);
                Some(
                    surface
                        .get_current_texture()
                        .expect("configuring did not solve surface outdated"),
                )
            }
            SurfaceError::Lost => {
                surface.configure(&device, &surface_configuration);
                Some(
                    surface
                        .get_current_texture()
                        .expect("configuring did not solve surface lost"),
                )
            }
            SurfaceError::OutOfMemory => {
                panic!("gpu out of memory");
            }
        },
    };
    cmd.insert_resource(CurrentSurfaceTexture { surface_texture });
}

pub fn command_encoder(mut cmd: Commands, device: Res<wgpu::Device>) {
    cmd.insert_resource(
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("command encoder"),
        }),
    );
}

pub fn begin_render_pass(
    mut cmd: Commands,
    mut surface_texture: ResMut<CurrentSurfaceTexture>,
    theme: Res<Theme>,
    depth_texture: Res<DepthTexture>,
    viewport: Res<viewport::Viewport>,
    device: Res<wgpu::Device>,
    mut command_encoder: ResMut<wgpu::CommandEncoder>,
) {
    if let Some(surface_texture) = surface_texture.surface_texture.take() {
        let surface_texture_view = surface_texture
            .texture
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
                view: &depth_texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default()),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(viewport.far_layer()),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });
        cmd.insert_resource(Arc::new(render_pass));
    }
}

pub fn end_render_pass(mut cmd: Commands) {
    cmd.remove_resource::<Arc<wgpu::RenderPass>>();
}

pub fn submit(
    queue: Res<wgpu::Queue>,
    mut command_encoder: Res<Arc<wgpu::CommandEncoder>>,
    surface_texture: Res<wgpu::SurfaceTexture>,
) {
    queue.submit(std::iter::once(command_encoder.finish()));
    surface_texture.present();
}
