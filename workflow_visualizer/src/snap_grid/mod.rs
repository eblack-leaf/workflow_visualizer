use crate::{
    Area, Attach, CoordinateUnit, DelayedBundle, InterfaceContext, Position, QueuedAnimation,
    Section, SyncPoint, TimeDelta, ViewportHandle, Visualizer,
};
use bevy_ecs::prelude::{IntoSystemConfigs, Res, Resource};
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::{Commands, Query};
pub use breakpoint::Breakpoint;
pub(crate) use float::reapply;
pub use float::{
    FloatArrangement, FloatLocation, FloatPlacement, FloatPlacementDescriptor, FloatPlacer,
    FloatPoint, FloatRange, FloatView,
};
pub use marker::GridPoint;
pub use marker::{GridBias, GridLocation, GridMarker, GridUnit};
pub use marker::{GridDirection, GridRange, GridView};
pub use responsive::{
    GridPlacer, ResponsiveGridLocation, ResponsiveGridPlacementDescriptor, ResponsiveGridPoint,
    ResponsiveGridRange, ResponsiveGridView,
};

mod breakpoint;
mod float;
mod marker;
mod responsive;

#[derive(Copy, Clone)]
pub struct Column {
    pub content: CoordinateUnit,
    pub gutter: CoordinateUnit,
    pub extension: Option<CoordinateUnit>,
}
impl Column {
    pub fn new(area: Area<InterfaceContext>, breakpoint: Breakpoint) -> Self {
        let actual = area.width.min(area.height * 1.778);
        Self {
            content: (actual - breakpoint.gutter() * (breakpoint.segments() + 1) as f32)
                / breakpoint.segments() as f32,
            gutter: breakpoint.gutter(),
            extension: if area.width > actual {
                Some(area.width - actual)
            } else {
                None
            },
        }
    }
}
#[test]
#[cfg(test)]
fn snap_grid_coverage() {
    for width in (375..1400).step_by(8) {
        for height in (375..1400).step_by(8) {
            let area = Area::new(width as f32, height as f32);
            let grid = SnapGrid::new(area);
            println!(
                "Snap-Grid@{}-{}: column: {:?} row: {:?}",
                width, height, grid.column.content, grid.row.content
            );
        }
    }
}
#[derive(Copy, Clone)]
pub struct Row {
    pub content: CoordinateUnit,
    pub gutter: CoordinateUnit,
    pub extension: Option<CoordinateUnit>,
}

