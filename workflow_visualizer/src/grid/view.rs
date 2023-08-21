use std::collections::HashMap;

use crate::{PathView, RawMarker};

/// Description of a Location on the Grid
#[derive(Copy, Clone)]
pub struct GridLocation {
    pub location: GridLocationDescriptor,
    pub offset: Option<GridLocationOffset>,
}

impl GridLocation {
    pub fn raw_offset<RM: Into<RawMarker>>(mut self, offset: RM) -> Self {
        let offset = offset.into();
        if let Some(current_offset) = self.offset.as_mut() {
            current_offset.0 .0 += offset.0;
        } else {
            self.offset.replace(GridLocationOffset(offset));
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

pub struct RelativePlacement {
    pub anchor: GridPoint,
    pub relative_offsets: HashMap<i32, RawMarker>,
}
impl RelativePlacement {
    pub fn new<GP: Into<GridPoint>>(anchor: GP) -> Self {
        Self {
            anchor: anchor.into(),
            relative_offsets: HashMap::new(),
        }
    }
    pub fn add<R: Into<RawMarker>>(&mut self, key: i32, marker: R) {
        self.relative_offsets.insert(key, marker.into());
    }
    pub fn view<R: Into<RawMarker>>(&self, hb: R, he: R, vb: R, ve: R) -> GridView {
        (
            (self.anchor.x.raw_offset(hb), self.anchor.x.raw_offset(he)),
            (self.anchor.y.raw_offset(vb), self.anchor.y.raw_offset(ve)),
        )
            .into()
    }
    pub fn point<R: Into<RawMarker>>(&self, hb: R, vb: R) -> GridPoint {
        (self.anchor.x.raw_offset(hb), self.anchor.y.raw_offset(vb)).into()
    }
}
