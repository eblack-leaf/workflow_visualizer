use crate::images::{Cache, Difference};
use crate::{
    Area, Color, EnableVisibility, ImageData, ImageFade, ImageRequest, ImageTag, InterfaceContext,
    Layer, ResourceHandle, Section, Tag,
};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
pub use bundled::BundledIcon;
use serde::{Deserialize, Serialize};

mod bundled;
pub type IconRequest = ImageRequest;
pub type IconTag = Tag<Icon>;
pub type IconData = ImageData;
#[derive(Bundle)]
pub struct Icon {
    section: Section<InterfaceContext>,
    layer: Layer,
    visibility: EnableVisibility,
    handle: ResourceHandle,
    fade: ImageFade,
    cache: Cache,
    difference: Difference,
    tag: ImageTag,
    image_icon_tag: IconTag,
    scale: IconScale,
    color: Color,
}

impl Icon {
    pub fn new<RH: Into<ResourceHandle>, IS: Into<IconScale>, L: Into<Layer>, C: Into<Color>>(
        handle: RH,
        scale: IS,
        layer: L,
        color: C,
    ) -> Self {
        Self {
            handle: handle.into(),
            scale: scale.into(),
            layer: layer.into(),
            color: color.into(),
            fade: ImageFade::OPAQUE,
            cache: Cache::default(),
            difference: Difference::default(),
            tag: ImageTag::new(),
            image_icon_tag: IconTag::new(),
            visibility: EnableVisibility::new(),
            section: Section::default(),
        }
    }
    pub(crate) const INVALID_COLOR: Color = Color {
        red: -1.0,
        green: -1.0,
        blue: -1.0,
        alpha: -1.0,
    };
}

#[derive(Component, Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
pub enum IconScale {
    Symmetrical(u32),
    Asymmetrical((u32, u32)),
}

impl From<u32> for IconScale {
    fn from(value: u32) -> Self {
        Self::Symmetrical(value)
    }
}

impl IconScale {
    pub fn as_area(&self) -> Area<InterfaceContext> {
        match &self {
            IconScale::Symmetrical(dim) => Area::from((*dim, *dim)),
            IconScale::Asymmetrical((width, height)) => Area::from((*width, *height)),
        }
    }
    pub fn width(&self) -> f32 {
        self.as_area().width
    }
    pub fn height(&self) -> f32 {
        self.as_area().height
    }
}
