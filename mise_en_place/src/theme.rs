use bevy_ecs::prelude::Resource;

use crate::color::Color;
use crate::engen::{Attach, Engen};

pub struct ThemeDescriptor {
    pub background: Option<Color>,
    pub primary: Option<Color>,
}

impl ThemeDescriptor {
    pub fn new() -> Self {
        Self {
            background: None,
            primary: None,
        }
    }
}

#[derive(Resource, Clone)]
pub struct Theme {
    pub background: Color,
    pub primary: Color,
}

impl Theme {
    pub fn new(descriptor: ThemeDescriptor) -> Self {
        Self {
            background: descriptor
                .background
                .unwrap_or(Color::from(Color::OFF_BLACK)),
            primary: descriptor.primary.unwrap_or(Color::from(Color::OFF_WHITE)),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::new(ThemeDescriptor::new())
    }
}

pub struct ThemeAttachment;

impl Attach for ThemeAttachment {
    fn attach(engen: &mut Engen) {
        engen.backend.container.insert_resource(Theme::default());
    }
}
