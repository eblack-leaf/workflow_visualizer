use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;

use crate::{Color, Location, UIView};
use crate::coord::Panel;
use crate::icon::IconKey;

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
    pub fn new<P: Into<Location<UIView>>, C: Into<Color>>(icon: Icon, size: IconSize, key: IconKey, location: P, color: C) -> Self {
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
