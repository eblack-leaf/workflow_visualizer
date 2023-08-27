use std::collections::HashMap;
use std::hash::Hash;

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
#[derive(Copy, Clone, Default)]
pub struct ReferenceView {
    pub top: Option<RawMarker>,
    pub left: Option<RawMarker>,
    pub right: Option<RawMarker>,
    pub bottom: Option<RawMarker>,
}
impl ReferenceView {
    pub fn new() -> Self {
        Self {
            top: None,
            left: None,
            right: None,
            bottom: None,
        }
    }
    pub fn top(&self) -> RawMarker {
        self.top.expect("top")
    }
    pub fn left(&self) -> RawMarker {
        self.left.expect("left")
    }
    pub fn right(&self) -> RawMarker {
        self.right.expect("right")
    }
    pub fn bottom(&self) -> RawMarker {
        self.bottom.expect("bottom")
    }
    pub fn with_top<R: Into<RawMarker>>(mut self, top: R) -> Self {
        self.top.replace(top.into());
        self
    }
    pub fn with_left<R: Into<RawMarker>>(mut self, left: R) -> Self {
        self.left.replace(left.into());
        self
    }
    pub fn with_right<R: Into<RawMarker>>(mut self, right: R) -> Self {
        self.right.replace(right.into());
        self
    }
    pub fn with_bottom<R: Into<RawMarker>>(mut self, bottom: R) -> Self {
        self.bottom.replace(bottom.into());
        self
    }
}
#[derive(Copy, Clone, Default)]
pub struct ReferencePoint {
    pub x: Option<RawMarker>,
    pub y: Option<RawMarker>,
}
impl ReferencePoint {
    pub fn new() -> Self {
        Self { x: None, y: None }
    }
    pub fn x(&self) -> RawMarker {
        self.x.expect("x")
    }
    pub fn y(&self) -> RawMarker {
        self.y.expect("y")
    }
    pub fn with_x<R: Into<RawMarker>>(mut self, x: R) -> Self {
        self.x.replace(x.into());
        self
    }
    pub fn with_y<R: Into<RawMarker>>(mut self, y: R) -> Self {
        self.y.replace(y.into());
        self
    }
}
pub struct Placer<PlacementKey> {
    pub anchor: GridPoint,
    pub relative_offsets: HashMap<PlacementKey, RawMarker>,
    pub reference_views: HashMap<PlacementKey, ReferenceView>,
    pub reference_points: HashMap<PlacementKey, ReferencePoint>,
}
impl<PlacementKey: Eq + PartialEq + Hash> Placer<PlacementKey> {
    pub fn new<GP: Into<GridPoint>>(anchor: GP) -> Self {
        Self {
            anchor: anchor.into(),
            relative_offsets: HashMap::new(),
            reference_views: HashMap::new(),
            reference_points: HashMap::new(),
        }
    }
    pub fn add<R: Into<RawMarker>>(&mut self, key: PlacementKey, marker: R) {
        self.relative_offsets.insert(key, marker.into());
    }
    pub fn add_reference_view<R: Into<ReferenceView>>(&mut self, key: PlacementKey, view: R) {
        self.reference_views.insert(key, view.into());
    }
    pub fn reference_view(&self, key: PlacementKey) -> ReferenceView {
        self.reference_views
            .get(&key)
            .copied()
            .expect("reference-view")
    }
    pub fn view_from_reference(&self, key: PlacementKey) -> GridView {
        let b = self.reference_view(key);
        self.view(b.left(), b.right(), b.top(), b.bottom())
    }
    pub fn get(&self, key: PlacementKey) -> RawMarker {
        self.relative_offsets
            .get(&key)
            .copied()
            .expect("RelativePlacer::get")
    }
    pub fn get_reference_point(&self, key: PlacementKey) -> ReferencePoint {
        self.reference_points
            .get(&key)
            .copied()
            .expect("reference-point")
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
    pub fn add_point<R: Into<RawMarker>>(&mut self, key: PlacementKey, x: R, y: R) {
        self.reference_points
            .insert(key, ReferencePoint::new().with_x(x).with_y(y));
    }
    pub fn point_from_reference(&self, key: PlacementKey) -> GridPoint {
        let b = self
            .reference_points
            .get(&key)
            .copied()
            .expect("reference-point");
        self.point(b.x(), b.y())
    }
}
#[derive(Default, Clone)]
pub struct Placement<PlacementKey> {
    pub(crate) views: HashMap<PlacementKey, GridView>,
    pub(crate) points: HashMap<PlacementKey, GridPoint>,
}
impl<PlacementKey: Eq + PartialEq + Hash> Placement<PlacementKey> {
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
