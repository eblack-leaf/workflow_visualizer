use std::collections::HashMap;

use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::prelude::Resource;

use crate::icon::mesh::{ColorInvert, IconKey};
use crate::{Color, Location, UIView, Visibility};

#[derive(Component)]
pub struct Icon {
    pub secondary_color: Color,
}

impl Icon {
    pub fn new<C: Into<Color>>(secondary_color: C) -> Self {
        Self {
            secondary_color: secondary_color.into(),
        }
    }
}

#[derive(Bundle)]
pub struct IconBundle {
    pub icon: Icon,
    pub size: IconSize,
    pub key: IconKey,
    #[bundle]
    pub location: Location<UIView>,
    pub color: Color,
    pub(crate) color_invert: ColorInvert,
    pub(crate) visibility: Visibility,
}

impl IconBundle {
    pub fn new<P: Into<Location<UIView>>, C: Into<Color>>(
        icon: Icon,
        size: IconSize,
        key: IconKey,
        location: P,
        color: C,
    ) -> Self {
        Self {
            icon,
            size,
            key,
            location: location.into(),
            color: color.into(),
            color_invert: ColorInvert::off(),
            visibility: Visibility::new(),
        }
    }
}

#[derive(Component, Hash, Eq, PartialEq)]
pub enum IconSize {
    Small,
    Medium,
    Large,
}

#[derive(Resource)]
pub struct IconAreaGuide {
    pub guide: HashMap<IconSize, u32>,
}

impl IconAreaGuide {
    pub fn new() -> Self {
        Self {
            guide: HashMap::new(),
        }
    }
}

impl Default for IconAreaGuide {
    fn default() -> Self {
        let mut guide = Self::new();
        guide.guide.insert(IconSize::Small, 12);
        guide.guide.insert(IconSize::Medium, 15);
        guide.guide.insert(IconSize::Large, 22);
        guide
    }
}
