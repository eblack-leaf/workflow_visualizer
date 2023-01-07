use crate::canvas::{Canvas, Viewport};
use crate::theme::Theme;
use crate::{Engen, Task};
pub enum RenderPhase {
    Opaque,
    Alpha,
}
type RenderCallFn = dyn FnMut(&Task, &mut RenderPassHandle);
type ExtractCallFn = dyn FnMut(&mut Task, &Task);
pub(crate) struct RenderCalls {
    pub(crate) opaque: Vec<Box<RenderCallFn>>,
    pub(crate) alpha: Vec<Box<RenderCallFn>>,
}
impl RenderCalls {
    pub(crate) fn new() -> Self {
        Self {
            opaque: Vec::new(),
            alpha: Vec::new(),
        }
    }
    pub(crate) fn insert<RenderCall: FnMut(&Task, &mut RenderPassHandle) + 'static>(
        &mut self,
        phase: RenderPhase,
        render_call: RenderCall,
    ) {
        let storage = match phase {
            RenderPhase::Opaque => &mut self.opaque,
            RenderPhase::Alpha => &mut self.alpha,
        };
        storage.push(Box::new(render_call));
    }
}
pub(crate) struct ExtractCalls {
    pub(crate) fns: Vec<Box<ExtractCallFn>>,
}
impl ExtractCalls {
    pub(crate) fn new() -> Self {
        Self { fns: Vec::new() }
    }
}
pub trait Render {
    fn extract(compute: &mut Task, render: &Task)
    where
        Self: Sized;
    fn phase() -> RenderPhase;
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport);
}
pub(crate) fn extract(engen: &mut Engen) {
    for caller in engen.extract_calls.fns.iter_mut() {
        caller(&mut engen.compute, &engen.render);
    }
}
pub struct RenderPassHandle<'a>(pub wgpu::RenderPass<'a>);
pub fn render(engen: &mut Engen) {
    let canvas = engen
        .render
        .container
        .get_resource::<Canvas>()
        .expect("no canvas attached");
    let theme = engen
        .render
        .container
        .get_resource::<Theme>()
        .expect("no theme attached");
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
                            load: wgpu::LoadOp::Clear(canvas.viewport.cpu.far_layer()),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                },
            ));
            for render_call in engen.render_calls.opaque.iter_mut() {
                render_call(&engen.render, &mut render_pass_handle);
            }
            for render_call in engen.render_calls.alpha.iter_mut() {
                render_call(&engen.render, &mut render_pass_handle);
            }
        }
        canvas
            .queue
            .submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
    }
}
