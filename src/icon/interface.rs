use bevy_ecs::prelude::{Bundle, Component};

use crate::{Color, InterfaceContext, Layer, Section};
use crate::icon::mesh::{ColorInvert, IconKey};
use crate::view::{ViewArea, ViewPosition};
use crate::visibility::EnableVisibility;

#[derive(Component, Copy, Clone)]
pub struct IconSecondaryColor {
    pub secondary_color: Color,
}

impl IconSecondaryColor {
    pub fn new<C: Into<Color>>(secondary_color: C) -> Self {
        Self {
            secondary_color: secondary_color.into(),
        }
    }
}

#[derive(Bundle)]
pub struct Icon {
    pub secondary_color: IconSecondaryColor,
    pub size: IconSize,
    pub key: IconKey,
    pub view_position: ViewPosition,
    pub view_area: ViewArea,
    pub layer: Layer,
    pub color: Color,
    pub(crate) color_invert: ColorInvert,
    #[bundle]
    pub(crate) visibility: EnableVisibility,
    pub(crate) section: Section<InterfaceContext>,
}

impl Icon {
    pub fn new<C: Into<Color>>(
        key: IconKey,
        view_position: ViewPosition,
        view_area: ViewArea,
        layer: Layer,
        size: IconSize,
        color: C,
        secondary_color: IconSecondaryColor,
    ) -> Self {
        Self {
            secondary_color,
            size,
            key,
            view_position,
            view_area,
            layer,
            color: color.into(),
            color_invert: ColorInvert::off(),
            visibility: EnableVisibility::new(),
            section: Section::default(),
        }
    }
}

#[derive(Component, Copy, Clone)]
pub enum IconSize {
    Small,
    Medium,
    Large,
    Custom((f32, f32)),
}
