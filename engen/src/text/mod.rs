mod font;
mod pipeline;
mod rasterization;
mod request;
mod scale;
mod vertex;

use crate::canvas::{Canvas, Viewport};
use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::instance::Coordinator;
use crate::instance::EntityKey;
use crate::render::{Render, RenderPhase};
use crate::text::pipeline::pipeline;
use crate::text::rasterization::{GlyphHash, Rasterization};
use crate::text::request::InstanceRequests;
pub use crate::text::scale::Scale;
use crate::text::vertex::GLYPH_AABB;
use crate::{render, Launcher, Task};
use bevy_ecs::prelude::SystemStage;
pub use font::{font, Font};
pub(crate) use request::{GlyphOffset, InstanceRequest};
use wgpu::RenderPass;

pub(crate) type InstanceCoordinator = Coordinator<EntityKey<GlyphOffset>, InstanceRequest>;
pub struct TextRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) rasterization: Rasterization,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) coordinator: InstanceCoordinator,
}
impl Render for TextRenderer {
    fn phase() -> RenderPhase
    where
        Self: Sized,
    {
        RenderPhase::Alpha
    }
    fn id() -> render::Id {
        render::Id("text")
    }
    fn extract(&mut self, compute: &mut Task) {
        self.coordinator.requests = compute
            .job
            .container
            .get_resource_mut::<InstanceRequests>()
            .expect("no instance requests")
            .requests
            .drain()
            .collect();
    }
    fn prepare(&mut self, canvas: &Canvas) {
        rasterization::read_requests(&mut self.rasterization, &self.coordinator);
        rasterization::resolve(&mut self.rasterization);
        rasterization::remove(&mut self.rasterization, canvas);
        rasterization::rasterize(&mut self.rasterization);
        rasterization::place(&mut self.rasterization);
        rasterization::write(&mut self.rasterization, canvas);
        rasterization::integrate_placements(&self.rasterization, &mut self.coordinator);
        self.coordinator.prepare(&canvas.device);
        self.coordinator.process_attribute(&canvas, |i| i.position);
        self.coordinator.process_attribute(&canvas, |i| i.area);
        self.coordinator.process_attribute(&canvas, |i| i.depth);
        self.coordinator.process_attribute(&canvas, |i| i.color);
        self.coordinator
            .process_attribute(&canvas, |i| i.descriptor.unwrap());
        self.coordinator.finish();
    }
    fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>, viewport: &'a Viewport) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &viewport.bind_group, &[]);
        render_pass.set_bind_group(1, &self.rasterization.buffer.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.coordinator.gpu_buffer::<Position>().slice(..));
        render_pass.set_vertex_buffer(2, self.coordinator.gpu_buffer::<Area>().slice(..));
        render_pass.set_vertex_buffer(3, self.coordinator.gpu_buffer::<Depth>().slice(..));
        render_pass.set_vertex_buffer(4, self.coordinator.gpu_buffer::<Color>().slice(..));
        render_pass.set_vertex_buffer(
            5,
            self.coordinator
                .gpu_buffer::<rasterization::Descriptor>()
                .slice(..),
        );
        if self.coordinator.current() > 0 {
            render_pass.draw(
                0..GLYPH_AABB.len() as u32,
                0..self.coordinator.current() as u32,
            );
        }
    }
    fn instrument(&self, task: &mut Task) {
        task.job.container.insert_resource(InstanceRequests::new());
        task.job.container.insert_resource(font());
        // replace with labeled stages of task so this can be last
        task.job
            .exec
            .add_stage("text request", SystemStage::single(request::emit_requests));
    }
    fn renderer(canvas: &Canvas) -> Self
    where
        Self: Sized,
    {
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
