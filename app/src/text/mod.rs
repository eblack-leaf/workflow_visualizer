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
pub use rasterize::placement::Placement;
pub use render::render;
#[derive(Component)]
pub struct Text {
    pub line: String,
}
impl Text {
    pub fn new(line: String) -> Self {
        Self {
            line: line.split('\n').collect().iter().first().unwrap(),
        }
    }
}
