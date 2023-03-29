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
    pub fn with_background<C: Into<Color>>(mut self, color: C) -> Self {
        self.background.replace(color.into());
        self
    }
    pub fn with_primary<C: Into<Color>>(mut self, color: C) -> Self {
        self.primary.replace(color.into());
        self
    }
}

#[derive(Resource, Copy, Clone)]
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
        let theme = engen.options.theme;
        engen.frontend.container.insert_resource(theme);
        engen.backend.container.insert_resource(theme);
    }
}
