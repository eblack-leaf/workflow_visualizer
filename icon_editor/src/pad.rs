use std::collections::HashMap;
use workflow_visualizer::bevy_ecs::prelude::{Commands, Entity, Query, ResMut, Resource, With};
use workflow_visualizer::{
    bevy_ecs, BundlePlacement, Color, GridPoint, Icon, IconScale, InteractionTracker,
    InterfaceContext, PanelTag, Position, RawMarker, ResponsiveGridPoint,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct PadLocation {
    pub(crate) x: i32,
    pub(crate) y: i32,
}
impl PadLocation {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl From<Position<InterfaceContext>> for PadLocation {
    fn from(value: Position<InterfaceContext>) -> Self {
        let icon_px = RawMarker::from(Pad::ICON_MARKERS).to_pixel();
        let x = value.x / icon_px;
        let y = value.y / icon_px;
        PadLocation {
            x: x.floor() as i32,
            y: y.floor() as i32,
        }
    }
}
#[derive(Resource)]
pub(crate) struct Pad {
    pub(crate) panel: Entity,
    pub(crate) icons: HashMap<PadLocation, Entity>,
    pub(crate) coverage_values: HashMap<PadLocation, u8>,
    pub(crate) current_coverage: u8,
    pub(crate) anchor: GridPoint,
}
pub(crate) fn pad_icons(mut pad: ResMut<Pad>, mut cmd: Commands) {
    for y in 0..Pad::ICON_DIMENSION {
        for x in 0..Pad::ICON_DIMENSION {
            let location = PadLocation::new(x, y);
            pad.coverage_values.insert(location, 0u8);
            let horizontal = pad.anchor.x.raw_offset(x * Pad::ICON_MARKERS);
            let vertical = pad.anchor.y.raw_offset(y * Pad::ICON_MARKERS);
            let point_view = GridPoint::from((horizontal, vertical));
            let entity = cmd
                .spawn(
                    Icon::new(
                        "square",
                        IconScale::Custom(RawMarker::from(Pad::ICON_MARKERS).to_pixel() as u32),
                        Pad::PAD_LAYER,
                        Color::OFF_WHITE,
                    )
                    .responsively_point_viewed(ResponsiveGridPoint::all_same(point_view)),
                )
                .id();
            pad.icons.insert(location, entity);
        }
    }
}
pub(crate) fn transfer_interaction_to_icon_coverage(
    mut pad: ResMut<Pad>,
    panel: Query<&InteractionTracker, With<PanelTag>>,
) {
    if let Ok(tracker) = panel.get(pad.panel) {
        if let Some(location) = tracker.location {
            let current_pad_location = PadLocation::from(location.current());
            if let Some(icon_coverage) = pad.coverage_values.get_mut(&current_pad_location) {
                *icon_coverage = pad.current_coverage;
            }
        }
    }
}
pub(crate) fn color_from_coverage(coverage: u8) -> Color {
    let normalized = coverage / 255u8;
    (normalized, normalized, normalized).into()
}
impl Pad {
    pub(crate) const ICON_MARKERS: i32 = 2;
    pub(crate) const ICON_DIMENSION: i32 = 20;
    pub(crate) const PAD_LAYER: u32 = 5;
    pub(crate) fn new(panel: Entity, anchor: GridPoint) -> Self {
        Self {
            panel,
            icons: HashMap::new(),
            coverage_values: HashMap::new(),
            current_coverage: 255u8,
            anchor,
        }
    }
}
