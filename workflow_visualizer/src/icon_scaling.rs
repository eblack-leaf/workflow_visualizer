use crate::{Area, InterfaceContext};
use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

#[derive(Component, Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
pub enum IconScale {
    Custom(u32),
    Asymmetrical((u32, u32)),
}

impl From<u32> for IconScale {
    fn from(value: u32) -> Self {
        Self::Custom(value)
    }
}

impl IconScale {
    pub fn as_area(&self) -> Area<InterfaceContext> {
        match &self {
            IconScale::Custom(dim) => Area::from((*dim, *dim)),
            IconScale::Asymmetrical((width, height)) => Area::from((*width, *height)),
        }
    }
    pub fn width(&self) -> f32 {
        self.as_area().width
    }
    pub fn height(&self) -> f32 {
        self.as_area().height
    }
}
