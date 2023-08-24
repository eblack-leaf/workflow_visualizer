use std::collections::HashMap;

use crate::{PathView, RawMarker, ResponsiveGridView};

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
impl GridView {
    pub fn all_same(self) -> ResponsiveGridView {
        ResponsiveGridView::all_same(self)
    }
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
pub type RelativePlacementKey = u32;
pub struct RelativePlacer {
    pub anchor: GridPoint,
    pub relative_offsets: HashMap<RelativePlacementKey, RawMarker>,
}
impl RelativePlacer {
    pub fn new<GP: Into<GridPoint>>(anchor: GP) -> Self {
        Self {
            anchor: anchor.into(),
            relative_offsets: HashMap::new(),
        }
    }
    pub fn add<R: Into<RawMarker>>(&mut self, key: RelativePlacementKey, marker: R) {
        self.relative_offsets.insert(key, marker.into());
    }
    pub fn get(&self, key: RelativePlacementKey) -> RawMarker {
        self.relative_offsets.get(&key).copied().expect("RelativePlacer::get")
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
pub type PlacementKey = u32;
#[derive(Default, Clone)]
pub struct Placement {
    pub(crate) views: HashMap<PlacementKey, GridView>,
    pub(crate) points: HashMap<PlacementKey, GridPoint>,
}
impl Placement {
    pub fn add_view<V: Into<GridView>>(&mut self, key: PlacementKey, view: V) {
        self.views.insert(key, view.into());
    }
    pub fn add_point<P: Into<GridPoint>>(&mut self, key: PlacementKey, point: P) {
        self.points.insert(key, point.into());
    }
    pub fn get_view(&self, key: PlacementKey) -> GridView {
        self.views.get(&key).copied().expect("Placement::get_view")
    }
    pub fn get_point(&self, key: PlacementKey) -> GridPoint {
        self.points
            .get(&key)
            .copied()
            .expect("Placement::get_point")
    }
}
