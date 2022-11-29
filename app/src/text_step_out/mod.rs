pub use crate::text_step_out::attributes::add::{
    add_cpu_attrs, add_gpu_attrs, add_instances, growth, setup_added_instances,
};
pub use crate::text_step_out::attributes::remove::remove_instances;
pub use crate::text_step_out::attributes::write::{write_cpu_attrs, write_gpu_attrs};
pub use crate::text_step_out::attributes::{
    setup_attribute_buffers, setup_attribute_queues, Coordinator, CpuAttributes, GpuAttributes,
};
pub use crate::text_step_out::font::font;
pub use crate::text_step_out::rasterization::placement::RasterizationPlacement;
pub use crate::text_step_out::rasterization::{rasterize, setup_rasterization, Rasterizations};
pub use crate::text_step_out::render::{setup_text_renderer, TextRenderer};
mod attributes;
mod cache;
mod font;
mod layout;
mod rasterization;
mod render;
mod scale;
mod vertex;
