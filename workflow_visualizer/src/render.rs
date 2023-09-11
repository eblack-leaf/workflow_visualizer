#[cfg(not(target_family = "wasm"))]
use bevy_ecs::prelude::Resource;
use tracing::trace;

use crate::gfx::{GfxSurfaceConfiguration, MsaaRenderAdapter};
use crate::visualizer::Visualizer;
use crate::{GfxSurface, Job, ScaleFactor, Theme, Viewport};

/// Phase for Rendering
pub enum RenderPhase {
    Opaque,
    Alpha(u32),
}

#[cfg(not(target_family = "wasm"))]
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

#[cfg(target_family = "wasm")]
pub(crate) fn invoke_render<'a, Renderer: Render + 'static>(
    job: &'a Job,
    render_pass_handle: &mut RenderPassHandle<'a>,
) {
    let viewport = job
        .container
        .get_non_send_resource::<Viewport>()
        .expect("no viewport attached");
    job.container
        .get_non_send_resource::<Renderer>()
        .expect("no render attachment")
        .render(render_pass_handle, viewport);
}

/// Wrapper around wgpu::RenderPass
pub struct RenderPassHandle<'a>(pub wgpu::RenderPass<'a>);
pub(crate) type RenderTask = Box<for<'a> fn(&'a Job, &mut RenderPassHandle<'a>)>;
pub(crate) struct RenderTaskManager {
    pub(crate) opaque: Vec<RenderTask>,
    pub(crate) transparent: Vec<(u32, RenderTask)>,
}
impl RenderTaskManager {
    pub(crate) fn new() -> Self {
        Self {
            opaque: Vec::new(),
            transparent: Vec::new(),
        }
    }
}

/// Trait to extend the render loop with a new pipeline
pub trait Render {
    fn setup(
        visualizer: &Visualizer,
        gfx: &GfxSurface,
        viewport: &Viewport,
        gfx_config: &GfxSurfaceConfiguration,
        msaa: &MsaaRenderAdapter,
        scale_factor: &ScaleFactor,
    ) -> Self;
    fn phase() -> RenderPhase;
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport);
}

pub(crate) fn internal_render(visualizer: &mut Visualizer) {
    #[cfg(not(target_family = "wasm"))]
    let gfx = visualizer
        .job
        .container
        .get_resource::<GfxSurface>()
        .expect("no gfx surface attached");
    #[cfg(not(target_family = "wasm"))]
    let gfx_config = visualizer
        .job
        .container
        .get_resource::<GfxSurfaceConfiguration>()
        .expect("no gfx surface configuration");
    #[cfg(not(target_family = "wasm"))]
    let viewport = visualizer
        .job
        .container
        .get_resource::<Viewport>()
        .expect("no viewport attached");
    #[cfg(not(target_family = "wasm"))]
    let msaa = visualizer
        .job
        .container
        .get_resource::<MsaaRenderAdapter>()
        .expect("no msaa attachment");
    #[cfg(target_family = "wasm")]
    let gfx = visualizer
        .job
        .container
        .get_non_send_resource::<GfxSurface>()
        .unwrap();
    #[cfg(target_family = "wasm")]
    let gfx_config = visualizer
        .job
        .container
        .get_non_send_resource::<GfxSurfaceConfiguration>()
        .unwrap();
    #[cfg(target_family = "wasm")]
    let msaa = visualizer
        .job
        .container
        .get_non_send_resource::<MsaaRenderAdapter>()
        .unwrap();
    #[cfg(target_family = "wasm")]
    let viewport = visualizer
        .job
        .container
        .get_non_send_resource::<Viewport>()
        .unwrap();
    let theme = visualizer
        .job
        .container
        .get_resource::<Theme>()
        .expect("no theme attached");
    if let Some(surface_texture) = gfx.surface_texture(gfx_config) {
        trace!("obtained surface texture");
        let mut command_encoder =
            gfx.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("command encoder"),
                });
        let surface_texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        {
            let depth_texture_view = viewport
                .depth_texture()
                .create_view(&wgpu::TextureViewDescriptor::default());
            let (v, rt) = match msaa.view() {
                Some(view) => (view, Some(&surface_texture_view)),
                None => (&surface_texture_view, None),
            };
            let should_store = msaa.requested() == 1;
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
                        load: wgpu::LoadOp::Clear(viewport.far_layer()),
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
            for invoke in visualizer.render_task_manager.opaque.iter_mut() {
                trace!("invoking opaque render fn");
                invoke(&visualizer.job, &mut render_pass_handle);
            }
            for (_, invoke) in visualizer.render_task_manager.transparent.iter_mut() {
                trace!("invoking transparent render fn");
                invoke(&visualizer.job, &mut render_pass_handle);
            }
        }
        gfx.queue.submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
    }
}
