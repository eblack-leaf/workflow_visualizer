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
    pub fn gutter_base(&self) -> RawMarkerGrouping {
        match self {
            HorizontalSpan::Four => RawMarkerGrouping(2),
            HorizontalSpan::Eight => RawMarkerGrouping(2),
            HorizontalSpan::Twelve => RawMarkerGrouping(3),
        }
    }
    pub fn content_base(&self) -> RawMarkerGrouping {
        match self {
            HorizontalSpan::Four => RawMarkerGrouping(10),
            HorizontalSpan::Eight => RawMarkerGrouping(9),
            HorizontalSpan::Twelve => RawMarkerGrouping(10),
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
                (HorizontalSpan::Twelve, RawMarkerGrouping(extension))
            } else if area.width > HorizontalSpan::SMALL_BREAKPOINT {
                let extension = Self::calc_extension(
                    area.width,
                    Self::SPAN_EIGHT_EXT_BASE,
                    Self::SPAN_EIGHT_COLUMNS,
                );
                (HorizontalSpan::Eight, RawMarkerGrouping(extension))
            } else {
                let extension = Self::calc_extension(
                    area.width,
                    Self::SPAN_FOUR_EXT_BASE,
                    Self::SPAN_FOUR_COLUMNS,
                );
                (HorizontalSpan::Four, RawMarkerGrouping(extension))
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
        ((width - base) / RawMarker::PX).floor() as i32 / columns
    }
    pub fn calc_section(&self, view: &ResponsiveGridView) -> Section<InterfaceContext> {
        let current_view = view.mapping.get(&self.span).expect("view mapping");
        let left = {
            let grid_location = current_view.horizontal.begin;
            self.calc_horizontal_location(grid_location)
        };
        let top = {
            let grid_location = current_view.vertical.begin;
            self.calc_vertical_location(grid_location)
        };
        let right = {
            let grid_location = current_view.horizontal.end;
            self.calc_horizontal_location(grid_location)
        };
        let bottom = {
            let grid_location = current_view.vertical.end;
            self.calc_vertical_location(grid_location)
        };
        let left = left as f32 * RawMarker::PX;
        let top = top as f32 * RawMarker::PX;
        let right = right as f32 * RawMarker::PX;
        let bottom = bottom as f32 * RawMarker::PX;
        Section::from_left_top_right_bottom(left, top, right, bottom)
    }

    pub fn markers_per_column(&self) -> i32 {
        self.column_config.base.0 + self.column_config.extension.0
    }

    pub fn calc_horizontal_location(&self, grid_location: GridLocation) -> i32 {
        let markers_per_column = self.markers_per_column();
        let content_location = grid_location.location;
        let location = content_location.marker.0 * markers_per_column
            + self.gutter_config.base.0 * content_location.marker.0;
        let location = if content_location.bias == GridMarkerBias::Near {
            location - markers_per_column
        } else {
            location
        };
        if let Some(offset) = grid_location.offset {
            location + offset.0 .0
        } else {
            location
        }
    }

    pub fn calc_vertical_location(&self, grid_location: GridLocation) -> i32 {
        let content_location = grid_location.location;
        let location = content_location.marker.0 * self.row_config.base.0
            + self.gutter_config.base.0 * content_location.marker.0;
        let location = if content_location.bias == GridMarkerBias::Near {
            location - self.row_config.base.0
        } else {
            location
        };
        if let Some(offset) = grid_location.offset {
            location + offset.0 .0
        } else {
            location
        }
    }
}
pub(crate) fn setup(viewport_handle: Res<ViewportHandle>, mut cmd: Commands) {
    let area = viewport_handle.section.area;
    let grid = Grid::new(area);
    cmd.insert_resource(grid);
}
/// Index of 8px alignment location
#[derive(Copy, Clone, PartialEq)]
pub struct RawMarker(pub i32);
impl RawMarker {
    pub const PX: f32 = 8f32;
}
impl From<i32> for RawMarker {
    fn from(value: i32) -> Self {
        RawMarker(value)
    }
}
/// Number of markers to include in a logical group
pub struct RawMarkerGrouping(pub i32);
pub(crate) struct ColumnConfig {
    pub base: RawMarkerGrouping,
    pub extension: RawMarkerGrouping,
}
/// MarkerGrouping for deciding gutter size
pub(crate) struct GutterConfig {
    pub base: RawMarkerGrouping,
}
/// MarkerGrouping fro deciding row size
pub(crate) struct RowConfig {
    pub base: RawMarkerGrouping,
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
/// Shorthand for specifying a GridLocation using ContentAligned
pub trait ResponsiveUnit {
    fn near(self) -> GridLocation;
    fn far(self) -> GridLocation;
}
impl ResponsiveUnit for i32 {
    fn near(self) -> GridLocation {
        (self, GridMarkerBias::Near).into()
    }
    fn far(self) -> GridLocation {
        (self, GridMarkerBias::Far).into()
    }
}
/// Description of a Location on the Grid
#[derive(Copy, Clone)]
pub struct GridLocation {
    pub location: GridLocationDescriptor,
    pub offset: Option<GridLocationOffset>,
}
impl GridLocation {
    pub fn offset(mut self, offset: i32) -> Self {
        self.offset.replace(GridLocationOffset(offset.into()));
        self
    }
}
/// Pair of ContentMarker and Offset to get an exact grid location
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
/// A mapping of GridView for each HorizontalSpan Option
#[derive(Component)]
pub struct ResponsiveView<T> {
    pub mapping: HashMap<HorizontalSpan, T>,
}
/// Convenience type for mapping to ContentViews
pub type ResponsiveGridView = ResponsiveView<GridView>;
impl ResponsiveGridView {
    pub fn with_span_four<T: Into<GridView>>(mut self, view: T) -> Self {
        self.mapping.insert(HorizontalSpan::Four, view.into());
        self
    }
    pub fn with_span_eight<T: Into<GridView>>(mut self, view: T) -> Self {
        self.mapping.insert(HorizontalSpan::Eight, view.into());
        self
    }
    pub fn with_span_twelve<T: Into<GridView>>(mut self, view: T) -> Self {
        self.mapping.insert(HorizontalSpan::Twelve, view.into());
        self
    }
}
impl<T: Into<GridView>> From<T> for ResponsiveGridView {
    fn from(value: T) -> Self {
        let value = value.into();
        let mut mapping = HashMap::new();
        mapping.insert(HorizontalSpan::Four, value);
        mapping.insert(HorizontalSpan::Eight, value);
        mapping.insert(HorizontalSpan::Twelve, value);
        ResponsiveGridView { mapping }
    }
}
impl<T: Into<GridView>> From<(T, T, T)> for ResponsiveGridView {
    fn from(value: (T, T, T)) -> Self {
        let mut mapping = HashMap::new();
        mapping.insert(HorizontalSpan::Four, value.0.into());
        mapping.insert(HorizontalSpan::Eight, value.1.into());
        mapping.insert(HorizontalSpan::Twelve, value.2.into());
        ResponsiveGridView { mapping }
    }
}
fn update_section(
    grid: &Grid,
    view: &ResponsiveGridView,
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
            &ResponsiveGridView,
            &mut Position<InterfaceContext>,
            &mut Area<InterfaceContext>,
        )>,
        Query<
            (
                &ResponsiveGridView,
                &mut Position<InterfaceContext>,
                &mut Area<InterfaceContext>,
            ),
            Changed<ResponsiveGridView>,
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
            &ResponsiveGridView,
            &mut Position<InterfaceContext>,
            &mut Area<InterfaceContext>,
        ),
        Changed<ResponsiveGridView>,
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
