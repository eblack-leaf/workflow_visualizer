use crate::snap_grid::Breakpoint;
use crate::{GridLocation, GridPoint, GridRange, GridView};
use bevy_ecs::component::Component;
#[derive(Copy, Clone)]
pub struct ResponsiveUnit<T: Copy + Clone> {
    pub mobile: T,
    pub tablet: Option<T>,
    pub desktop: Option<T>,
    pub workstation: Option<T>,
}
impl<T: Copy + Clone> ResponsiveUnit<T> {
    pub fn new(mobile: T) -> Self {
        Self {
            mobile,
            tablet: None,
            desktop: None,
            workstation: None,
        }
    }
    pub fn current(&self, breakpoint: Breakpoint) -> T {
        match breakpoint {
            Breakpoint::Mobile => self.mobile,
            Breakpoint::Tablet => self.tablet.unwrap_or(self.mobile),
            Breakpoint::Desktop => self.desktop.unwrap_or(self.tablet.unwrap_or(self.mobile)),
            Breakpoint::Workstation => self
                .workstation
                .unwrap_or(self.desktop.unwrap_or(self.tablet.unwrap_or(self.mobile))),
        }
    }
    pub fn with_tablet(mut self, unit: T) -> Self {
        self.tablet.replace(unit);
        self
    }
    pub fn with_desktop(mut self, unit: T) -> Self {
        self.desktop.replace(unit);
        self
    }
    pub fn with_workstation(mut self, unit: T) -> Self {
        self.workstation.replace(unit);
        self
    }
}
pub type ResponsiveGridLocation = ResponsiveUnit<GridLocation>;
#[derive(Component, Copy, Clone)]
pub struct ResponsiveGridPoint {
    pub x: ResponsiveGridLocation,
    pub y: ResponsiveGridLocation,
}

impl ResponsiveGridPoint {
    pub fn new(x: ResponsiveGridLocation, y: ResponsiveGridLocation) -> Self {
        Self { x, y }
    }
    pub fn current(&self, breakpoint: Breakpoint) -> GridPoint {
        let x = self.x.current(breakpoint);
        let y = self.y.current(breakpoint);
        GridPoint::new(x, y)
    }
}

#[derive(Copy, Clone)]
pub struct ResponsiveGridRange {
    pub begin: ResponsiveGridLocation,
    pub end: ResponsiveGridLocation,
}

impl ResponsiveGridRange {
    pub fn new(begin: ResponsiveGridLocation, end: ResponsiveGridLocation) -> Self {
        Self { begin, end }
    }
    pub fn current(&self, breakpoint: Breakpoint) -> GridRange {
        GridRange::new(self.begin.current(breakpoint), self.end.current(breakpoint))
    }
}

#[derive(Component, Copy, Clone)]
pub struct ResponsiveGridView {
    pub horizontal: ResponsiveGridRange,
    pub vertical: ResponsiveGridRange,
}

impl ResponsiveGridView {
    pub fn current(&self, horizontal_breakpoint: Breakpoint) -> GridView {
        GridView::new(
            self.horizontal.current(horizontal_breakpoint),
            self.vertical.current(horizontal_breakpoint),
        )
    }
    pub fn new(horizontal: ResponsiveGridRange, vertical: ResponsiveGridRange) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}
