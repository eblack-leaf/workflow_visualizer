use crate::{CoordinateUnit, SnapGrid};

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum Breakpoint {
    Mobile = 650,
    Tablet = 950,
    Desktop = 1250,
    Workstation = 1550,
}

impl Breakpoint {
    pub fn gutter(&self) -> CoordinateUnit {
        match self {
            Breakpoint::Mobile => SnapGrid::GUTTER_BASE,
            Breakpoint::Tablet => SnapGrid::GUTTER_BASE * 1.25f32,
            Breakpoint::Desktop => SnapGrid::GUTTER_BASE * 1.75f32,
            Breakpoint::Workstation => SnapGrid::GUTTER_BASE * 2f32,
        }
    }
    pub fn segments(&self) -> i32 {
        match self {
            Breakpoint::Mobile => 15,
            Breakpoint::Tablet => 17,
            Breakpoint::Desktop => 19,
            Breakpoint::Workstation => 20,
        }
    }
    pub fn value(&self) -> CoordinateUnit {
        (*self as i32) as f32
    }
    pub fn establish(dimension: CoordinateUnit) -> Self {
        if dimension <= Self::Mobile.value() {
            Self::Mobile
        } else if dimension <= Self::Tablet.value() {
            Self::Tablet
        } else if dimension <= Self::Desktop.value() {
            Self::Desktop
        } else {
            Self::Workstation
        }
    }
}
