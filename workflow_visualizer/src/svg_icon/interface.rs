use crate::{
    Area, Color, Disabled, EnableVisibility, InterfaceContext, Layer, Position, ResourceHandle,
    Section, Tag, Visibility,
};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::query::{Changed, Without};
use bevy_ecs::system::Query;
use serde::{Deserialize, Serialize};

#[derive(Bundle)]
pub struct SvgIcon {
    pub section: Section<InterfaceContext>,
    pub color: Color,
    pub layer: Layer,
    pub handle: ResourceHandle,
    tag: SvgTag,
    enable_visibility: EnableVisibility,
    cache: Cache,
    difference: Difference,
    pub scale: SvgIconScale,
}
#[derive(Component, Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
pub enum SvgIconScale {
    Custom(u32),
    Asymmetrical((u32, u32)),
}
impl From<u32> for SvgIconScale {
    fn from(value: u32) -> Self {
        Self::Custom(value)
    }
}
impl SvgIconScale {
    pub fn as_area(&self) -> Area<InterfaceContext> {
        match &self {
            SvgIconScale::Custom(dim) => Area::from((*dim, *dim)),
            SvgIconScale::Asymmetrical((width, height)) => Area::from((*width, *height)),
        }
    }
    pub fn width(&self) -> f32 {
        self.as_area().width
    }
    pub fn height(&self) -> f32 {
        self.as_area().height
    }
}
pub(crate) fn scale_change(
    mut svgs: Query<(&SvgIconScale, &mut Area<InterfaceContext>), Changed<SvgIconScale>>,
) {
    for (scale, mut area) in svgs.iter_mut() {
        area.width = scale.width();
        area.height = scale.height();
    }
}
impl SvgIcon {
    pub fn new<H: Into<ResourceHandle>, L: Into<Layer>, C: Into<Color>, S: Into<SvgIconScale>>(
        handle: H,
        scale: S,
        layer: L,
        color: C,
    ) -> Self {
        Self {
            section: Section::default(),
            color: color.into(),
            layer: layer.into(),
            handle: handle.into(),
            tag: SvgTag::new(),
            enable_visibility: EnableVisibility::default(),
            cache: Default::default(),
            difference: Default::default(),
            scale: scale.into(),
        }
    }
}
pub type SvgTag = Tag<SvgIcon>;
#[derive(Default, Copy, Clone)]
pub(crate) struct Attributes {
    pub(crate) svg: Option<ResourceHandle>,
    pub(crate) position: Option<Position<InterfaceContext>>,
    pub(crate) area: Option<Area<InterfaceContext>>,
    pub(crate) layer: Option<Layer>,
    pub(crate) color: Option<Color>,
}
#[derive(Default, Component)]
pub(crate) struct Cache {
    pub(crate) attributes: Attributes,
}
#[derive(Default, Component)]
pub(crate) struct Difference {
    pub(crate) attributes: Attributes,
}
pub(crate) fn management(
    mut svgs: Query<
        (
            &ResourceHandle,
            &Position<InterfaceContext>,
            &Area<InterfaceContext>,
            &Layer,
            &Color,
            &Visibility,
            &mut Cache,
            &mut Difference,
        ),
        (Changed<Visibility>, Without<Disabled>),
    >,
) {
    for (handle, pos, area, layer, color, vis, mut cache, mut diff) in svgs.iter_mut() {
        if vis.visible() {
            let mut attributes = Attributes::default();
            attributes.svg.replace(*handle);
            attributes.position.replace(*pos);
            attributes.area.replace(*area);
            attributes.layer.replace(*layer);
            attributes.color.replace(*color);
            cache.attributes = attributes;
            diff.attributes = attributes;
        }
    }
}
pub(crate) fn svg_diff(
    mut svgs: Query<(&ResourceHandle, &mut Cache, &mut Difference), Changed<ResourceHandle>>,
) {
    for (handle, mut cache, mut difference) in svgs.iter_mut() {
        if let Some(cached) = cache.attributes.svg.as_ref() {
            if *cached != *handle {
                difference.attributes.svg.replace(*handle);
            }
        }
        cache.attributes.svg.replace(*handle);
    }
}
pub(crate) fn position_diff(
    mut svgs: Query<
        (&Position<InterfaceContext>, &mut Cache, &mut Difference),
        Changed<Position<InterfaceContext>>,
    >,
) {
    for (handle, mut cache, mut difference) in svgs.iter_mut() {
        if let Some(cached) = cache.attributes.position.as_ref() {
            if *cached != *handle {
                difference.attributes.position.replace(*handle);
            }
        }
        cache.attributes.position.replace(*handle);
    }
}
pub(crate) fn area_diff(
    mut svgs: Query<
        (&Area<InterfaceContext>, &mut Cache, &mut Difference),
        Changed<Area<InterfaceContext>>,
    >,
) {
    for (handle, mut cache, mut difference) in svgs.iter_mut() {
        if let Some(cached) = cache.attributes.area.as_ref() {
            if *cached != *handle {
                difference.attributes.area.replace(*handle);
            }
        }
        cache.attributes.area.replace(*handle);
    }
}
pub(crate) fn layer_diff(mut svgs: Query<(&Layer, &mut Cache, &mut Difference), Changed<Layer>>) {
    for (handle, mut cache, mut difference) in svgs.iter_mut() {
        if let Some(cached) = cache.attributes.layer.as_ref() {
            if *cached != *handle {
                difference.attributes.layer.replace(*handle);
            }
        }
        cache.attributes.layer.replace(*handle);
    }
}
pub(crate) fn color_diff(mut svgs: Query<(&Color, &mut Cache, &mut Difference), Changed<Color>>) {
    for (handle, mut cache, mut difference) in svgs.iter_mut() {
        if let Some(cached) = cache.attributes.color.as_ref() {
            if *cached != *handle {
                difference.attributes.color.replace(*handle);
            }
        }
        cache.attributes.color.replace(*handle);
    }
}
