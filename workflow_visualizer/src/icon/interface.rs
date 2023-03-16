use crate::icon::mesh::{ColorInvert, IconKey};
use crate::visibility::EnableVisibility;
use crate::{Area, Color, InterfaceContext, Location};
use bevy_ecs::prelude::{Bundle, Component};

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
    #[bundle]
    pub location: Location<InterfaceContext>,
    pub color: Color,
    pub(crate) color_invert: ColorInvert,
    #[bundle]
    pub(crate) visibility: EnableVisibility,
    pub(crate) area: Area<InterfaceContext>,
}

impl Icon {
    pub fn new<P: Into<Location<InterfaceContext>>, C: Into<Color>>(
        key: IconKey,
        location: P,
        size: IconSize,
        color: C,
        secondary_color: IconSecondaryColor,
    ) -> Self {
        Self {
            secondary_color,
            size,
            key,
            location: location.into(),
            color: color.into(),
            color_invert: ColorInvert::off(),
            visibility: EnableVisibility::new(),
            area: Area::default(),
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