impl Row {
    pub fn new(height: CoordinateUnit, breakpoint: Breakpoint) -> Self {
        let actual = height.min(breakpoint.value());
        Self {
            content: (actual - breakpoint.gutter() * (breakpoint.segments() + 1) as f32)
                / breakpoint.segments() as f32,
            gutter: breakpoint.gutter(),
            extension: if height > actual {
                Some(height - actual)
            } else {
                None
            },
        }
    }
}
/// Macro placement tool segmented into fixed number of columns/rows.
#[derive(Resource, Copy, Clone)]
pub struct SnapGrid {
    pub column: Column,
    pub row: Row,
    pub breakpoint: Breakpoint,
}
impl SnapGrid {
    pub const GUTTER_BASE: CoordinateUnit = 8f32;
    pub fn new(area: Area<InterfaceContext>) -> Self {
        let breakpoint = Breakpoint::establish(area.width);
        Self {
            column: Column::new(area, breakpoint),
            row: Row::new(area.height, breakpoint),
            breakpoint,
        }
    }
    pub fn location_coordinates(
        &self,
        direction: GridDirection,
        location: GridLocation,
    ) -> CoordinateUnit {
        match direction {
            GridDirection::Horizontal => {
                self.column.content * location.marker.0 as f32
                    + self.column.gutter * location.marker.0 as f32
                    - if location.bias == GridBias::Near {
                        self.column.content
                    } else {
                        0f32
                    }
            }
            GridDirection::Vertical => {
                self.row.content * location.marker.0 as f32
                    + self.row.gutter * location.marker.0 as f32
                    - if location.bias == GridBias::Near {
                        self.row.content
                    } else {
                        0f32
                    }
            }
        }
    }
    pub fn view_coordinates(&self, view: GridView) -> Section<InterfaceContext> {
        Section::from_left_top_right_bottom(
            self.location_coordinates(GridDirection::Horizontal, view.horizontal.begin),
            self.location_coordinates(GridDirection::Vertical, view.vertical.begin),
            self.location_coordinates(GridDirection::Horizontal, view.horizontal.end),
            self.location_coordinates(GridDirection::Vertical, view.vertical.end),
        )
    }
    pub fn range_coordinates(
        &self,
        direction: GridDirection,
        range: GridRange,
    ) -> (CoordinateUnit, CoordinateUnit) {
        (
            self.location_coordinates(direction, range.begin),
            self.location_coordinates(direction, range.end),
        )
    }
    pub fn point_coordinates(&self, point: GridPoint) -> Position<InterfaceContext> {
        Position::new(
            self.location_coordinates(GridDirection::Horizontal, point.x),
            self.location_coordinates(GridDirection::Vertical, point.y),
        )
    }
    pub fn responsive_view_coordinates(
        &self,
        view: ResponsiveGridView,
    ) -> Section<InterfaceContext> {
        self.view_coordinates(view.current(self.breakpoint, self.breakpoint))
    }
    pub fn responsive_point_coordinates(
        &self,
        point: ResponsiveGridPoint,
    ) -> Position<InterfaceContext> {
        self.point_coordinates(point.current(self.breakpoint, self.breakpoint))
    }
    pub fn responsive_range_coordinates(
        &self,
        direction: GridDirection,
        range: ResponsiveGridRange,
    ) -> (CoordinateUnit, CoordinateUnit) {
        match direction {
            GridDirection::Horizontal => {
                self.range_coordinates(direction, range.current(self.breakpoint))
            }
            GridDirection::Vertical => {
                self.range_coordinates(direction, range.current(self.breakpoint))
            }
        }
    }
    pub fn responsive_location_coordinates(
        &self,
        direction: GridDirection,
        location: ResponsiveGridLocation,
    ) -> CoordinateUnit {
        let location = match direction {
            GridDirection::Horizontal => location.current(self.breakpoint),
            GridDirection::Vertical => location.current(self.breakpoint),
        };
        self.location_coordinates(direction, location)
    }
    pub fn animate_location(
        &self,
        _begin: ResponsiveGridPoint,
        _other: ResponsiveGridPoint,
        _interval: TimeDelta,
        _delay: Option<TimeDelta>,
    ) -> (
        QueuedAnimation<Position<InterfaceContext>>,
        DelayedBundle<ResponsiveGridPoint>,
    ) {
        todo!()
    }
    pub fn animate_view(
        &self,
        _begin: ResponsiveGridView,
        _end: ResponsiveGridView,
        _interval: TimeDelta,
        _delay: Option<TimeDelta>,
    ) -> (
        QueuedAnimation<Position<InterfaceContext>>,
        QueuedAnimation<Area<InterfaceContext>>,
        DelayedBundle<ResponsiveGridView>,
    ) {
        todo!()
    }
}
pub(crate) fn calculate(
    mut gridded: Query<
        (
            &mut Position<InterfaceContext>,
            Option<&mut Area<InterfaceContext>>,
            Option<&ResponsiveGridPoint>,
            Option<&ResponsiveGridView>,
        ),
        Or<(Changed<ResponsiveGridPoint>, Changed<ResponsiveGridView>)>,
    >,
    grid: Res<SnapGrid>,
) {
    for (mut position, area, location, view) in gridded.iter_mut() {
        if let Some(loc) = location {
            *position = grid.responsive_point_coordinates(*loc);
        } else if let Some(view) = view {
            let section = grid.responsive_view_coordinates(*view);
            *position = section.position;
            if let Some(mut area) = area {
                *area = section.area;
            }
        }
    }
}

pub(crate) fn setup_snap_grid(mut cmd: Commands, viewport_handle: Res<ViewportHandle>) {
    cmd.insert_resource(SnapGrid::new(viewport_handle.section.area));
}

pub(crate) struct SnapGridAttachment;
impl Attach for SnapGridAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.task(Visualizer::TASK_STARTUP).add_systems((
            setup_snap_grid.in_set(SyncPoint::Initialization),
            calculate.in_set(SyncPoint::Finish),
        ));
        visualizer.task(Visualizer::TASK_MAIN).add_systems((
            reapply.in_set(SyncPoint::SecondaryEffects),
            calculate.in_set(SyncPoint::PostProcessPreparation),
        ));
        visualizer.enable_delayed_spawn::<ResponsiveGridPoint>();
        visualizer.enable_delayed_spawn::<ResponsiveGridView>();
    }
}
