mod attributes;
mod font;
mod glyph;
mod layout;
mod rasterization;
mod render;
mod scale;
mod vertex;
pub use crate::text_step_out::attributes::add::{
    add_cpu_attrs, add_gpu_attrs, add_instances, growth, setup_added_instances,
};
pub use crate::text_step_out::attributes::remove::remove_instances;
pub use crate::text_step_out::attributes::write::{write_cpu_attrs, write_gpu_attrs};
pub use crate::text_step_out::attributes::{setup_attribute_buffers, setup_attributes};
pub use crate::text_step_out::rasterization::placement::RasterizationPlacement;
pub use crate::text_step_out::rasterization::{rasterize_adds, rasterize_writes};
pub use crate::text_step_out::render::TextRenderer;
