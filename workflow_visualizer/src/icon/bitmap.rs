use std::collections::HashMap;

use crate::{ResourceHandle, TextureCoordinates};
use bevy_ecs::component::Component;
use bevy_ecs::prelude::{Commands, Entity, Query, Resource};
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(
    bytemuck::Pod,
    bytemuck::Zeroable,
    Copy,
    Clone,
    Default,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
)]
pub struct IconPixelData {
    pub data: u8,
}

impl IconPixelData {
    pub const FULL_COVERAGE: u8 = 255u8;
    pub const NO_COVERAGE: u8 = 0u8;
    pub fn new<T: Into<Self>>(data: T) -> Self {
        data.into()
    }
}

impl From<u8> for IconPixelData {
    fn from(value: u8) -> Self {
        IconPixelData { data: value }
    }
}

#[derive(Clone)]
pub struct IconBitmap {
    pub(crate) data: Vec<IconPixelData>,
}

pub enum BundledIcon {
    Square,
    Edit,
    Add,
    ArrowRight,
    ArrowLeft,
    Run,
    Delete,
    Generate,
    At,
}

impl IconBitmap {
    pub fn new<T: Into<IconPixelData>>(mut data: Vec<T>) -> Self {
        assert_eq!(
            data.len() as u32,
            ICON_BITMAP_DIMENSION * ICON_BITMAP_DIMENSION
        );
        Self {
            data: data
                .drain(..)
                .map(|d| d.into())
                .collect::<Vec<IconPixelData>>(),
        }
    }
    pub fn bundled(icon: BundledIcon) -> Self {
        match icon {
            BundledIcon::Square => Self::new(Self::read_icon_file(include_str!(
                "bundled_icons/square.icon"
            ))),
            BundledIcon::Edit => Self::new(Self::read_icon_file(include_str!(
                "bundled_icons/edit_alt.icon"
            ))),
            BundledIcon::Add => {
                Self::new(Self::read_icon_file(include_str!("bundled_icons/add.icon")))
            }
            BundledIcon::ArrowRight => Self::new(Self::read_icon_file(include_str!(
                "bundled_icons/page_right.icon"
            ))),
            BundledIcon::ArrowLeft => Self::new(Self::read_icon_file(include_str!(
                "bundled_icons/page_left.icon"
            ))),
            BundledIcon::Run => {
                Self::new(Self::read_icon_file(include_str!("bundled_icons/run.icon")))
            }
            BundledIcon::Delete => Self::new(Self::read_icon_file(include_str!(
                "bundled_icons/delete.icon"
            ))),
            BundledIcon::Generate => Self::new(Self::read_icon_file(include_str!(
                "bundled_icons/generate.icon"
            ))),
            BundledIcon::At => {
                Self::new(Self::read_icon_file(include_str!("bundled_icons/at.icon")))
            }
        }
    }
    fn read_icon_file(file: &str) -> Vec<IconPixelData> {
        serde_json::from_str::<Vec<u8>>(file)
            .expect("file parsing")
            .drain(..)
            .map(|d| d.into())
            .collect()
    }
}

#[derive(Resource, Default)]
pub(crate) struct IconBitmapRequestManager {
    pub(crate) requests: Vec<IconBitmapRequest>,
}

impl IconBitmapRequestManager {
    pub(crate) fn add<R: Into<IconBitmapRequest>>(&mut self, request: R) {
        self.requests.push(request.into());
    }
}

pub(crate) fn cleanup_requests(requests: Query<(Entity, &IconBitmapRequest)>, mut cmd: Commands) {
    for (entity, _) in requests.iter() {
        cmd.entity(entity).despawn();
    }
}
#[derive(Component, Clone)]
pub struct IconBitmapRequest {
    pub id: ResourceHandle,
    pub bitmap: Option<IconBitmap>,
}

impl<I: Into<ResourceHandle>, IB: Into<IconBitmap>> From<(I, IB)> for IconBitmapRequest {
    fn from(value: (I, IB)) -> Self {
        Self {
            id: value.0.into(),
            bitmap: Some(value.1.into()),
        }
    }
}

pub(crate) struct IconBitmapLayout {
    pub(crate) bitmap_locations: HashMap<ResourceHandle, TextureCoordinates>,
}

impl IconBitmapLayout {
    pub(crate) fn new() -> Self {
        Self {
            bitmap_locations: HashMap::new(),
        }
    }
}

pub const ICON_BITMAP_DIMENSION: u32 = 20;
