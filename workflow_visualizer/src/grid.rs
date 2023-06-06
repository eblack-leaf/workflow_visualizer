use crate::viewport::{frontend_area_adjust, ViewportHandle};
use crate::{
    Area, Attach, InterfaceContext, Position, Section, SyncPoint, UserSpaceSyncPoint, Visualizer,
};
use bevy_ecs::component::Component;
use bevy_ecs::prelude::{
    Commands, DetectChanges, IntoSystemConfig, ParamSet, Query, Res, Resource,
};
use bevy_ecs::query::Changed;
use bevy_ecs::system::ResMut;
use std::collections::HashMap;

#[derive(Resource, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
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
impl Grid {
    pub(crate) const SPAN_FOUR_EXT_BASE: f32 = 400f32;
    pub(crate) const SPAN_EIGHT_EXT_BASE: f32 = 720f32;
    pub(crate) const SPAN_TWELVE_EXT_BASE: f32 = 1168f32;
    pub(crate) const SPAN_TWELVE_COLUMNS: i32 = 12;
    pub(crate) const SPAN_EIGHT_COLUMNS: i32 = 8;
    pub(crate) const SPAN_FOUR_COLUMNS: i32 = 4;
    pub(crate) fn new(area: Area<InterfaceContext>) -> Self {
        let (span, extension) = {
            if area.width > HorizontalSpan::MEDIUM_BREAKPOINT {
                let extension = Self::calc_extension(
                    area.width,
                    Self::SPAN_TWELVE_EXT_BASE,
                    Self::SPAN_TWELVE_COLUMNS,
                );
                (HorizontalSpan::Twelve, MarkerGrouping(extension))
            } else if area.width > HorizontalSpan::SMALL_BREAKPOINT {
                let extension = Self::calc_extension(
                    area.width,
                    Self::SPAN_EIGHT_EXT_BASE,
                    Self::SPAN_EIGHT_COLUMNS,
                );
                (HorizontalSpan::Eight, MarkerGrouping(extension))
            } else {
                let extension = Self::calc_extension(
                    area.width,
                    Self::SPAN_FOUR_EXT_BASE,
                    Self::SPAN_FOUR_COLUMNS,
                );
                (HorizontalSpan::Four, MarkerGrouping(extension))
            }
        };
        Self {
            span,
            column_config: ColumnConfig {
                base: span.content_base(),
                extension,
            },
            row_config: RowConfig {
                base: span.content_base(),
            },
            gutter_config: GutterConfig {
                base: span.gutter_base(),
            },
        }
    }

