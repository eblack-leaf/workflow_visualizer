use crate::viewport::{frontend_area_adjust, ViewportHandle};
use crate::{
    Area, Attach, InterfaceContext, Position, Section, SyncPoint, UserSpaceSyncPoint, Visualizer,
};
use bevy_ecs::component::Component;
use bevy_ecs::prelude::{Commands, DetectChanges, IntoSystemConfig, Query, Res, Resource};
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
pub struct MarkerGrouping(pub i8);
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
pub struct ContentMarker(pub i32);
/// whether to attach to beginning/end of column
#[derive(PartialEq, Eq)]
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
pub type ResponsiveContentView = ResponsiveView<ContentView>;
pub(crate) fn grid_response(
    viewport_handle: Res<ViewportHandle>,
    mut responsively_viewed: Query<(
        &ResponsiveContentView,
        &mut Position<InterfaceContext>,
        &mut Area<InterfaceContext>,
    )>,
    grid: ResMut<Grid>,
) {
    if viewport_handle.is_changed() {
        // configure grid configs + span
        *grid = Grid::new(viewport_handle.section.area);
        // set from view for all
        for (view, mut pos, mut area) in responsively_viewed.iter_mut() {
            let section = grid.calc_section(view);
            *pos = section.pos;
            *area = section.area;
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
        let section = grid.calc_section(view);
        *pos = section.pos;
        *area = section.area;
    }
}
pub(crate) struct GridAttachment;
impl Attach for GridAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer
            .job
            .task(Visualizer::TASK_STARTUP)
            .add_systems((setup.in_set(SyncPoint::Initialization)));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            grid_response.in_set(SyncPoint::Config),
            set_from_view.in_set(SyncPoint::Config),// if this troubles ordering, combine with SystemParams into grid_response
            set_from_view.in_set(SyncPoint::Reconfigure),
        ));
    }
}
