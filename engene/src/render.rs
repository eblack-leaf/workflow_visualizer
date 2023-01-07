use crate::canvas::{Canvas, Viewport};
use crate::theme::Theme;
use crate::{Engen, Task};
use bevy_ecs::prelude::Resource;
pub enum RenderPhase {
    Opaque,
    Alpha,
}
pub(crate) fn call_extract<Renderer: Render>(compute: &Task, render: &mut Task) {
    Renderer::extract(compute, render);
}
pub(crate) fn call_render<'a, Renderer: Render + Resource>(
    render: &'a Task,
    render_pass_handle: &mut RenderPassHandle<'a>,
) {
    let viewport = &render
        .container
        .get_resource::<Canvas>()
        .expect("no canvas attached")
        .viewport;
    render
        .container
        .get_resource::<Renderer>()
        .expect("no renderer attached")
        .render(render_pass_handle, viewport);
}
pub(crate) struct RenderCalls {
    pub(crate) opaque: Vec<Box<for<'a> fn(&'a Task, &mut RenderPassHandle<'a>)>>,
    pub(crate) alpha: Vec<Box<for<'a> fn(&'a Task, &mut RenderPassHandle<'a>)>>,
}
impl RenderCalls {
    pub(crate) fn new() -> Self {
        Self {
            opaque: Vec::new(),
            alpha: Vec::new(),
        }
    }
    pub(crate) fn add(
        &mut self,
        phase: RenderPhase,
        render_call: for<'a> fn(&'a Task, &mut RenderPassHandle<'a>),
    ) {
        let storage = match phase {
            RenderPhase::Opaque => &mut self.opaque,
            RenderPhase::Alpha => &mut self.alpha,
        };
        storage.push(Box::new(render_call));
    }
}
pub(crate) struct ExtractCalls {
    pub(crate) fns: Vec<Box<fn(&Task, &mut Task)>>,
}
impl ExtractCalls {
    pub(crate) fn new() -> Self {
        Self { fns: Vec::new() }
    }
    pub(crate) fn add(&mut self, caller: fn(&Task, &mut Task)) {
        self.fns.push(Box::new(caller));
    }
}
pub trait Render {
    fn extract(compute: &Task, render: &mut Task)
    where
        Self: Sized;
    fn phase() -> RenderPhase;
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport);
}
pub(crate) fn extract(engen: &mut Engen) {
    for caller in engen.extract_calls.fns.iter_mut() {
        caller(&engen.compute, &mut engen.render);
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
