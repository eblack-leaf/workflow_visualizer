use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Entity, Resource};

use crate::canvas::Viewport;
use crate::render::{Render, RenderPassHandle, RenderPhase};
use crate::Task;
use crate::text::extraction::Extraction;
use crate::text::render_group::RenderGroup;
use crate::text::vertex::GLYPH_AABB;

#[derive(Resource)]
pub struct TextRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) render_groups: HashMap<Entity, RenderGroup>,
    pub(crate) render_group_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) sampler_bind_group: wgpu::BindGroup,
}

impl Render for TextRenderer {
    fn extract(compute: &mut Task, render: &mut Task)
        where
            Self: Sized,
    {
        let mut extraction = compute
            .container
            .get_resource_mut::<Extraction>()
            .expect("no extraction in compute");
        render.container.insert_resource(extraction.clone());
        *extraction = Extraction::new();
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
            if render_group.count() > 0 {
                render_pass_handle
                    .0
                    .set_vertex_buffer(1, render_group.glyph_position_gpu.slice(..));
                render_pass_handle.0.set_vertex_buffer(4, render_group.glyph_area_gpu.slice(..));
                render_pass_handle
                    .0
                    .set_vertex_buffer(2, render_group.null_gpu.slice(..));
                render_pass_handle
                    .0
                    .set_vertex_buffer(3, render_group.coords_gpu.slice(..));
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
                render_pass_handle
                    .0
                    .draw(0..GLYPH_AABB.len() as u32, 0..render_group.count() as u32);
            }
        }
    }
}
