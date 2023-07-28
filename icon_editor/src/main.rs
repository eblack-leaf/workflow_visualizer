use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use workflow_visualizer::bevy_ecs::prelude::{
    Entity, IntoSystemConfig, NonSend, Query, Res, ResMut, Resource,
};
use workflow_visualizer::touch::adapter::{PrimaryTouch, TouchLocation};
use workflow_visualizer::touch::component::{
    CurrentlyPressed, TouchListener, TouchTrigger, Touchable,
};
use workflow_visualizer::RawMarker;
use workflow_visualizer::ResponsiveUnit;
use workflow_visualizer::{bevy_ecs, Position, ScaleFactor, Sender, TextValue, UserSpaceSyncPoint};
use workflow_visualizer::{
    Attach, BundlePlacement, BundledIcon, Color, GfxOptions, Icon, IconBitmap, IconBitmapRequest,
    IconPixelData, IconScale, Panel, PanelType, Runner, Text, TextScaleAlignment, TextWrapStyle,
    Theme, ThemeDescriptor, Visualizer, Workflow,
};
use workflow_visualizer::{ResponsiveGridPoint, ResponsiveGridView};

#[derive(Hash, Eq, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
struct BitmapLocation {
    x: u32,
    y: u32,
}

impl BitmapLocation {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

struct Engen {
    bitmap_panel: HashMap<BitmapLocation, IconPixelData>,
}

impl Engen {
    fn write_out(&self) {
        println!("[");
        for y in 0..20 {
            for x in 0..20 {
                println!(
                    "{:?},",
                    self.bitmap_panel
                        .get(&BitmapLocation::new(x, y))
                        .unwrap()
                        .data
                );
            }
        }
        println!("]");
    }
}

impl Default for Engen {
    fn default() -> Self {
        Engen {
            bitmap_panel: {
                let mut mapping = HashMap::new();
                for x in 0..20 {
                    for y in 0..20 {
                        mapping.insert(BitmapLocation::new(x, y), IconPixelData::default());
                    }
                }
                mapping
            },
        }
    }
}

#[derive(Resource)]
struct BitmapRepr {
    fill_data: IconPixelData,
    bitmap_repr: HashMap<BitmapLocation, Entity>,
    queue: Vec<(BitmapLocation, IconPixelData)>,
    written_out: bool,
    updates_saved: bool,
}

impl BitmapRepr {
    fn color(&self) -> Color {
        Color::from(Color::OFF_WHITE).with_alpha(self.fill_data.data as f32 / 255f32)
    }
    fn new() -> Self {
        Self {
            fill_data: IconPixelData::NO_COVERAGE.into(),
            bitmap_repr: HashMap::new(),
            queue: vec![],
            written_out: false,
            updates_saved: false,
        }
    }
}

fn update(
    mut bitmap_repr: ResMut<BitmapRepr>,
    primary_touch: Res<PrimaryTouch>,
    mut text: Query<(&mut TextValue)>,
    mut icons: Query<(Entity, &mut Color)>,
    scale_factor: Res<ScaleFactor>,
    sender: NonSend<Sender<Engen>>,
    touchables: Query<(Entity, &TouchTrigger, &TouchLocation, &CurrentlyPressed)>,
) {
    let coverage_up_entity = entity_store.get("coverage-up").unwrap();
    let coverage_display_entity = entity_store.get("coverage-display").unwrap();
    let up_touched =
        if let Ok((_, trigger, _, currently_pressed)) = touchables.get(coverage_up_entity) {
            currently_pressed.currently_pressed()
        } else {
            false
        };
    if up_touched {
        let new_coverage = bitmap_repr.fill_data.data.checked_add(1);
        bitmap_repr.fill_data.data = match new_coverage {
            None => IconPixelData::FULL_COVERAGE,
            Some(new_fill) => new_fill,
        }
    }
    let coverage_down_entity = entity_store.get("coverage-down").unwrap();
    let down_touched =
        if let Ok((_, trigger, _, currently_pressed)) = touchables.get(coverage_down_entity) {
            currently_pressed.currently_pressed()
        } else {
            false
        };
    if down_touched {
        let new_coverage = bitmap_repr.fill_data.data.checked_sub(1);
        bitmap_repr.fill_data.data = match new_coverage {
            None => IconPixelData::NO_COVERAGE,
            Some(new_fill) => new_fill,
        }
    }
    if let Ok(mut coverage_text_value) = text.get_mut(coverage_display_entity) {
        coverage_text_value.0 = bitmap_repr.fill_data.data.to_string();
    }
    let write_out_entity = entity_store.get("write-out").unwrap();
    let written_out_requested = if let Ok((_, trigger, _, _)) = touchables.get(write_out_entity) {
        trigger.triggered()
    } else {
        false
    };
    if written_out_requested {
        sender.send(Action::SendUpdates(bitmap_repr.queue.drain(..).collect()));
        sender.send(Action::WriteOut);
    }
    let bitmap_panel_entity = entity_store.get("bitmap-panel").unwrap();
    let panel_currently_pressed =
        if let Ok((_, _, location, pressed)) = touchables.get(bitmap_panel_entity) {
            pressed.currently_pressed()
        } else {
            false
        };
    if panel_currently_pressed {
        let panel_touch_location = primary_touch
            .touch
            .unwrap()
            .current
            .to_interface(scale_factor.factor());
        let logical_location =
            panel_touch_location - Position::new(RawMarker::PX * 10f32, RawMarker::PX * 34f32);
        let logical_location =
            logical_location / Position::new(RawMarker::PX * 4f32, RawMarker::PX * 4f32);
        let bitmap_location =
            BitmapLocation::new(logical_location.x as u32, logical_location.y as u32);
        let new_color =
            Color::from(Color::OFF_WHITE).with_alpha(bitmap_repr.fill_data.data as f32 / 255f32);
        if let Some(entity) = bitmap_repr.bitmap_repr.get(&bitmap_location).copied() {
            // send fill data to queue
            let fill_data = bitmap_repr.fill_data;
            bitmap_repr.queue.push((bitmap_location, fill_data));
            if let Ok((entity, mut color)) = icons.get_mut(entity) {
                *color = new_color;
            }
        }
    }
}
impl Attach for BitmapRepr {
    fn attach(visualizer: &mut Visualizer) {
        visualizer
            .job
            .task(Visualizer::TASK_MAIN)
            .add_systems((update.in_set(UserSpaceSyncPoint::Process),));
        let mut bitmap_repr_data = vec![];
        // map entity to location in the grid offset from left/top
        let bitmap_panel_left = 1.near().raw_offset(4);
        let bitmap_panel_right = 4.far().raw_offset(-4);
        let bitmap_panel_top = 2.near().raw_offset(4);
        let bitmap_panel_bottom = 5.far().raw_offset(-4);
        let increment_amount = 4;
        visualizer.add_entities(vec![IconBitmapRequest::from((
            "square",
            IconBitmap::bundled(BundledIcon::Square),
        ))]);
        for y in 0..20 {
            for x in 0..20 {
                let bundle = Icon::new(
                    "square",
                    IconScale::Custom(RawMarker::PX as u32 * 4),
                    2,
                    Color::from(Color::OFF_WHITE).with_alpha(0.0),
                )
                .responsively_point_viewed(ResponsiveGridPoint::all_same((
                    bitmap_panel_left.raw_offset(increment_amount * x + 2),
                    bitmap_panel_top.raw_offset(increment_amount * y + 2),
                )));
                bitmap_repr_data.push(bundle);
            }
        }
        let mut bitmap_repr = BitmapRepr::new();
        let ids = visualizer.add_entities(bitmap_repr_data);
        let mut index = 0;
        for y in 0..20 {
            for x in 0..20 {
                bitmap_repr
                    .bitmap_repr
                    .insert(BitmapLocation::new(x, y), *ids.get(index).unwrap());
                index += 1;
            }
        }
        visualizer.add_named_entities(
            vec![EntityName::new("bitmap-panel")],
            vec![(
                Panel::new(PanelType::Border, 3, Color::MEDIUM_GREY, Color::OFF_WHITE)
                    .responsively_viewed(ResponsiveGridView::all_same((
                        (bitmap_panel_left, bitmap_panel_right),
                        (bitmap_panel_top, bitmap_panel_bottom),
                    ))),
                Touchable::new(TouchListener::on_press()),
            )],
        );
        let coverage_display_horizontal = (
            bitmap_panel_left.raw_offset(4),
            bitmap_panel_left.raw_offset(12),
        );
        let coverage_display_vertical = (
            bitmap_panel_top.raw_offset(-8),
            bitmap_panel_top.raw_offset(-2),
        );
        let coverage_display_location =
            ResponsiveGridView::all_same((coverage_display_horizontal, coverage_display_vertical));
        visualizer.add_named_entities(
            vec![EntityName::new("coverage-display")],
            vec![Text::new(
                3,
                "255",
                TextScaleAlignment::Small,
                Color::GREY,
                TextWrapStyle::word(),
            )
            .responsively_viewed(coverage_display_location)],
        );
        let coverage_down_horizontal = (
            coverage_display_horizontal.1.raw_offset(2),
            coverage_display_horizontal.1.raw_offset(7),
        );
        let coverage_down_vertical = (
            coverage_display_vertical.0.raw_offset(0),
            coverage_display_vertical.0.raw_offset(5),
        );
        let coverage_down_location =
            ResponsiveGridView::all_same((coverage_down_horizontal, coverage_down_vertical));
        let coverage_down = Panel::new(PanelType::Panel, 3, Color::RED_ORANGE, Color::OFF_WHITE)
            .responsively_viewed(coverage_down_location);
        let coverage_up_horizontal = (
            coverage_down_horizontal.1.raw_offset(2),
            coverage_down_horizontal.1.raw_offset(7),
        );
        let coverage_up_vertical = coverage_down_vertical;
        let coverage_up_location =
            ResponsiveGridView::all_same((coverage_up_horizontal, coverage_up_vertical));
        let coverage_up = Panel::new(PanelType::Panel, 3, Color::GREEN, Color::OFF_WHITE)
            .responsively_viewed(coverage_up_location);
        visualizer.add_named_entities(
            vec!["coverage-up".into(), "coverage-down".into()],
            vec![
                (coverage_up, Touchable::new(TouchListener::on_press())),
                (coverage_down, Touchable::new(TouchListener::on_press())),
            ],
        );
        let pos_neg_space_button_horizontal = (
            coverage_up_horizontal.1.raw_offset(20),
            coverage_up_horizontal.1.raw_offset(25),
        );
        let pos_neg_space_button_vertical = (
            coverage_display_vertical.0.raw_offset(0),
            coverage_display_vertical.0.raw_offset(5),
        );
        let pos_neg_space_button_location = ResponsiveGridView::all_same((
            pos_neg_space_button_horizontal,
            pos_neg_space_button_vertical,
        ));
        let pos_neg_space_button =
            Panel::new(PanelType::Panel, 3, Color::MEDIUM_GREY, Color::OFF_WHITE)
                .responsively_viewed(pos_neg_space_button_location);
        let pos_neg_space_button_text_horizontal = (
            pos_neg_space_button_horizontal.1.raw_offset(4),
            pos_neg_space_button_horizontal.1.raw_offset(24),
        );
        let pos_neg_space_button_text_vertical = (
            pos_neg_space_button_vertical.0.raw_offset(0),
            pos_neg_space_button_vertical.0.raw_offset(5),
        );
        let pos_neg_space_button_text_location = ResponsiveGridView::all_same((
            pos_neg_space_button_text_horizontal,
            pos_neg_space_button_text_vertical,
        ));
        let pos_neg_space_button_text = Text::new(
            2,
            "positive",
            TextScaleAlignment::Small,
            Color::OFF_WHITE,
            TextWrapStyle::word(),
        )
        .responsively_viewed(pos_neg_space_button_text_location);
        visualizer.add_named_entities(
            vec![EntityName::new("pos-neg-space-button")],
            vec![pos_neg_space_button],
        );
        visualizer.add_named_entities(
            vec![EntityName::new("pos-neg-space-button-text")],
            vec![pos_neg_space_button_text],
        );
        let write_out_horizontal = (
            bitmap_panel_left.raw_offset(2),
            bitmap_panel_left.raw_offset(6),
        );
        let write_out_vertical = (
            bitmap_panel_bottom.raw_offset(2),
            bitmap_panel_bottom.raw_offset(6),
        );
        let write_out_view =
            ResponsiveGridView::all_same((write_out_horizontal, write_out_vertical));
        let write_out = Panel::new(PanelType::Panel, 3, Color::BLUE, Color::OFF_WHITE)
            .responsively_viewed(write_out_view);
        visualizer.add_named_entities(
            vec!["write-out".into()],
            vec![(write_out, Touchable::new(TouchListener::on_press()))],
        );
        visualizer.job.container.insert_resource(bitmap_repr);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
enum Action {
    ExitRequest,
    SendUpdates(Vec<(BitmapLocation, IconPixelData)>),
    WriteOut,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
enum Response {
    ExitConfirmed,
    UpdatesSaved,
    WrittenOut,
}

#[async_trait]
impl Workflow for Engen {
    type Action = Action;
    type Response = Response;

    fn handle_response(visualizer: &mut Visualizer, response: Self::Response) {
        match response {
            Response::ExitConfirmed => {}
            Response::UpdatesSaved => {
                visualizer
                    .job
                    .container
                    .get_resource_mut::<BitmapRepr>()
                    .unwrap()
                    .updates_saved = true;
            }
            Response::WrittenOut => {
                visualizer
                    .job
                    .container
                    .get_resource_mut::<BitmapRepr>()
                    .unwrap()
                    .written_out = true;
            }
        }
    }

    fn exit_action() -> Self::Action {
        Action::ExitRequest
    }

    async fn handle_action(engen: Arc<Mutex<Self>>, action: Self::Action) -> Self::Response {
        match action {
            Action::ExitRequest => return Response::ExitConfirmed,
            Action::SendUpdates(data) => {
                // add to engen.
                for d in data {
                    engen.lock().unwrap().bitmap_panel.insert(d.0, d.1);
                }
                Response::UpdatesSaved
            }
            Action::WriteOut => {
                engen.lock().unwrap().write_out();
                Response::WrittenOut
            }
        }
    }

    fn is_exit_response(res: &Self::Response) -> bool {
        match res {
            Response::ExitConfirmed => true,
            _ => false,
        }
    }
}

fn main() {
    // tracing_subscriber::fmt().with_max_level(Level::TRACE).init();
    let mut visualizer = Visualizer::new(
        Theme::new(ThemeDescriptor::new().with_background(Color::OFF_BLACK)),
        GfxOptions::native_defaults().with_msaa(2),
    );
    visualizer.add_attachment::<BitmapRepr>();
    Runner::new()
        .with_desktop_dimensions((400, 600))
        .native_run::<Engen>(visualizer);
}
