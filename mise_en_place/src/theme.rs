use bevy_ecs::prelude::Resource;

use crate::color::Color;
use crate::{Preparation, Stove};

pub struct ThemeDescriptor {
    pub background: Option<Color>,
}

impl ThemeDescriptor {
    pub fn new() -> Self {
        Self { background: None }
    }
}

#[derive(Resource, Clone)]
pub struct Butter {
    pub background: Color,
}

impl Butter {
    pub fn new(descriptor: ThemeDescriptor) -> Self {
        Self {
            background: descriptor.background.unwrap_or(Color::rgb(0.0, 0.0, 0.0)),
        }
    }
}

impl Default for Butter {
    fn default() -> Self {
        Butter::new(ThemeDescriptor::new())
    }
}

impl Preparation for Butter {
    fn prepare(engen: &mut Stove) {
        engen.backend.container.insert_resource(Butter::default());
    }
}
