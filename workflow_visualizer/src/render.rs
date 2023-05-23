use crate::gfx::{GfxSurfaceConfiguration, MsaaRenderAdapter};
use crate::visualizer::Visualizer;
use crate::{GfxSurface, Job, Theme, Viewport};
use bevy_ecs::prelude::Resource;
use tracing::{trace, warn};

pub enum RenderPhase {
    Opaque,
    Alpha(u32),
}
pub(crate) fn invoke_render<'a, Renderer: Render + Resource>(
    job: &'a Job,
    render_pass_handle: &mut RenderPassHandle<'a>,
) {
    let viewport = job
        .container
        .get_resource::<Viewport>()
        .expect("no viewport attached");
    job.container
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

pub(crate) fn internal_render(visualizer: &mut Visualizer) {
    let gfx_surface = visualizer
        .job
        .container
        .get_resource::<GfxSurface>()
        .expect("no gfx surface attached");
    let gfx_surface_configuration = visualizer
        .job
        .container
        .get_resource::<GfxSurfaceConfiguration>()
        .expect("no gfx surface configuration");
    let theme = visualizer
        .job
        .container
        .get_resource::<Theme>()
        .expect("no theme attached");
    let viewport = visualizer
        .job
        .container
        .get_resource::<Viewport>()
        .expect("no viewport attached");
    let msaa_attachment = visualizer
        .job
        .container
        .get_resource::<MsaaRenderAdapter>()
        .expect("no msaa attachment");
    if let Some(surface_texture) = gfx_surface.surface_texture(gfx_surface_configuration) {
        trace!("obtained surface texture");
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
            let (v, rt) = match msaa_attachment.view() {
                Some(view) => (view, Some(&surface_texture_view)),
                None => (&surface_texture_view, None),
            };
            let should_store = msaa_attachment.requested() == 1;
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
            trace!("beginning render pass");
            for invoke in visualizer.render_fns.0.iter_mut() {
                trace!("invoking opaque render fn");
                invoke(&visualizer.job, &mut render_pass_handle);
            }
            for invoke in visualizer.render_fns.1.iter_mut() {
                trace!("invoking transparent render fn");
                invoke(&visualizer.job, &mut render_pass_handle);
            }
        }
        gfx_surface
            .queue
            .submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
    }
}
