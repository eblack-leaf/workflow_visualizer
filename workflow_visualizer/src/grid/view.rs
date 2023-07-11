use crate::{EntityName, RawMarker};
use std::collections::HashMap;

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

#[derive(Copy, Clone)]
pub struct GridViewBuilder {
    horizontal: Option<GridRange>,
    vertical: Option<GridRange>,
}

impl GridViewBuilder {
    pub fn new() -> Self {
        Self {
            horizontal: None,
            vertical: None,
        }
    }
    pub fn with_horizontal<T: Into<GridRange>>(mut self, range: T) -> Self {
        self.horizontal.replace(range.into());
        self
    }
    pub fn with_vertical<T: Into<GridRange>>(mut self, range: T) -> Self {
        self.vertical.replace(range.into());
        self
    }
    pub fn horizontal(&self) -> Option<GridRange> {
        self.horizontal
    }
    pub fn vertical(&self) -> Option<GridRange> {
        self.vertical
    }
    pub fn build(&mut self) -> Option<GridView> {
        if let Some(hor) = self.horizontal {
            if let Some(ver) = self.vertical {
                return Some(GridView::from((hor, ver)));
            }
        }
        None
    }
}
impl<T: Into<GridRange>> From<(T, T)> for GridViewBuilder {
    fn from(value: (T, T)) -> Self {
        GridViewBuilder {
            horizontal: Some(value.0.into()),
            vertical: Some(value.1.into()),
        }
    }
}
pub struct PlacementBuilder {
    pub views: HashMap<EntityName, GridViewBuilder>,
    pub points: HashMap<EntityName, GridPoint>,
}
impl PlacementBuilder {
    pub fn new() -> Self {
        Self {
            views: HashMap::new(),
            points: HashMap::new(),
        }
    }
    pub fn add<T: Into<EntityName>, G: Into<GridViewBuilder>>(&mut self, name: T, view: G) {
        self.views.insert(name.into(), view.into());
    }
    pub fn add_point<T: Into<EntityName>, G: Into<GridPoint>>(&mut self, name: T, point: G) {
        self.points.insert(name.into(), point.into());
    }
    pub fn view_get_mut<T: Into<EntityName>>(&mut self, name: T) -> &mut GridViewBuilder {
        self.views.get_mut(&name.into()).expect("no view")
    }
    pub fn view_get<T: Into<EntityName>>(&self, name: T) -> GridViewBuilder {
        self.views.get(&name.into()).copied().expect("no view")
    }
    pub fn point_get<T: Into<EntityName>>(&self, name: T) -> GridPoint {
        self.points.get(&name.into()).copied().expect("no point")
    }
}
