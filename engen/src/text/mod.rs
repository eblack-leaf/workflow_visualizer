use crate::text::rasterization::Rasterization;
use bevy_ecs::prelude::{Entity, Resource};
use std::collections::{HashMap, HashSet};

mod attach;
mod cache;
mod changes;
mod component;
mod compute_instrumentation;
mod extract;
mod font;
mod grow;
mod index;
mod instance;
mod rasterization;
mod render;
mod scale;
mod vertex;
pub use crate::text::component::{Text, TextBundle};
pub use crate::text::scale::Scale;

#[derive(Resource)]
pub struct Renderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) rasterizations: HashMap<Entity, Rasterization>,
    pub(crate) visible_text_entities: HashSet<Entity>,
    pub(crate) rasterization_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) sampler_bind_group: wgpu::BindGroup,
}
