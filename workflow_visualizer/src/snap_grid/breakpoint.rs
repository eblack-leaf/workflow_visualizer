use crate::{CoordinateUnit, SnapGrid};

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum Breakpoint {
    Mobile = 600,
    Tablet = 800,
    Desktop = 1000,
    Workstation = 1200,
}

impl Breakpoint {
    pub fn gutter(&self) -> CoordinateUnit {
        match self {
            Breakpoint::Mobile => SnapGrid::GUTTER_BASE,
            Breakpoint::Tablet => SnapGrid::GUTTER_BASE * 1.5f32,
            Breakpoint::Desktop => SnapGrid::GUTTER_BASE * 2f32,
            Breakpoint::Workstation => SnapGrid::GUTTER_BASE * 3f32,
        }
    }
    pub fn segments(&self) -> i32 {
        match self {
            Breakpoint::Mobile => 16,
            Breakpoint::Tablet => 20,
            Breakpoint::Desktop => 24,
            Breakpoint::Workstation => 28,
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
        } else if width <= Self::Desktop.value() {
            Self::Desktop
        } else {
            Self::Workstation
        }
    }
}
