use crate::{
    Area, Attach, CoordinateUnit, DelayedBundle, InterfaceContext, Position, QueuedAnimation,
    Section, SyncPoint, TimeDelta, ViewportHandle, Visualizer,
};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{IntoSystemConfigs, Res, Resource};
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::{Commands, ParamSet, Query};
use std::collections::HashMap;

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum Breakpoint {
    Mobile = 550,
    Tablet = 800,
    Desktop = 1050,
}
impl Breakpoint {
    pub fn gutter(&self) -> CoordinateUnit {
        match self {
            Breakpoint::Mobile => SnapGrid::GUTTER_BASE,
            Breakpoint::Tablet => SnapGrid::GUTTER_BASE * 1.5f32,
            Breakpoint::Desktop => SnapGrid::GUTTER_BASE * 3f32,
        }
    }
    pub fn segments(&self) -> i32 {
        match self {
            Breakpoint::Mobile => 12,
            Breakpoint::Tablet => 18,
            Breakpoint::Desktop => 24,
        }
    }
    pub fn value(&self) -> CoordinateUnit {
        (*self as i32) as f32
    }
    pub fn establish(width: CoordinateUnit) -> Self {
        if width <= Self::Mobile.value() {
            Self::Mobile
        } else if width <= Self::Tablet.value() {
            Self::Tablet
        } else {
            Self::Desktop
        }
    }
}
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct GridMarker(pub i32);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GridBias {
    Near,
    Far,
}
pub trait GridUnit {
    fn near(self) -> GridLocation;
    fn far(self) -> GridLocation;
}
impl GridUnit for i32 {
    fn near(self) -> GridLocation {
        GridLocation::new(GridMarker(self), GridBias::Near)
    }

    fn far(self) -> GridLocation {
        GridLocation::new(GridMarker(self), GridBias::Far)
    }
}
#[derive(Copy, Clone)]
pub struct GridLocation {
    pub marker: GridMarker,
    pub bias: GridBias,
}
impl GridLocation {
    pub fn new(marker: GridMarker, bias: GridBias) -> Self {
        Self { marker, bias }
    }
}
#[derive(Copy, Clone)]
pub struct ResponsiveGridLocation {
    pub mobile: GridLocation,
    pub tablet: Option<GridLocation>,
    pub desktop: Option<GridLocation>,
}
#[derive(Copy, Clone)]
pub struct GridPoint {
    pub x: GridLocation,
    pub y: GridLocation,
}
impl GridPoint {
    pub fn new(x: GridLocation, y: GridLocation) -> Self {
        Self { x, y }
    }
}
#[derive(Component, Copy, Clone)]
pub struct ResponsiveGridPoint {
    pub x: ResponsiveGridLocation,
    pub y: ResponsiveGridLocation,
}
impl ResponsiveGridPoint {
    pub fn new(x: ResponsiveGridLocation, y: ResponsiveGridLocation) -> Self {
        Self { x, y }
    }
    pub fn current(&self, horizontal: Breakpoint, vertical: Breakpoint) -> GridPoint {
        let x = self.x.current(horizontal);
        let y = self.y.current(vertical);
        GridPoint::new(x, y)
    }
}
impl ResponsiveGridLocation {
    pub fn new(mobile: GridLocation) -> Self {
        Self {
            mobile,
            tablet: None,
            desktop: None,
        }
    }
    pub fn current(&self, breakpoint: Breakpoint) -> GridLocation {
        match breakpoint {
            Breakpoint::Mobile => self.mobile,
            Breakpoint::Tablet => self.tablet.unwrap_or(self.mobile),
            Breakpoint::Desktop => self.desktop.unwrap_or(self.tablet.unwrap_or(self.mobile)),
        }
    }
    pub fn with_tablet(mut self, location: GridLocation) -> Self {
        self.tablet.replace(location);
        self
    }
    pub fn with_desktop(mut self, location: GridLocation) -> Self {
        self.desktop.replace(location);
        self
    }
}
pub struct GridRange {
    pub begin: GridLocation,
    pub end: GridLocation,
}
impl GridRange {
    pub fn new(begin: GridLocation, end: GridLocation) -> Self {
        Self { begin, end }
    }
}
#[derive(Copy, Clone)]
pub struct ResponsiveGridRange {
    pub begin: ResponsiveGridLocation,
    pub end: ResponsiveGridLocation,
}
impl ResponsiveGridRange {
    pub fn new(begin: ResponsiveGridLocation, end: ResponsiveGridLocation) -> Self {
        Self { begin, end }
    }
    pub fn current(&self, breakpoint: Breakpoint) -> GridRange {
        GridRange::new(self.begin.current(breakpoint), self.end.current(breakpoint))
    }
}
pub struct GridView {
    pub horizontal: GridRange,
    pub vertical: GridRange,
}
impl GridView {
    pub fn new(horizontal: GridRange, vertical: GridRange) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}
