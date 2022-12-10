use bevy_ecs::prelude::Component;

pub(crate) mod attribute;
mod bundle;
pub(crate) mod font;
pub(crate) mod pipeline;
pub(crate) mod rasterize;
pub(crate) mod render;
mod scale;
mod vertex;
pub(crate) mod vertex_buffer;

#[derive(Component)]
pub struct Text {
    pub line: String,
}

impl Text {
    pub fn new(line: String) -> Self {
        Self {
            line: line
                .split('\n')
                .collect::<Vec<&'static str>>()
                .first()
                .unwrap()
                .parse()
                .unwrap(),
        }
    }
}
