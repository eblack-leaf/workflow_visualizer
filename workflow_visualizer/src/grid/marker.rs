use std::ops::{Add, Div, Mul, Neg, Sub};

/// Index of 8px alignment location
#[derive(Copy, Clone, PartialEq, Default)]
pub struct RawMarker(pub i32);
impl Neg for RawMarker {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl RawMarker {
    pub const PX: f32 = 4f32;
    pub fn to_pixel(self) -> f32 {
        self.0 as f32 * Self::PX
    }
    pub fn from_pixel_exclusive(value: f32) -> Self {
        Self((value / Self::PX).floor() as i32)
    }
    pub fn from_pixel_inclusive(value: f32) -> Self {
        Self((value / Self::PX).ceil() as i32)
    }
}

impl Add for RawMarker {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        RawMarker(self.0 + rhs.0)
    }
}
impl Sub for RawMarker {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        RawMarker(self.0 - rhs.0)
    }
}
impl Mul for RawMarker {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        RawMarker(self.0 * rhs.0)
    }
}
impl Div for RawMarker {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        RawMarker(self.0 / rhs.0)
    }
}
impl From<i32> for RawMarker {
    fn from(value: i32) -> Self {
        RawMarker(value)
    }
}

/// Number of markers to include in a logical group
pub struct RawMarkerGrouping(pub i32);

pub(crate) struct ColumnConfig {
    pub base: RawMarkerGrouping,
    pub extension: RawMarkerGrouping,
}

/// MarkerGrouping for deciding gutter size
pub(crate) struct GutterConfig {
    pub base: RawMarkerGrouping,
}

/// MarkerGrouping fro deciding row size
#[allow(unused)]
pub(crate) struct RowConfig {
    pub base: RawMarkerGrouping,
}