    fn calc_extension(width: f32, base: f32, columns: i32) -> i32 {
        ((width - base) / Marker::PX).floor() as i32 / columns
    }
    pub fn calc_section(&self, view: &ResponsiveContentView) -> Section<InterfaceContext> {
        let current_view = view.mapping.get(&self.span).expect("view mapping");
        let markers_per_column = self.column_config.base.0 + self.column_config.extension.0;
        let left = match &current_view.horizontal.begin {
            GridLocation::Raw(marker) => marker.0,
            GridLocation::ContentAligned(content_location) => {
                let left = content_location.marker.0 * markers_per_column
                    + self.gutter_config.base.0 * content_location.marker.0;
                let left = if content_location.offset == ContentOffset::Near {
                    left - markers_per_column
                } else {
                    left
                };
                left
            }
        };
        let top = match &current_view.vertical.begin {
            GridLocation::Raw(marker) => marker.0,
            GridLocation::ContentAligned(content_location) => {
                let top = content_location.marker.0 * self.row_config.base.0
                    + self.gutter_config.base.0 * content_location.marker.0;
                let top = if content_location.offset == ContentOffset::Near {
                    top - self.row_config.base.0
                } else {
                    top
                };
                top
            }
        };
        let right = match &current_view.horizontal.end {
            GridLocation::Raw(marker) => marker.0,
            GridLocation::ContentAligned(content_location) => {
                let right = content_location.marker.0 * markers_per_column
                    + self.gutter_config.base.0 * content_location.marker.0;
                let right = if content_location.offset == ContentOffset::Near {
                    right - markers_per_column
                } else {
                    right
                };
                right
            }
        };
        let bottom = match &current_view.vertical.end {
            GridLocation::Raw(marker) => marker.0,
            GridLocation::ContentAligned(content_location) => {
                let bottom = content_location.marker.0 * self.row_config.base.0
                    + self.gutter_config.base.0 * content_location.marker.0;
                let bottom = if content_location.offset == ContentOffset::Near {
                    bottom - self.row_config.base.0
                } else {
                    bottom
                };
                bottom
            }
        };
        let left = left as f32 * Marker::PX;
        let top = top as f32 * Marker::PX;
        let right = right as f32 * Marker::PX;
        let bottom = bottom as f32 * Marker::PX;
        Section::from_left_top_right_bottom(left, top, right, bottom)
    }
}
pub(crate) fn setup(viewport_handle: Res<ViewportHandle>, mut cmd: Commands) {
    let area = viewport_handle.section.area;
    let grid = Grid::new(area);
    cmd.insert_resource(grid);
}
/// Index of 8px alignment location
#[derive(Copy, Clone)]
pub struct Marker(pub i32);
impl Marker {
    pub const PX: f32 = 8f32;
}
impl From<i32> for Marker {
    fn from(value: i32) -> Self {
        Marker(value)
    }
}
/// Number of markers to include in a logical group
pub struct MarkerGrouping(pub i32);
pub(crate) struct ColumnConfig {
    pub base: MarkerGrouping,
    pub extension: MarkerGrouping,
}
/// MarkerGrouping for deciding gutter size
pub(crate) struct GutterConfig {
    pub base: MarkerGrouping,
}
/// MarkerGrouping fro deciding row size
pub(crate) struct RowConfig {
    pub base: MarkerGrouping,
}
/// Logical index using groupings to get actual markers then to px
#[derive(Copy, Clone)]
pub struct ContentMarker(pub i32);
impl From<i32> for ContentMarker {
    fn from(value: i32) -> Self {
        ContentMarker(value)
    }
}
/// Whether to attach to beginning/end of column
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ContentOffset {
    Near,
    Far,
}
/// Shorthand for specifying a GridLocation using ContentAligned
pub trait ResponsiveUnit {
    fn near(self) -> GridLocation;
    fn far(self) -> GridLocation;
    fn raw(self) -> GridLocation;
}
impl ResponsiveUnit for i32 {
    fn near(self) -> GridLocation {
        (self, ContentOffset::Near).into()
    }

