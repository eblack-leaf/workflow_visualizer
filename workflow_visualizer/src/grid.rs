use crate::viewport::ViewportHandle;
use crate::{Area, InterfaceContext, Position, Section};
use bevy_ecs::component::Component;
use bevy_ecs::prelude::{Commands, DetectChanges, Query, Res, Resource};
use bevy_ecs::query::Changed;
use bevy_ecs::system::ResMut;
use std::collections::HashMap;

#[derive(Resource, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum HorizontalSpan {
    Four,
    Eight,
    Twelve,
}
impl HorizontalSpan {
    pub fn gutter_base(&self) -> MarkerGrouping {
        match self {
            HorizontalSpan::Four => MarkerGrouping(2),
            HorizontalSpan::Eight => MarkerGrouping(2),
            HorizontalSpan::Twelve => MarkerGrouping(3),
        }
    }
    pub fn content_base(&self) -> MarkerGrouping {
        match self {
            HorizontalSpan::Four => MarkerGrouping(10),
            HorizontalSpan::Eight => MarkerGrouping(9),
            HorizontalSpan::Twelve => MarkerGrouping(10),
        }
    }
    pub const SMALL_BREAKPOINT: f32 = 720f32;
    pub const MEDIUM_BREAKPOINT: f32 = 1168f32;
}
#[derive(Resource)]
pub struct Grid {
    pub(crate) span: HorizontalSpan,
    pub(crate) column_config: ColumnConfig,
    pub(crate) row_config: RowConfig,
    pub(crate) gutter_config: GutterConfig,
}
pub(crate) fn setup(viewport_handle: Res<ViewportHandle>, mut cmd: Commands) {
    let area = viewport_handle.section.area;

}
/// Index of 8px alignment location
pub struct Marker(pub i32);
impl Marker {
    pub const PX: f32 = 8f32;
}
/// Number of markers to include in a logical group
pub struct MarkerGrouping(pub u8);
pub struct ColumnConfig {
    pub base: MarkerGrouping,
    pub extension: MarkerGrouping,
}
/// MarkerGrouping for deciding gutter size
pub struct GutterConfig {
    pub base: MarkerGrouping,
}
/// MarkerGrouping fro deciding row size
pub struct RowConfig {
    pub base: MarkerGrouping,
}
/// Logical index using groupings to get actual markers then to px
pub struct ContentMarker(pub i32);
/// whether to attach to beginning/end of column
pub enum ContentOffset {
    Near,
    Far,
}
/// Pair of ContentMarker and Offset to get an exact grid location
pub struct ContentLocation {
    pub marker: ContentMarker,
    pub offset: ContentOffset,
}
/// Beginning and End GridLocation grouping
pub struct ContentRange {
    pub begin: ContentLocation,
    pub end: ContentLocation,
}
/// A GridRange for horizontal + vertical aspects
pub struct ContentView {
    pub horizontal: ContentRange,
    pub vertical: ContentRange,
}
/// A mapping of GridView for each HorizontalSpan Option
#[derive(Component)]
pub struct ResponsiveView<T> {
    pub mapping: HashMap<HorizontalSpan, T>,
}
/// View of a Point on a Grid
pub struct PointView {
    pub horizontal: Marker,
    pub vertical: Marker,
}
pub type ResponsiveContentView = ResponsiveView<ContentView>;
pub type ResponsivePointView = ResponsiveView<PointView>;
pub(crate) fn grid_response(
    viewport_handle: Res<ViewportHandle>,
    responsively_viewed: Query<(
        &ResponsiveContentView,
        &mut Position<InterfaceContext>,
        &mut Area<InterfaceContext>,
    )>,
    grid: ResMut<Grid>,
) {
    if viewport_handle.is_changed() {
        // configure grid configs + span
        // set from view for all
    }
}
pub(crate) fn set_from_view(
    grid: Res<Grid>,
    mut changed: Query<
        (
            &ResponsiveContentView,
            &mut Position<InterfaceContext>,
            &mut Area<InterfaceContext>,
        ),
        Changed<ResponsiveContentView>,
    >,
) {
    for (responsive_view, mut pos, mut area) in changed.iter_mut() {
        // match from grid and set to pos / area
    }
}
