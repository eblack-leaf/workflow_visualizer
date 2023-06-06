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
    pub(crate) fn new(area: Area<InterfaceContext>) -> Self {
        let (span, extension) = {
            if area.width > HorizontalSpan::MEDIUM_BREAKPOINT {
                (HorizontalSpan::Twelve, MarkerGrouping(0))
            } else if area.width > HorizontalSpan::SMALL_BREAKPOINT {
                (HorizontalSpan::Eight, MarkerGrouping(0))
            } else {
                (HorizontalSpan::Four, MarkerGrouping(0))
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
    pub fn calc_section(&self, view: &ResponsiveContentView) -> Section<InterfaceContext> {
        let current_view = view.mapping.get(&self.span).expect("view mapping");
        let markers_per_column = self.column_config.base.0 + self.column_config.extension.0;
        let left = current_view.horizontal.begin.marker.0 * markers_per_column
            + self.gutter_config.base.0 * current_view.horizontal.begin.marker.0;
        let left = if current_view.horizontal.begin.offset == ContentOffset::Near {
            left - markers_per_column
        } else {
            left
        };
        let top = current_view.vertical.begin.marker.0 * self.row_config.base.0
            + self.gutter_config.base.0 * current_view.vertical.begin.marker.0;
        let top = if current_view.vertical.begin.offset == ContentOffset::Near {
            top - self.row_config.base.0
        } else {
            top
        };
        let right = current_view.horizontal.end.marker.0 * markers_per_column
            + self.gutter_config.base.0 * current_view.horizontal.end.marker.0;
        let right = if current_view.horizontal.end.offset == ContentOffset::Near {
            right - markers_per_column
        } else {
            right
        };
        let bottom = current_view.vertical.end.marker.0 * self.row_config.base.0
            + self.gutter_config.base.0 * current_view.vertical.end.marker.0;
        let bottom = if current_view.vertical.end.offset == ContentOffset::Near {
            bottom - self.row_config.base.0
        } else {
            bottom
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
pub struct Marker(pub i32);
impl Marker {
    pub const PX: f32 = 8f32;
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
/// whether to attach to beginning/end of column
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ContentOffset {
    Near,
    Far,
}
/// Pair of ContentMarker and Offset to get an exact grid location
#[derive(Copy, Clone)]
pub struct ContentLocation {
    pub marker: ContentMarker,
    pub offset: ContentOffset,
}
impl<T: Into<ContentMarker>> From<(T, ContentOffset)> for ContentLocation {
    fn from(value: (T, ContentOffset)) -> Self {
        ContentLocation {
            marker: value.0.into(),
            offset: value.1,
        }
    }
}
/// Beginning and End GridLocation grouping
#[derive(Copy, Clone)]
pub struct ContentRange {
    pub begin: ContentLocation,
    pub end: ContentLocation,
}
impl<T: Into<ContentLocation>> From<(T, T)> for ContentRange {
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
        for (view, mut pos, mut area) in queries.p0().iter_mut() {
            update_section(grid.as_ref(), view, pos.as_mut(), area.as_mut());
        }
    } else {
        for (view, mut pos, mut area) in queries.p1().iter_mut() {
            update_section(grid.as_ref(), view, pos.as_mut(), area.as_mut());
        }
    }
}
fn update_section(grid: &Grid, view: &ResponsiveContentView, pos: &mut Position<InterfaceContext>, area: &mut Area<InterfaceContext>) {
    let section = grid.calc_section(view);
    *pos = section.position;
    *area = section.area;
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
