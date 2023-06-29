use bevy_ecs::component::Component;
use bevy_ecs::prelude::Bundle;
use bytemuck::{Pod, Zeroable};
use compact_str::CompactString;

use crate::grid::ResponsiveGridPoint;
use crate::icon::cache::{Cache, Difference};
use crate::{Area, Color, EnableVisibility, InterfaceContext, Layer, Section};

#[derive(Component)]
pub enum IconScale {
    Small,
    Medium,
    Large,
    Custom(u32),
}

impl IconScale {
    pub fn px(&self) -> f32 {
        match &self {
            IconScale::Small => 13f32,
            IconScale::Medium => 17f32,
            IconScale::Large => 20f32,
            IconScale::Custom(dim) => *dim as f32,
        }
    }
}

#[derive(Bundle)]
pub struct Icon {
    id: IconId,
    icon_scale: IconScale,
    layer: Layer,
    pos_color: Color,
    section: Section<InterfaceContext>,
    visibility: EnableVisibility,
    cache: Cache,
    difference: Difference,
}

impl Icon {
    pub fn new<Id: Into<IconId>, S: Into<IconScale>, L: Into<Layer>, C: Into<Color>>(
        id: Id,
        scale: S,
        layer: L,
        pos_color: C,
    ) -> Self {
        Self {
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

#[derive(Component, Clone, Hash, Eq, PartialEq)]
pub struct IconId(pub CompactString);

impl From<&'static str> for IconId {
    fn from(value: &'static str) -> Self {
        IconId(CompactString::new(value))
    }
}
