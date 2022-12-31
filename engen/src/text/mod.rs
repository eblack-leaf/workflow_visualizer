mod pipeline;
mod rasterization;
mod scale;
mod vertex;

use crate::canvas::{Canvas, Viewport};
use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::instance::{Coordinator, EntityKey};
use crate::render::Render;
use crate::text::pipeline::pipeline;
use crate::text::rasterization::{GlyphHash, Rasterization};
pub use crate::text::scale::Scale;
use crate::text::vertex::GLYPH_AABB;
use crate::{render, App, Launcher};
use wgpu::RenderPass;
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct GlyphOffset(pub(crate) u32);
#[derive(Clone)]
pub(crate) struct InstanceRequest {
    pub(crate) character: char,
    pub(crate) scale: Scale,
    pub(crate) hash: GlyphHash,
    pub(crate) position: Position,
    pub(crate) area: Area,
    pub(crate) depth: Depth,
    pub(crate) color: Color,
    pub(crate) descriptor: Option<rasterization::Descriptor>,
}
pub(crate) type InstanceCoordinator = Coordinator<EntityKey<GlyphOffset>, InstanceRequest>;
pub struct TextRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) rasterization: Rasterization,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) coordinator: InstanceCoordinator,
}
impl TextRenderer {
    pub(crate) fn new(canvas: &Canvas) -> Self {
        let rasterization = Rasterization::new(&canvas.device);
        let pipeline = pipeline(canvas, &rasterization);
        let vertex_buffer = vertex::buffer(&canvas.device);
        let mut coordinator = InstanceCoordinator::new(10);
        coordinator.setup_attribute::<Position>(&canvas.device);
        coordinator.setup_attribute::<Area>(&canvas.device);
        coordinator.setup_attribute::<Depth>(&canvas.device);
        coordinator.setup_attribute::<Color>(&canvas.device);
        coordinator.setup_attribute::<rasterization::Descriptor>(&canvas.device);
        Self {
            pipeline,
            rasterization,
            vertex_buffer,
            coordinator,
        }
    }
}
impl Render for TextRenderer {
    fn id() -> render::Id {
        render::Id("text")
    }
    fn extract(&mut self, compute: &mut App) {
        todo!()
    }
    fn prepare(&mut self, canvas: &Canvas) {
        rasterization::read_requests(&mut self.rasterization, &self.coordinator);
        rasterization::resolve(&mut self.rasterization);
        rasterization::remove(&mut self.rasterization, canvas);
        rasterization::rasterize(&mut self.rasterization);
        rasterization::place(&mut self.rasterization);
        rasterization::write(&mut self.rasterization, canvas);
        rasterization::integrate_placements(&self.rasterization, &mut self.coordinator);
        self.coordinator.prepare();
        self.coordinator.process_attribute(|i| i.position);
        self.coordinator.process_attribute(|i| i.area);
        self.coordinator.process_attribute(|i| i.depth);
        self.coordinator.process_attribute(|i| i.color);
        self.coordinator
            .process_attribute(|i| i.descriptor.unwrap());
        self.coordinator.write::<Position>();
        self.coordinator.write::<Area>();
        self.coordinator.write::<Depth>();
        self.coordinator.write::<Color>();
        self.coordinator.write::<rasterization::Descriptor>();
    }
    fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>, viewport: &'a Viewport) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &viewport.bind_group, &[]);
        render_pass.set_bind_group(1, &self.rasterization.buffer.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.coordinator.attribute_buffer::<Position>().slice(..));
        render_pass.set_vertex_buffer(2, self.coordinator.attribute_buffer::<Area>().slice(..));
        render_pass.set_vertex_buffer(3, self.coordinator.attribute_buffer::<Depth>().slice(..));
        render_pass.set_vertex_buffer(4, self.coordinator.attribute_buffer::<Color>().slice(..));
        render_pass.set_vertex_buffer(
            5,
            self.coordinator
                .attribute_buffer::<rasterization::Descriptor>()
                .slice(..),
        );
        if self.coordinator.current() > 0 {
            render_pass.draw(
                0..GLYPH_AABB.len() as u32,
                0..self.coordinator.current() as u32,
            );
        }
    }
    fn attach(self, launcher: &mut Launcher) {
        let text_renderer = TextRenderer::new(launcher.canvas.as_ref().unwrap());
        // ... instrument compute with needed resources and systems
        launcher
            .renderers
            .alpha
            .insert(Self::id(), Box::new(text_renderer));
    }
}
