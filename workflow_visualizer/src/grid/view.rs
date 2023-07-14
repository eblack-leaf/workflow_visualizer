use std::collections::HashMap;

use crate::RawMarker;

/// Description of a Location on the Grid
#[derive(Copy, Clone)]
pub struct GridLocation {
    pub location: GridLocationDescriptor,
    pub offset: Option<GridLocationOffset>,
}

impl GridLocation {
    pub fn raw_offset(mut self, offset: i32) -> Self {
        if let Some(current_offset) = self.offset.as_mut() {
            current_offset.0 .0 += offset;
        } else {
            self.offset.replace(GridLocationOffset(offset.into()));
        }
        self
    }
    pub fn column_offset(mut self, offset: i32) -> Self {
        self.location.marker.0 += offset;
        self
    }
}

/// Pair of GridMarker and Bias to get an exact grid location
#[derive(Copy, Clone)]
pub struct GridLocationDescriptor {
    pub marker: GridMarker,
    pub bias: GridMarkerBias,
}

impl<T: Into<GridMarker>> From<(T, GridMarkerBias)> for GridLocation {
    fn from(value: (T, GridMarkerBias)) -> Self {
        GridLocation {
            location: GridLocationDescriptor {
                marker: value.0.into(),
                bias: value.1,
            },
            offset: None,
        }
    }
}

/// Beginning and End GridLocation grouping
#[derive(Copy, Clone)]
pub struct GridRange {
    pub begin: GridLocation,
    pub end: GridLocation,
}

impl<T: Into<GridLocation>> From<(T, T)> for GridRange {
    fn from(value: (T, T)) -> Self {
        GridRange {
            begin: value.0.into(),
            end: value.1.into(),
        }
    }
}

/// A GridRange for horizontal + vertical aspects
#[derive(Copy, Clone)]
pub struct GridView {
    pub horizontal: GridRange,
    pub vertical: GridRange,
}

impl<T: Into<GridRange>> From<(T, T)> for GridView {
    fn from(value: (T, T)) -> Self {
        GridView {
            horizontal: value.0.into(),
            vertical: value.1.into(),
        }
    }
}
/// Point in a Grid with x/y as GridLocations
#[derive(Copy, Clone)]
pub struct GridPoint {
    pub x: GridLocation,
    pub y: GridLocation,
}

impl<T: Into<GridLocation>> From<(T, T)> for GridPoint {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}

/// Logical index using groupings to get actual markers then to px
#[derive(Copy, Clone)]
pub struct GridMarker(pub i32);

impl From<i32> for GridMarker {
    fn from(value: i32) -> Self {
        GridMarker(value)
    }
}

/// Whether to attach to beginning/end of column
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum GridMarkerBias {
    Near,
    Far,
}

#[derive(Copy, Clone, PartialEq)]
pub struct GridLocationOffset(pub RawMarker);

pub struct PlacementReference {
    pub locations: HashMap<&'static str, GridLocation>,
    pub horizontal_ranges: HashMap<&'static str, GridRange>,
    pub vertical_ranges: HashMap<&'static str, GridRange>,
    pub points: HashMap<&'static str, GridPoint>,
}

impl PlacementReference {
    pub fn new() -> Self {
        Self {
            locations: HashMap::new(),
            horizontal_ranges: HashMap::new(),
            vertical_ranges: HashMap::new(),
            points: HashMap::new(),
        }
    }
    pub fn add_location<L: Into<GridLocation>>(&mut self, id: &'static str, location: L) {}
    pub fn add_horizontal_range<T: Into<&'static str>, R: Into<GridRange>>(
        &mut self,
        name: T,
        range: R,
    ) {}
    pub fn add_vertical_range<T: Into<&'static str>, R: Into<GridRange>>(
        &mut self,
        name: T,
        range: R,
    ) {}
    pub fn add_view<V: Into<GridView>, N: Into<&'static str>>(&mut self, name: N, view: V) {}
    pub fn view<T: Into<&'static str>>(&self, name: T) -> Option<GridView> {
        let name = name.into();
        if let Some(horiz) = self.horizontal_ranges.get(&name).copied() {
            if let Some(vert) = self.vertical_ranges.get(&name).copied() {
                return Some((horiz, vert).into());
            }
        }
        None
    }
    pub fn point<T: Into<&'static str>>(&self, name: T) -> Option<GridPoint> {
        self.points.get(&name.into()).copied()
    }
    pub fn location(&self, id: &'static str) -> Option<GridLocation> {
        self.locations.get(id).copied()
    }
    pub fn horizontal<N: Into<&'static str>>(&self, name: N) -> GridRange {
        self.horizontal_ranges
            .get(&name.into())
            .copied()
            .expect("no horizontal")
    }
    pub fn vertical<N: Into<&'static str>>(&self, name: N) -> GridRange {
        self.vertical_ranges
            .get(&name.into())
            .copied()
            .expect("no horizontal")
    }
}
