use std::collections::HashMap;

use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::prelude::Resource;

use crate::coord::Panel;
use crate::icon::mesh::IconKey;
use crate::{Color, Location, UIView};

#[derive(Component)]
pub struct Icon {}

#[derive(Bundle)]
pub struct IconBundle {
    pub icon: Icon,
    pub size: IconSize,
    pub key: IconKey,
    #[bundle]
    pub location: Location<UIView>,
    pub color: Color,
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
        guide.guide.insert(IconSize::Large, 18);
        guide
    }
}
