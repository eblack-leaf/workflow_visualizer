use crate::snap_grid::Breakpoint;
use crate::{GridLocation, GridPoint, GridRange, GridView};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct ResponsiveGridLocation {
    pub mobile: GridLocation,
    pub tablet: Option<GridLocation>,
    pub desktop: Option<GridLocation>,
}

#[derive(Component, Copy, Clone)]
pub struct ResponsiveGridPoint {
    pub x: ResponsiveGridLocation,
    pub y: ResponsiveGridLocation,
}

impl ResponsiveGridPoint {
    pub fn new(x: ResponsiveGridLocation, y: ResponsiveGridLocation) -> Self {
        Self { x, y }
    }
    pub fn current(&self, horizontal: Breakpoint, vertical: Breakpoint) -> GridPoint {
        let x = self.x.current(horizontal);
        let y = self.y.current(vertical);
        GridPoint::new(x, y)
    }
}

impl ResponsiveGridLocation {
    pub fn new(mobile: GridLocation) -> Self {
        Self {
            mobile,
            tablet: None,
            desktop: None,
        }
    }
    pub fn current(&self, breakpoint: Breakpoint) -> GridLocation {
        match breakpoint {
            Breakpoint::Mobile => self.mobile,
            Breakpoint::Tablet => self.tablet.unwrap_or(self.mobile),
            Breakpoint::Desktop => self.desktop.unwrap_or(self.tablet.unwrap_or(self.mobile)),
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
    pub fn current(
        &self,
        horizontal_breakpoint: Breakpoint,
        vertical_breakpoint: Breakpoint,
    ) -> GridView {
        GridView::new(
            self.horizontal.current(horizontal_breakpoint),
            self.vertical.current(vertical_breakpoint),
        )
    }
    pub fn new(horizontal: ResponsiveGridRange, vertical: ResponsiveGridRange) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}

pub enum ResponsiveGridPlacementDescriptor {
    Location(ResponsiveGridLocation),
    View(ResponsiveGridView),
}

pub struct GridPlacer {
    pub placements: HashMap<Entity, ResponsiveGridPlacementDescriptor>,
}

impl GridPlacer {
    pub fn new() -> Self {
        Self {
            placements: HashMap::new(),
        }
    }
    pub fn add(&mut self, _placement: ResponsiveGridPlacementDescriptor) {
        todo!()
    }
}
