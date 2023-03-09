use bevy_ecs::prelude::Resource;

use crate::engen::Engen;
use crate::gfx::{GfxSurface, GfxSurfaceConfiguration};
use crate::{Job, Theme, Viewport};

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

#[derive(Resource)]
pub(crate) struct MsaaRenderAttachment {
    pub(crate) max: u32,
    pub(crate) requested: u32,
    pub(crate) view: Option<wgpu::TextureView>,
}

impl MsaaRenderAttachment {
    pub(crate) fn new(
        gfx_surface: &GfxSurface,
        gfx_surface_config: &GfxSurfaceConfiguration,
        max: u32,
        requested: u32,
    ) -> Self {
        let requested = requested.min(max);
        match requested > 1u32 {
            true => {
                let texture_extent = wgpu::Extent3d {
                    width: gfx_surface_config.configuration.width,
                    height: gfx_surface_config.configuration.height,
                    depth_or_array_layers: 1,
                };
                let descriptor = wgpu::TextureDescriptor {
                    size: texture_extent,
                    mip_level_count: 1,
                    sample_count: requested,
                    dimension: wgpu::TextureDimension::D2,
                    format: gfx_surface_config.configuration.format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    label: Some("msaa render attachment"),
                    view_formats: &[],
                };
                let texture = gfx_surface.device.create_texture(&descriptor);
                let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                Self {
                    view: Some(view),
                    max,
                    requested,
                }
            }
            false => Self {
                view: None,
                max: 1,
                requested,
            },
        }
    }
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
    let msaa_attachment = engen
        .backend
        .container
        .get_resource::<MsaaRenderAttachment>()
        .expect("no msaa attachment");
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
            let (v, rt) = match &msaa_attachment.view {
                Some(view) => (view, Some(&surface_texture_view)),
                None => (&surface_texture_view, None),
            };
            let should_store = msaa_attachment.requested == 1;
            let color_attachment = wgpu::RenderPassColorAttachment {
                view: v,
                resolve_target: rt,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(theme.background.into()),
                    store: should_store,
                },
            };
            let render_pass_descriptor = wgpu::RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[Some(color_attachment)],
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
            };
            let mut render_pass_handle =
                RenderPassHandle(command_encoder.begin_render_pass(&render_pass_descriptor));
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
