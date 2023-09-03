use std::collections::HashMap;

use workflow_visualizer::bevy_ecs::prelude::{
    Commands, Entity, IntoSystemConfigs, Query, Res, ResMut, Resource, With,
};
use workflow_visualizer::{
    bevy_ecs, ActiveInteraction, Attach, BundleExtension, BundlePlacement, BundledIcon, Button,
    ButtonBorder, ButtonType, Color, GridPoint, Icon, IconBitmap, IconBitmapRequest, IconHandle,
    IconScale, Interactable, InteractionTracker, InterfaceContext, Panel, PanelTag, PanelType,
    Position, RawMarker, ResponsiveGridPoint, ResponsiveGridView, ResponsiveUnit, SyncPoint, Text,
    TextValue, TextWrapStyle, Triggered, Visualizer,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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
#[repr(i32)]
#[derive(Copy, Clone)]
pub(crate) enum IconHandles {
    ArrowLeft,
    ArrowRight,
    Generate,
    Square,
}
impl IconHandles {
    pub(crate) fn handle(&self) -> IconHandle {
        (*self as i32).into()
    }
}
impl Into<IconHandle> for IconHandles {
    fn into(self) -> IconHandle {
        self.handle()
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
                        IconHandles::Square,
                        IconScale::Custom(RawMarker::from(Pad::ICON_MARKERS).to_pixel() as u32),
                        Pad::PAD_LAYER - 1,
                        color_from_coverage(0),
                    )
                    .responsively_point_viewed(ResponsiveGridPoint::all_same(point_view)),
                )
                .id();
            pad.icons.insert(location, entity);
        }
    }
}
#[derive(Resource)]
pub(crate) struct SidePanel {
    pub(crate) coverage: Entity,
    pub(crate) coverage_left: Entity,
    pub(crate) coverage_right: Entity,
    pub(crate) write_out: Entity,
}
impl SidePanel {
    pub(crate) fn new(
        coverage: Entity,
        coverage_left: Entity,
        coverage_right: Entity,
        write_out: Entity,
    ) -> Self {
        Self {
            coverage,
            coverage_left,
            coverage_right,
            write_out,
        }
    }
}
pub(crate) fn side_panel_button_triggers(
    buttons: Query<(&Triggered, &ActiveInteraction)>,
    mut text: Query<&mut TextValue>,
    side_panel: Res<SidePanel>,
    mut pad: ResMut<Pad>,
) {
    if let Ok((_trigger, active)) = buttons.get(side_panel.coverage_left) {
        if active.active() {
            pad.current_coverage = pad.current_coverage.checked_sub(1).unwrap_or_default();
            if let Ok(mut text_val) = text.get_mut(side_panel.coverage) {
                text_val.0 = pad.current_coverage.to_string();
            }
        }
    }
    if let Ok((_trigger, active)) = buttons.get(side_panel.coverage_right) {
        if active.active() {
            pad.current_coverage = pad.current_coverage.checked_add(1).unwrap_or(255);
            if let Ok(mut text_val) = text.get_mut(side_panel.coverage) {
                text_val.0 = pad.current_coverage.to_string();
            }
        }
    }
    if let Ok((trigger, _active)) = buttons.get(side_panel.write_out) {
        if trigger.active() {
            // write out pad.coverage_values
            println!("[");
            for y in 0..Pad::ICON_DIMENSION {
                for x in 0..Pad::ICON_DIMENSION {
                    let value = *pad
                        .coverage_values
                        .get(&PadLocation::new(x, y))
                        .expect("coverage-value");
                    let comma = if x == 19 && y == 19 { "" } else { "," };
                    println!("{:?}{}", value, comma);
                }
            }
            println!("]");
        }
    }
}
pub(crate) fn setup(mut cmd: Commands) {
    let anchor: GridPoint = (1.near(), 1.near()).into();
    let total_pad_markers = Pad::ICON_MARKERS * Pad::ICON_DIMENSION;
    let panel_horizontal = (
        anchor.x.raw_offset(-1),
        anchor.x.raw_offset(total_pad_markers + 1),
    );
    let panel_vertical = (
        anchor.y.raw_offset(-1),
        anchor.y.raw_offset(total_pad_markers + 1),
    );
    let panel = cmd
        .spawn(
            Panel::new(
                PanelType::BorderedFlat,
                Pad::PAD_LAYER,
                Color::MEDIUM_GREY,
                Color::OFF_WHITE,
            )
            .responsively_viewed(ResponsiveGridView::all_same((
                panel_horizontal,
                panel_vertical,
            )))
            .extend(Interactable::default()),
        )
        .id();
    let pad = Pad::new(panel, anchor);
    let coverage_horizontal = (
        anchor
            .x
            .raw_offset(total_pad_markers)
            .raw_offset(Pad::PAD_PADDING),
        anchor
            .x
            .raw_offset(total_pad_markers + Pad::PAD_PADDING + 9),
    );
    let coverage_vertical = (
        anchor.y.raw_offset(Pad::PAD_PADDING),
        anchor.y.raw_offset(Pad::PAD_PADDING + 7),
    );
    let coverage = cmd
        .spawn(
            Text::new(
                Pad::PAD_LAYER,
                "255",
                17,
                Color::OFF_WHITE,
                TextWrapStyle::letter(),
            )
            .responsively_viewed(ResponsiveGridView::all_same((
                coverage_horizontal,
                coverage_vertical,
            ))),
        )
        .id();
    let coverage_left_horizontal = (
        coverage_horizontal.1.raw_offset(Pad::PAD_PADDING),
        coverage_horizontal
            .1
            .raw_offset(Pad::PAD_PADDING + Pad::PAD_BUTTON_SCALE as i32),
    );
    let coverage_left_vertical = (
        coverage_vertical.0.raw_offset(1),
        coverage_vertical.0.raw_offset(Pad::PAD_BUTTON_SCALE),
    );
    let icon_scale = (Pad::PAD_BUTTON_SCALE - 4) as f32 * RawMarker::PX;
    let icon_scale = icon_scale as u32;
    let coverage_left = cmd
        .spawn(
            Button::new(
                ButtonType::Press,
                Pad::PAD_LAYER,
                Color::DARK_GREY,
                Color::OFF_WHITE,
                IconHandles::ArrowLeft,
                "",
                0,
                icon_scale,
                ButtonBorder::None,
            )
            .responsively_viewed(ResponsiveGridView::all_same((
                coverage_left_horizontal,
                coverage_left_vertical,
            ))),
        )
        .id();
    let coverage_right_horizontal = (
        coverage_left_horizontal.1.raw_offset(Pad::PAD_PADDING),
        coverage_left_horizontal
            .1
            .raw_offset(Pad::PAD_PADDING + Pad::PAD_BUTTON_SCALE),
    );
    let coverage_right = cmd
        .spawn(
            Button::new(
                ButtonType::Press,
                Pad::PAD_LAYER,
                Color::DARK_GREY,
                Color::OFF_WHITE,
                IconHandles::ArrowRight,
                "",
                0,
                icon_scale,
                ButtonBorder::None,
            )
            .responsively_viewed(ResponsiveGridView::all_same((
                coverage_right_horizontal,
                coverage_left_vertical,
            ))),
        )
        .id();
    let write_out_horizontal = (coverage_horizontal.0, coverage_horizontal.0.raw_offset(40));
    let write_out_vertical = (
        coverage_vertical.1.raw_offset(Pad::PAD_PADDING * 3),
        coverage_vertical.1.raw_offset(Pad::PAD_PADDING * 3 + 7),
    );
    let write_out = cmd
        .spawn(
            Button::new(
                ButtonType::Press,
                Pad::PAD_PADDING,
                Color::DARK_GREY,
                Color::OFF_WHITE,
                IconHandles::Generate,
                "write-out",
                18,
                18,
                ButtonBorder::Border,
            )
            .responsively_viewed(ResponsiveGridView::all_same((
                write_out_horizontal,
                write_out_vertical,
            ))),
        )
        .id();
    cmd.insert_resource(pad);
    cmd.insert_resource(SidePanel::new(
        coverage,
        coverage_left,
        coverage_right,
        write_out,
    ));
}
pub(crate) fn transfer_interaction_to_icon_coverage(
    mut pad: ResMut<Pad>,
    panel: Query<(&Position<InterfaceContext>, &InteractionTracker), With<PanelTag>>,
    mut icons: Query<&mut Color>,
) {
    if let Ok((pos, tracker)) = panel.get(pad.panel) {
        if let Some(location) = tracker.location {
            let current_pad_location = PadLocation::from(location.current() - *pos);
            if let Some(icon) = pad.icons.get(&current_pad_location) {
                if let Ok(mut color) = icons.get_mut(*icon) {
                    *color = color_from_coverage(pad.current_coverage);
                }
                let current_coverage = pad.current_coverage;
                if let Some(icon_coverage) = pad.coverage_values.get_mut(&current_pad_location) {
                    *icon_coverage = current_coverage;
                }
            }
        }
    }
}
pub(crate) fn color_from_coverage(coverage: u8) -> Color {
    let normalized = coverage as f32 / 255f32;
    (normalized, normalized, normalized).into()
}
impl Pad {
    pub(crate) const ICON_MARKERS: i32 = 4;
    pub(crate) const ICON_DIMENSION: i32 = 20;
    pub(crate) const PAD_LAYER: u32 = 5;
    pub(crate) const PAD_PADDING: i32 = 4;
    pub(crate) const PAD_BUTTON_SCALE: i32 = 10;
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
pub(crate) struct PadAttachment;
impl Attach for PadAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.spawn(IconBitmapRequest::from((
            IconHandles::ArrowRight,
            IconBitmap::bundled(BundledIcon::ArrowRight),
        )));
        visualizer.spawn(IconBitmapRequest::from((
            IconHandles::ArrowLeft,
            IconBitmap::bundled(BundledIcon::ArrowLeft),
        )));
        visualizer.spawn(IconBitmapRequest::from((
            IconHandles::Generate,
            IconBitmap::bundled(BundledIcon::Generate),
        )));
        visualizer.spawn(IconBitmapRequest::from((
            IconHandles::Square,
            IconBitmap::bundled(BundledIcon::Square),
        )));
        visualizer.job.task(Visualizer::TASK_STARTUP).add_systems((
            setup.in_set(SyncPoint::PostInitialization),
            pad_icons.in_set(SyncPoint::Resolve),
        ));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            transfer_interaction_to_icon_coverage.in_set(SyncPoint::PostProcessPreparation),
            side_panel_button_triggers.in_set(SyncPoint::Process),
        ));
    }
}
