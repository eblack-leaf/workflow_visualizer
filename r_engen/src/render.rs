use crate::canvas::{Canvas, Viewport};
use crate::task::Task;
use crate::theme::Theme;
use crate::Engen;
use bevy_ecs::prelude::{NonSendMut, Res};
use std::collections::HashMap;
use wgpu::RenderPass;
pub trait RenderAttachment {
    fn attach(&self, engen: &mut Engen);
    fn extractor(&self) -> Box<dyn Extract>;
    fn renderer(&self, canvas: &Canvas) -> Box<dyn Render>;
}
#[derive(Eq, Hash, PartialEq)]
pub struct Id(pub &'static str);
pub trait Extract {
    fn extract(&mut self, compute: &Task, render: &mut Task);
}
pub trait Render {
    fn id(&self) -> Id;
    fn phase(&self) -> RenderPhase;
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport);
}
pub enum RenderPhase {
    Opaque,
    Alpha,
}
pub type Renderer = Box<dyn Render>;
pub struct RenderPhases {
    pub opaque: HashMap<Id, Renderer>,
    pub alpha: HashMap<Id, Renderer>,
}
impl RenderPhases {
    pub fn new() -> Self {
        Self {
            opaque: HashMap::new(),
            alpha: HashMap::new(),
        }
    }
    pub fn insert(&mut self, renderer: Box<dyn Render>) {
        let phases = match renderer.phase() {
            RenderPhase::Opaque => &mut self.opaque,
            RenderPhase::Alpha => &mut self.alpha,
        };
        phases.insert(renderer.id(), renderer);
    }
    pub(crate) fn render<'a>(
        &'a self,
        render_pass: &mut RenderPassHandle<'a>,
        viewport: &'a Viewport,
    ) {
        for (_id, renderer) in self.opaque.iter() {
            renderer.render(render_pass, viewport);
        }
        for (_id, renderer) in self.alpha.iter() {
            renderer.render(render_pass, viewport);
        }
    }
}
pub struct RenderPassHandle<'a>(pub wgpu::RenderPass<'a>);
pub fn render(canvas: Res<Canvas>, theme: Res<Theme>, render_phases: NonSendMut<RenderPhases>) {
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
            let mut render_pass = RenderPassHandle(command_encoder.begin_render_pass(
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
            render_phases.render(&mut render_pass, &canvas.viewport);
        }
        canvas
            .queue
            .submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
    }
}
