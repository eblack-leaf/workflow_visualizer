use crate::{Area, InterfaceContext, Position, Section};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Or, ParamSet, Query};
use std::collections::HashMap;

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