#[derive(Component, Copy, Clone)]
pub struct ResponsiveGridView {
    pub horizontal: ResponsiveGridRange,
    pub vertical: ResponsiveGridRange,
}
impl ResponsiveGridView {
    pub fn current(
        &self,
        horizontal_breakpoint: Breakpoint,
        vertical_breakpoint: Breakpoint,
    ) -> GridView {
        GridView::new(
            self.horizontal.current(horizontal_breakpoint),
            self.vertical.current(vertical_breakpoint),
        )
    }
    pub fn new(horizontal: ResponsiveGridRange, vertical: ResponsiveGridRange) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}
pub enum GridPlacementDescriptor {
    Location(ResponsiveGridLocation),
    View(ResponsiveGridView),
}
pub struct GridPlacer {
    pub placements: HashMap<Entity, GridPlacementDescriptor>,
}
impl GridPlacer {
    pub fn new() -> Self {
        Self {
            placements: HashMap::new(),
        }
    }
    pub fn add(&mut self, _placement: GridPlacementDescriptor) {
        todo!()
    }
}
#[derive(Copy, Clone)]
pub struct Column {
    pub content: CoordinateUnit,
    pub gutter: CoordinateUnit,
    pub breakpoint: Breakpoint,
}
impl Column {
    pub fn new(width: CoordinateUnit, breakpoint: Breakpoint) -> Self {
        Self {
            content: (width - breakpoint.gutter() * (breakpoint.segments() + 1) as f32)
                / breakpoint.segments() as f32,
            gutter: breakpoint.gutter(),
            breakpoint,
        }
    }
}
#[derive(Copy, Clone)]
pub struct Row {
    pub content: CoordinateUnit,
    pub gutter: CoordinateUnit,
    pub breakpoint: Breakpoint,
}
#[derive(Copy, Clone)]
pub enum GridDirection {
    Horizontal,
    Vertical,
}
impl Row {
    pub fn new(height: CoordinateUnit, breakpoint: Breakpoint) -> Self {
        Self {
            content: (height - breakpoint.gutter() * (breakpoint.segments() + 1) as f32)
                / breakpoint.segments() as f32,
            gutter: breakpoint.gutter(),
            breakpoint,
        }
    }
}
/// Macro placement tool segmented into fixed number of columns/rows.
#[derive(Resource, Copy, Clone)]
pub struct SnapGrid {
    pub column: Column,
    pub row: Row,
}
impl SnapGrid {
    pub const GUTTER_BASE: CoordinateUnit = 4f32;
    pub fn new(area: Area<InterfaceContext>) -> Self {
        Self {
            column: Column::new(area.width, Breakpoint::establish(area.width)),
            row: Row::new(area.height, Breakpoint::establish(area.height)),
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
        self.view_coordinates(view.current(self.column.breakpoint, self.row.breakpoint))
    }
    pub fn responsive_point_coordinates(
        &self,
        point: ResponsiveGridPoint,
    ) -> Position<InterfaceContext> {
        self.point_coordinates(point.current(self.column.breakpoint, self.row.breakpoint))
    }
    pub fn responsive_range_coordinates(
        &self,
        direction: GridDirection,
        range: ResponsiveGridRange,
    ) -> (CoordinateUnit, CoordinateUnit) {
        match direction {
            GridDirection::Horizontal => {
                self.range_coordinates(direction, range.current(self.column.breakpoint))
            }
            GridDirection::Vertical => {
                self.range_coordinates(direction, range.current(self.row.breakpoint))
            }
        }
    }
    pub fn responsive_location_coordinates(
        &self,
        direction: GridDirection,
        location: ResponsiveGridLocation,
    ) -> CoordinateUnit {
        let location = match direction {
            GridDirection::Horizontal => location.current(self.column.breakpoint),
            GridDirection::Vertical => location.current(self.row.breakpoint),
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
pub struct FloatLocation {
    percent: f32,
}
impl FloatLocation {
    pub fn new(percent: f32) -> Self {
        assert!(percent >= 0f32);
        assert!(percent <= 1f32);
        Self { percent }
    }
    pub fn percent(&self) -> f32 {
        self.percent
    }
}
impl From<f32> for FloatLocation {
    fn from(value: f32) -> Self {
        FloatLocation::new(value)
    }
}
pub struct FloatPoint {
    pub x: FloatLocation,
    pub y: FloatLocation,
}
pub struct FloatRange {
    pub begin: FloatLocation,
    pub end: FloatLocation,
}
impl FloatRange {
    pub fn new(begin: FloatLocation, end: FloatLocation) -> Self {
        Self { begin, end }
    }
}
pub struct FloatView {
    pub horizontal: FloatRange,
    pub vertical: FloatRange,
}
impl FloatView {
    pub fn new(horizontal: FloatRange, vertical: FloatRange) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}
pub enum FloatPlacementDescriptor {
    LocationDesc(FloatPoint),
    ViewDesc(FloatView),
}
impl FloatPlacementDescriptor {
    pub fn calculate(&self, section: Section<InterfaceContext>) -> FloatPlacement {
        match &self {
            FloatPlacementDescriptor::LocationDesc(point) => {
                FloatPlacement::FloatPosition(Position::new(
                    section.position.x + point.x.percent() * section.width(),
                    section.position.y + point.y.percent() * section.height(),
                ))
            }
            FloatPlacementDescriptor::ViewDesc(view) => {
                FloatPlacement::FloatSection(Section::from_left_top_right_bottom(
                    section.position.x + view.horizontal.begin.percent() * section.width(),
                    section.position.y + view.vertical.begin.percent() * section.height(),
                    section.position.x + view.horizontal.end.percent() * section.width(),
                    section.position.y + view.vertical.end.percent() * section.height(),
                ))
            }
        }
    }
}
pub enum FloatPlacement {
    FloatPosition(Position<InterfaceContext>),
    FloatSection(Section<InterfaceContext>),
}
#[derive(Default)]
pub struct FloatArrangement(pub HashMap<Entity, FloatPlacement>);
/// Float Layout tool for micro placements within a responsively bound section.
/// This is useful when the segments of the grid are not precise enough.
#[derive(Component)]
pub struct FloatPlacer {
    pub placements: HashMap<Entity, FloatPlacementDescriptor>,
}
impl FloatPlacer {
    pub fn new() -> Self {
        Self {
            placements: HashMap::new(),
        }
    }
    pub fn add(&mut self, entity: Entity, placement: FloatPlacementDescriptor) {
        self.placements.insert(entity, placement);
    }
    pub fn apply(&self, view_coordinates: Section<InterfaceContext>) -> FloatArrangement {
        let mut arrangement = FloatArrangement::default();
        for (entity, placement_descriptor) in self.placements.iter() {
            arrangement
                .0
                .insert(*entity, placement_descriptor.calculate(view_coordinates));
        }
        arrangement
    }
}
pub(crate) fn reapply(
    mut float_layouts: ParamSet<(
        Query<
            (
                &FloatPlacer,
                &Position<InterfaceContext>,
                &Area<InterfaceContext>,
            ),
            Or<(
                Changed<Position<InterfaceContext>>,
                Changed<Area<InterfaceContext>>,
                Changed<FloatPlacer>,
            )>,
        >,
        Query<(
            &mut Position<InterfaceContext>,
            Option<&mut Area<InterfaceContext>>,
        )>,
    )>,
) {
    let mut pos_changes = HashMap::new();
    let mut section_changes = HashMap::new();
    for (placer, pos, area) in float_layouts.p0().iter() {
        let arrangement = placer.apply(Section::new(*pos, *area));
        for (entity, placement) in arrangement.0 {
            match placement {
                FloatPlacement::FloatPosition(pos) => {
                    pos_changes.insert(entity, pos);
                }
                FloatPlacement::FloatSection(section) => {
                    section_changes.insert(entity, section);
                }
            };
        }
    }
    for change in pos_changes {
        if let Ok((mut pos, _)) = float_layouts.p1().get_mut(change.0) {
            *pos = change.1;
        }
    }
    for change in section_changes {
        if let Ok((mut pos, area)) = float_layouts.p1().get_mut(change.0) {
            *pos = change.1.position;
            *area.unwrap() = change.1.area;
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
