use bevy_ecs::component::Component;
use bevy_ecs::prelude::Bundle;
use bytemuck::{Pod, Zeroable};
use compact_str::CompactString;

use crate::{Area, Color, EnableVisibility, InterfaceContext, Layer, Section};
use crate::grid::ResponsiveGridPoint;
use crate::icon::cache::{Cache, Difference};
use crate::icon::renderer::AreaAndLayer;

#[derive(Component, Copy, Clone, PartialEq)]
pub struct NegativeSpaceColor(pub Color);

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
            IconScale::Small => 16f32,
            IconScale::Medium => 20f32,
            IconScale::Large => 24f32,
            IconScale::Custom(dim) => *dim as f32,
        }
    }
}

#[derive(Bundle)]
pub struct Icon {
    id: IconId,
    responsive_grid_point: ResponsiveGridPoint,
    icon_scale: IconScale,
    layer: Layer,
    pos_color: Color,
    neg_color: NegativeSpaceColor,
    color_invert: ColorInvert,
    section: Section<InterfaceContext>,
    visibility: EnableVisibility,
    area_and_layer: AreaAndLayer,
    cache: Cache,
    difference: Difference,
}

impl Icon {
    pub fn new<
        Id: Into<IconId>,
        ResGridPnt: Into<ResponsiveGridPoint>,
        S: Into<IconScale>,
        L: Into<Layer>,
        C: Into<Color>,
    >(
        id: Id,
        gp: ResGridPnt,
        scale: S,
        layer: L,
        pos_color: C,
        neg_color: C,
    ) -> Self {
        Self {
            id: id.into(),
            responsive_grid_point: gp.into(),
            icon_scale: scale.into(),
            layer: layer.into(),
            pos_color: pos_color.into(),
            neg_color: NegativeSpaceColor(neg_color.into()),
            color_invert: ColorInvert::off(),
            section: Section::default(),
            visibility: EnableVisibility::default(),
            area_and_layer: AreaAndLayer::new(),
            cache: Cache::new(),
            difference: Difference::new(),
        }
    }
}

#[repr(C)]
#[derive(Component, Copy, Clone, Pod, Zeroable, Default, PartialEq)]
pub struct ColorInvert {
    pub signal: u32,
}

impl ColorInvert {
    pub fn on() -> Self {
        Self { signal: 1 }
    }
    pub fn off() -> Self {
        Self { signal: 0 }
    }
}

#[derive(Component, Clone, Hash, Eq, PartialEq)]
pub struct IconId(pub CompactString);

impl From<&'static str> for IconId {
    fn from(value: &'static str) -> Self {
        IconId(CompactString::new(value))
    }
}
