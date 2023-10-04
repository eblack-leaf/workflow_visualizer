use bevy_ecs::component::Component;
use crate::{Area, CoordinateUnit, InterfaceContext, NumericalContext, Position, Section};

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum Breakpoint {
    Mobile = 550,
    Tablet = 800,
    Desktop = 1050,
}
impl Breakpoint {
    pub fn gutter(&self) -> CoordinateUnit {
        match self {
            Breakpoint::Mobile => SnapGrid::GUTTER_BASE,
            Breakpoint::Tablet => SnapGrid::GUTTER_BASE * 1.5f32,
            Breakpoint::Desktop => SnapGrid::GUTTER_BASE * 3f32,
        }
    }
    pub fn segments(&self) -> i32 {
        match self {
            Breakpoint::Mobile => 12,
            Breakpoint::Tablet => 18,
            Breakpoint::Desktop => 24,
        }
    }
    pub fn value(&self) -> CoordinateUnit {
        (*self as i32) as f32
    }
    pub fn establish(width: CoordinateUnit) -> Self {
        return if width <= Self::Mobile.value() {
            Self::Mobile
        } else if width <= Self::Tablet.value() {
            Self::Tablet
        } else {
            Self::Desktop
        };
    }
}
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct GridMarker(pub i32);
#[derive(Copy, Clone)]
pub enum GridBias {
    Near,
    Far,
}
pub trait GridUnit {
    fn near(self) -> GridLocation;
    fn far(self) -> GridLocation;
}
impl GridUnit for i32 {
    fn near(self) -> GridLocation {
        GridLocation::new(GridMarker(self), GridBias::Near)
    }

    fn far(self) -> GridLocation {
        GridLocation::new(GridMarker(self), GridBias::Far)
    }
}
#[derive(Copy, Clone)]
pub struct GridLocation {
    pub marker: GridMarker,
    pub bias: GridBias,
}
impl GridLocation {
    pub fn new(marker: GridMarker, bias: GridBias) -> Self {
        Self {
            marker,
            bias,
        }
    }
}
#[derive(Copy, Clone)]
pub struct ResponsiveGridLocation {
    pub mobile: GridLocation,
    pub tablet: Option<GridLocation>,
    pub desktop: Option<GridLocation>,
}

impl ResponsiveGridLocation {
    pub fn new(mobile: GridLocation) -> Self {
        Self {
            mobile,
            tablet: None,
            desktop: None,
        }
    }
    pub fn with_tablet(mut self, location: GridLocation) -> Self {
        self.tablet.replace(location);
        self
    }
    pub fn with_desktop(mut self, location: GridLocation) -> Self {
        self.desktop.replace(location);
        self
    }

}

#[derive(Copy, Clone)]
pub struct ResponsiveGridRange {
    pub begin: ResponsiveGridLocation,
    pub end: ResponsiveGridLocation,
}
impl ResponsiveGridRange {
    pub fn new(begin: ResponsiveGridLocation, end: ResponsiveGridLocation) -> Self {
        Self {
            begin,
            end,
        }
    }
}
#[derive(Component, Copy, Clone)]
pub struct ResponsiveGridView {
    pub horizontal: ResponsiveGridRange,
    pub vertical: ResponsiveGridRange,
}
impl ResponsiveGridView {
    pub fn new(horizontal: ResponsiveGridRange, vertical: ResponsiveGridRange) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}
pub struct Column {
    pub content: CoordinateUnit,
    pub gutter: CoordinateUnit,
}
impl Column {
    pub fn new(width: CoordinateUnit, breakpoint: Breakpoint) -> Self {
        Self {
            content: (width - breakpoint.gutter() * (breakpoint.segments() + 1) as f32)
                / breakpoint.segments() as f32,
            gutter: breakpoint.gutter(),
        }
    }
}
pub struct Row {
    pub content: CoordinateUnit,
    pub gutter: CoordinateUnit,
}
impl Row {
    pub fn new(height: CoordinateUnit, breakpoint: Breakpoint) -> Self {
        Self {
            content: (height - breakpoint.gutter() * (breakpoint.segments() + 1) as f32)
                / breakpoint.segments() as f32,
            gutter: breakpoint.gutter(),
        }
    }
}

pub struct SnapGrid {
    pub horizontal_breakpoint: Breakpoint,
    pub vertical_breakpoint: Breakpoint,
    pub column: Column,
    pub row: Row,
}
impl SnapGrid {
    pub const GUTTER_BASE: CoordinateUnit = 2f32;
    pub fn new(area: Area<NumericalContext>) -> Self {
        let horizontal_breakpoint = Breakpoint::establish(area.width);
        let vertical_breakpoint = Breakpoint::establish(area.height);
        Self {
            horizontal_breakpoint,
            vertical_breakpoint,
            column: Column::new(area.width, horizontal_breakpoint),
            row: Row::new(area.height, vertical_breakpoint),
        }
    }
    pub fn view_coordinates(&self, view: ResponsiveGridView) -> Section<InterfaceContext> {
        todo!()
    }
    pub fn range_coordinates(&self, range: ResponsiveGridRange) -> CoordinateUnit {
        todo!()
    }
    pub fn location_coordinates(&self, location: ResponsiveGridLocation) -> Position<InterfaceContext> {
        todo!()
    }
}
