use bevy_ecs::prelude::Resource;

use crate::color::Color;
use crate::Engen;

pub struct ThemeDescriptor {
    pub background: Option<Color>,
}

impl ThemeDescriptor {
    pub fn new() -> Self {
        Self { background: None }
    }
}

#[derive(Resource, Clone)]
pub struct Theme {
    pub background: Color,
}

impl Theme {
    pub fn new(descriptor: ThemeDescriptor) -> Self {
        Self {
            background: descriptor.background.unwrap_or(Color::rgb(0.0, 0.0, 0.0)),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::new(ThemeDescriptor::new())
    }
}

