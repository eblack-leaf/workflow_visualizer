use std::collections::{HashMap, HashSet};
use bevy_ecs::prelude::{Entity, Resource};
use crate::canvas::Viewport;
use crate::clean_text::render_group::RenderGroup;
use crate::clean_text::vertex::GLYPH_AABB;
use crate::render::{Render, RenderPassHandle, RenderPhase};
use crate::Task;

#[derive(Resource)]
pub struct Renderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) render_groups: HashMap<Entity, RenderGroup>,
    pub(crate) visible_entities: HashSet<Entity>,
    pub(crate) render_group_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) sampler_bind_group: wgpu::BindGroup,
}

impl Render for Renderer {
    fn extract(compute: &mut Task, render: &mut Task) where Self: Sized {
        todo!()
    }

    fn phase() -> RenderPhase {
        RenderPhase::Alpha
    }

    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        render_pass_handle.0.set_pipeline(&self.pipeline);
        render_pass_handle
            .0
            .set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass_handle
            .0
            .set_bind_group(0, &viewport.bind_group, &[]);
        render_pass_handle
            .0
            .set_bind_group(1, &self.sampler_bind_group, &[]);
        for (entity, render_group) in self.render_groups.iter() {
            if render_group.count() > 0 && self.visible_entities.contains(entity) {
                render_pass_handle.0.set_vertex_buffer(1, render_group.glyph_position_gpu.slice(..));
                render_pass_handle.0.set_vertex_buffer(2, render_group.coords_gpu.slice(..));
                render_pass_handle.0.set_vertex_buffer(3, render_group.null_gpu.slice(..));
                render_pass_handle
                    .0
                    .set_bind_group(2, &render_group.bind_group, &[]);
                if let Some(bound) = &render_group.bounds {
                    render_pass_handle.0.set_scissor_rect(
                        bound.position.x as u32,
                        bound.position.y as u32,
                        bound.area.width as u32,
                        bound.area.height as u32,
                    );
                }
                render_pass_handle.0.draw(
                    0..GLYPH_AABB.len() as u32,
                    0..render_group.count() as u32,
                );
            }
        }
    }
}