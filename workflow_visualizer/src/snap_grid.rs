use crate::{
    Area, Attach, CoordinateUnit, DelayedBundle, InterfaceContext,
    Position, QueuedAnimation, Section, SyncPoint, TimeDelta, ViewportHandle, Visualizer,
};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{IntoSystemConfigs, Res};
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::{Commands, Query};
use std::collections::HashMap;
use std::hash::Hash;

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
        return if width <= Self::Mobile.value() {
            Self::Mobile
        } else if width <= Self::Tablet.value() {
            Self::Tablet
        } else {
            Self::Desktop
        };
    }
}
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct GridMarker(pub i32);
#[derive(Copy, Clone)]
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
    pub fn add(&mut self, placement: GridPlacementDescriptor) {
        todo!()
    }
}
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
pub struct Row {
    pub content: CoordinateUnit,
    pub gutter: CoordinateUnit,
    pub breakpoint: Breakpoint,
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
    pub fn view_coordinates(&self, view: ResponsiveGridView) -> Section<InterfaceContext> {
        todo!()
    }
    pub fn range_coordinates(&self, range: ResponsiveGridRange) -> CoordinateUnit {
        todo!()
    }
    pub fn location_coordinates(
        &self,
        location: ResponsiveGridLocation,
    ) -> Position<InterfaceContext> {
        todo!()
    }
    pub fn animate_location(
        &self,
        begin: ResponsiveGridLocation,
        other: ResponsiveGridLocation,
        interval: TimeDelta,
        delay: Option<TimeDelta>,
    ) -> (
        QueuedAnimation<Position<InterfaceContext>>,
        DelayedBundle<ResponsiveGridLocation>,
    ) {
        todo!()
    }
    pub fn animate_view(
        &self,
        begin: ResponsiveGridView,
        end: ResponsiveGridView,
        interval: TimeDelta,
        delay: Option<TimeDelta>,
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
            Option<&ResponsiveGridLocation>,
            Option<&ResponsiveGridView>,
        ),
        Or<(Changed<ResponsiveGridLocation>, Changed<ResponsiveGridView>)>,
    >,
    grid: Res<SnapGrid>,
) {
    for (mut position, mut area, location, view) in gridded.iter_mut() {
        if let Some(loc) = location {
            *position = grid.location_coordinates(*loc);
        } else if let Some(view) = view {
            let section = grid.view_coordinates(*view);
            *position = section.position;
            if let Some(area) = area {
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
        assert_eq!(percent <= 0f32, true);
        assert_eq!(percent >= 1f32, true);
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
    Location(FloatLocation),
    View(FloatView),
}
impl FloatPlacementDescriptor {
    pub fn calculate(&self, section: Section<InterfaceContext>) -> FloatPlacement {
        todo!()
    }
}
pub enum FloatPlacement {
    Position(Position<InterfaceContext>),
    Section(Section<InterfaceContext>),
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
    float_layouts: Query<
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
    mut cmd: Commands,
) {
    for (placer, pos, area) in float_layouts.iter() {
        let arrangement = placer.apply(Section::new(*pos, *area));
        for (entity, placement) in arrangement.0 {
            cmd.entity(entity).insert(match placement {
                FloatPlacement::Position(pos) => pos,
                FloatPlacement::Section(section) => section,
            })
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
        visualizer.enable_delayed_spawn::<ResponsiveGridLocation>();
        visualizer.enable_delayed_spawn::<ResponsiveGridView>();
    }
}
