use bevy_ecs::component::Component;
use bevy_ecs::prelude::Bundle;
use compact_str::CompactString;

use crate::icon::cache::{Cache, Difference};
use crate::{Color, EnableVisibility, InterfaceContext, Layer, Section, Tag};

#[derive(Component, Copy, Clone)]
pub enum IconScale {
    Small,
    Medium,
    Large,
    Custom(u32),
    Asymmetrical((u32, u32)),
}

impl IconScale {
    pub fn width_px(&self) -> f32 {
        match &self {
            IconScale::Small => 13f32,
            IconScale::Medium => 17f32,
            IconScale::Large => 20f32,
            IconScale::Custom(dim) => *dim as f32,
            IconScale::Asymmetrical((w, h)) => *w as f32,
        }
    }
    pub fn height_px(&self) -> f32 {
        match &self {
            IconScale::Asymmetrical((w, h)) => *h as f32,
            _ => self.width_px(),
        }
    }
}

impl From<u32> for IconScale {
    fn from(value: u32) -> Self {
        match value {
            13u32 => IconScale::Small,
            17u32 => IconScale::Medium,
            20u32 => IconScale::Large,
            val => IconScale::Custom(val),
        }
    }
}
pub type IconTag = Tag<Icon>;
#[derive(Bundle)]
pub struct Icon {
    tag: IconTag,
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

#[derive(Component, Clone, Hash, Eq, PartialEq)]
pub struct IconId(pub CompactString);

impl From<&'static str> for IconId {
    fn from(value: &'static str) -> Self {
        IconId(CompactString::new(value))
    }
}