    fn far(self) -> GridLocation {
        (self, ContentOffset::Far).into()
    }
    fn raw(self) -> GridLocation {
        self.into()
    }
}
/// Description of a Location on the Grid
#[derive(Copy, Clone)]
pub enum GridLocation {
    /// Raw markers of 8px to align to specific spots
    Raw(Marker),
    /// Content Aligned description with ContentMarker
    /// and Offset
    ContentAligned(ContentLocation),
}
impl<T: Into<Marker>> From<T> for GridLocation {
    fn from(value: T) -> Self {
        GridLocation::Raw(value.into())
    }
}
/// Pair of ContentMarker and Offset to get an exact grid location
#[derive(Copy, Clone)]
pub struct ContentLocation {
    pub marker: ContentMarker,
    pub offset: ContentOffset,
}
impl<T: Into<ContentMarker>> From<(T, ContentOffset)> for GridLocation {
    fn from(value: (T, ContentOffset)) -> Self {
        GridLocation::ContentAligned(ContentLocation {
            marker: value.0.into(),
            offset: value.1,
        })
    }
}
/// Beginning and End GridLocation grouping
#[derive(Copy, Clone)]
pub struct ContentRange {
    pub begin: GridLocation,
    pub end: GridLocation,
}
impl<T: Into<GridLocation>> From<(T, T)> for ContentRange {
    fn from(value: (T, T)) -> Self {
        ContentRange {
            begin: value.0.into(),
            end: value.1.into(),
        }
    }
}
/// A GridRange for horizontal + vertical aspects
#[derive(Copy, Clone)]
pub struct ContentView {
    pub horizontal: ContentRange,
    pub vertical: ContentRange,
}
impl<T: Into<ContentRange>> From<(T, T)> for ContentView {
    fn from(value: (T, T)) -> Self {
        ContentView {
            horizontal: value.0.into(),
            vertical: value.1.into(),
        }
    }
}
/// A mapping of GridView for each HorizontalSpan Option
#[derive(Component)]
pub struct ResponsiveView<T> {
    pub mapping: HashMap<HorizontalSpan, T>,
}
/// Convenience type for mapping to ContentViews
pub type ResponsiveContentView = ResponsiveView<ContentView>;
impl ResponsiveContentView {
    pub fn with_span_four<T: Into<ContentView>>(mut self, view: T) -> Self {
        self.mapping.insert(HorizontalSpan::Four, view.into());
        self
    }
    pub fn with_span_eight<T: Into<ContentView>>(mut self, view: T) -> Self {
        self.mapping.insert(HorizontalSpan::Eight, view.into());
        self
    }
    pub fn with_span_twelve<T: Into<ContentView>>(mut self, view: T) -> Self {
        self.mapping.insert(HorizontalSpan::Twelve, view.into());
        self
    }
}
impl<T: Into<ContentView>> From<T> for ResponsiveContentView {
    fn from(value: T) -> Self {
        let value = value.into();
        let mut mapping = HashMap::new();
        mapping.insert(HorizontalSpan::Four, value);
        mapping.insert(HorizontalSpan::Eight, value);
        mapping.insert(HorizontalSpan::Twelve, value);
        ResponsiveContentView { mapping }
    }
}
impl<T: Into<ContentView>> From<(T, T, T)> for ResponsiveContentView {
    fn from(value: (T, T, T)) -> Self {
        let mut mapping = HashMap::new();
        mapping.insert(HorizontalSpan::Four, value.0.into());
        mapping.insert(HorizontalSpan::Eight, value.1.into());
        mapping.insert(HorizontalSpan::Twelve, value.2.into());
        ResponsiveContentView { mapping }
    }
}
fn update_section(
    grid: &Grid,
    view: &ResponsiveContentView,
    pos: &mut Position<InterfaceContext>,
    area: &mut Area<InterfaceContext>,
) {
    let section = grid.calc_section(view);
    *pos = section.position;
    *area = section.area;
}
pub(crate) fn config_grid(
    viewport_handle: Res<ViewportHandle>,
    mut queries: ParamSet<(
        Query<(
            &ResponsiveContentView,
            &mut Position<InterfaceContext>,
            &mut Area<InterfaceContext>,
        )>,
        Query<
            (
                &ResponsiveContentView,
                &mut Position<InterfaceContext>,
                &mut Area<InterfaceContext>,
            ),
            Changed<ResponsiveContentView>,
        >,
    )>,
    mut grid: ResMut<Grid>,
) {
    if viewport_handle.is_changed() {
        // configure grid configs + span
        *grid = Grid::new(viewport_handle.section.area);
        // update all views
        for (view, mut pos, mut area) in queries.p0().iter_mut() {
            update_section(grid.as_ref(), view, pos.as_mut(), area.as_mut());
        }
    } else {
        // only update changed views
        for (view, mut pos, mut area) in queries.p1().iter_mut() {
            update_section(grid.as_ref(), view, pos.as_mut(), area.as_mut());
        }
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
    for (view, mut pos, mut area) in changed.iter_mut() {
        update_section(grid.as_ref(), view, pos.as_mut(), area.as_mut());
    }
}
pub(crate) struct GridAttachment;
impl Attach for GridAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer
            .job
            .task(Visualizer::TASK_STARTUP)
            .add_systems((setup.in_set(SyncPoint::Initialization),));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            config_grid.in_set(SyncPoint::Config),
            set_from_view.in_set(SyncPoint::Reconfigure),
        ));
    }
}
