mod attribute;
mod bundle;
mod font;
mod pipeline;
mod rasterize;
mod render;
mod scale;
mod vertex;
mod vertex_buffer;

use bevy_ecs::prelude::Component;
pub use pipeline::Pipeline;
pub use rasterize::placement::RasterizationPlacement;
pub use render::render;
pub use rasterize::RasterizationBinding;
pub use attribute::{Coordinator, CpuAttributes, GpuAttributes};
pub use vertex_buffer::VertexBuffer;
#[derive(Component)]
pub struct Text {
    pub line: String,
}
impl Text {
    pub fn new(line: String) -> Self {
        Self {
            line: line.split('\n').collect::<Vec<String>>().iter().first().unwrap(),
        }
    }
}
