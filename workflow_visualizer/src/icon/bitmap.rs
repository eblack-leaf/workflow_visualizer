use std::collections::HashMap;

use bevy_ecs::component::Component;
use bevy_ecs::prelude::Resource;
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use crate::icon::component::IconId;
use crate::TextureCoordinates;

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
    Something,
    Edit,
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
            BundledIcon::Something => Self::new(Self::read_icon_file(include_str!(
                "bundled_icons/something.icon"
            ))),
            BundledIcon::Edit => Self::new(Self::read_icon_file(include_str!(
                "bundled_icons/edit.icon"
            ))),
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

#[derive(Component, Clone)]
pub struct IconBitmapRequest {
    pub id: IconId,
    pub bitmap: IconBitmap,
}

impl<I: Into<IconId>, IB: Into<IconBitmap>> From<(I, IB)> for IconBitmapRequest {
    fn from(value: (I, IB)) -> Self {
        Self {
            id: value.0.into(),
            bitmap: value.1.into(),
        }
    }
}

#[derive(Resource)]
pub(crate) struct IconBitmapLayout {
    pub(crate) bitmap_locations: HashMap<IconId, TextureCoordinates>,
}

impl IconBitmapLayout {
    pub(crate) fn new() -> Self {
        Self {
            bitmap_locations: HashMap::new(),
        }
    }
}

pub(crate) const ICON_BITMAP_DIMENSION: u32 = 20;

#[cfg(test)]
#[test]
fn sample() {
    // let value = (0,0,0);
    // let string =  serde_json::to_string(&value).unwrap();
    println!("{:?}", 1 << 9);
}
