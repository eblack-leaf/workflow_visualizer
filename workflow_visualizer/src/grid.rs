use crate::viewport::ViewportHandle;
use crate::{Area, InterfaceContext, Position, Section};
use bevy_ecs::component::Component;
use bevy_ecs::prelude::{DetectChanges, Query, Res, Resource};
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
    pub fn spacer_base(&self) -> MarkerGrouping {
        match self {
            HorizontalSpan::Four => MarkerGrouping(10),
            HorizontalSpan::Eight => MarkerGrouping(9),
            HorizontalSpan::Twelve => MarkerGrouping(10),
        }
    }
}
#[derive(Resource)]
pub struct Grid {
    pub(crate) span: HorizontalSpan,
    pub(crate) column_config: ColumnConfig,
    pub(crate) row_config: RowConfig,
    pub(crate) gutter_config: GutterConfig,
}
/// Index of 8px alignment location
pub struct Marker(pub i32);
/// Number of markers to include in a logical group
pub struct MarkerGrouping(pub i8);
pub struct ColumnConfig {
    pub base: MarkerGrouping,
    pub extension: MarkerGrouping,
}
pub struct GutterConfig {
    pub base: MarkerGrouping,
}
pub struct RowConfig {
    pub base: MarkerGrouping,
}
pub struct GridSpacer(pub i32);
pub enum GridSpacerOffset {
    Near,
    Far,
}
pub struct GridLocation {
    pub identifier: GridSpacer,
    pub offset: GridSpacerOffset,
}
pub struct GridRange {
    pub begin: GridLocation,
    pub end: GridLocation,
}
pub struct GridView {
    pub horizontal_range: GridRange,
    pub vertical_range: GridRange,
}
#[derive(Component)]
pub struct ResponsiveGridView {
    pub mapping: HashMap<HorizontalSpan, GridView>,
}
pub(crate) fn grid_response(
    viewport_handle: Res<ViewportHandle>,
    responsively_viewed: Query<(
        &ResponsiveGridView,
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
            &ResponsiveGridView,
            &mut Position<InterfaceContext>,
            &mut Area<InterfaceContext>,
        ),
        Changed<ResponsiveGridView>,
    >,
) {
    for (responsive_view, mut pos, mut area) in changed.iter_mut() {
        // match from grid and set to pos / area
    }
}
