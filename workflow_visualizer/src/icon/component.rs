use bevy_ecs::component::Component;
use bevy_ecs::prelude::Bundle;

use crate::icon::cache::{Cache, Difference};
use crate::{Color, EnableVisibility, IconScale, InterfaceContext, Layer, Section, Tag};

pub type IconTag = Tag<Icon>;
#[derive(Bundle)]
pub struct Icon {
    tag: IconTag,
    id: IconHandle,
    icon_scale: IconScale,
    layer: Layer,
    pos_color: Color,
    section: Section<InterfaceContext>,
    visibility: EnableVisibility,
    cache: Cache,
    difference: Difference,
}

impl Icon {
    pub fn new<Id: Into<IconHandle>, S: Into<IconScale>, L: Into<Layer>, C: Into<Color>>(
        id: Id,
        scale: S,
        layer: L,
        pos_color: C,
    ) -> Self {
        Self {
            tag: IconTag::new(),
            id: id.into(),
            icon_scale: scale.into(),
            layer: layer.into(),
            pos_color: pos_color.into(),
            section: Section::default(),
            visibility: EnableVisibility::default(),
            cache: Cache::new(),
            difference: Difference::new(),
        }
    }
}

#[derive(Component, Copy, Clone, Hash, Eq, PartialEq)]
pub struct IconHandle(pub i32);

impl From<i32> for IconHandle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
