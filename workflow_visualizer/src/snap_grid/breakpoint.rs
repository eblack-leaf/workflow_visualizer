use crate::{CoordinateUnit, SnapGrid};

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum Breakpoint {
    Mobile = 586,
    Tablet = 876,
    Desktop = 1186,
}

impl Breakpoint {
    pub fn gutter(&self) -> CoordinateUnit {
        match self {
            Breakpoint::Mobile => SnapGrid::GUTTER_BASE,
            Breakpoint::Tablet => SnapGrid::GUTTER_BASE * 2f32,
            Breakpoint::Desktop => SnapGrid::GUTTER_BASE * 3f32,
        }
    }
    pub fn segments(&self) -> i32 {
        match self {
            Breakpoint::Mobile => 12,
            Breakpoint::Tablet => 16,
            Breakpoint::Desktop => 20,
        }
    }
    pub fn value(&self) -> CoordinateUnit {
        (*self as i32) as f32
    }
    pub fn establish(width: CoordinateUnit) -> Self {
        if width <= Self::Mobile.value() {
            Self::Mobile
        } else if width <= Self::Tablet.value() {
            Self::Tablet
        } else {
            Self::Desktop
        }
    }
}
