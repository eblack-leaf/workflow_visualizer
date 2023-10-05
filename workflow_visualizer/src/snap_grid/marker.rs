#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct GridMarker(pub i32);

#[derive(Copy, Clone, Eq, PartialEq)]
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
        Self { marker, bias }
    }
}

#[derive(Copy, Clone)]
pub struct GridPoint {
    pub x: GridLocation,
    pub y: GridLocation,
}

impl GridPoint {
    pub fn new(x: GridLocation, y: GridLocation) -> Self {
        Self { x, y }
    }
}

pub struct GridRange {
    pub begin: GridLocation,
    pub end: GridLocation,
}

impl GridRange {
    pub fn new(begin: GridLocation, end: GridLocation) -> Self {
        Self { begin, end }
    }
}

pub struct GridView {
    pub horizontal: GridRange,
    pub vertical: GridRange,
}

impl GridView {
    pub fn new(horizontal: GridRange, vertical: GridRange) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}

#[derive(Copy, Clone)]
pub enum GridDirection {
    Horizontal,
    Vertical,
}
