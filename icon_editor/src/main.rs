use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use workflow_visualizer::{
    bevy_ecs, CurrentlyPressed, EntityStore, PrimaryTouch, TextValue, TouchLocation, TouchTrigger,
    UserSpaceSyncPoint,
};
use workflow_visualizer::{
    Attach, BundledIcon, Color, EntityName, GfxOptions, Icon, IconBitmap, IconBitmapRequest,
    IconPixelData, IconScale, Panel, PanelType, RawMarker, Request, ResponsiveGridPoint,
    ResponsiveGridView, ResponsiveUnit, Runner, Text, TextScaleAlignment, TextWrapStyle, Theme,
    ThemeDescriptor, Touchable, TouchListener, Visualizer, Workflow,
};
use workflow_visualizer::bevy_ecs::prelude::{
    Entity, IntoSystemConfig, Query, Res, ResMut, Resource,
};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
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
        println!("{:?}", self.bitmap_panel);
    }
}

impl Default for Engen {
    fn default() -> Self {
        Engen {
            bitmap_panel: HashMap::new(),
        }
    }
}

#[derive(Resource)]
struct BitmapRepr {
    fill_data: IconPixelData,
    bitmap_repr: HashMap<BitmapLocation, Entity>,
}

impl BitmapRepr {
    fn color(&self) -> Color {
        Color::from(Color::OFF_WHITE).with_alpha(self.fill_data.data[0] as f32 / 255f32)
    }
    fn new() -> Self {
        Self {
            fill_data: (
                IconPixelData::FULL_COVERAGE,
                IconPixelData::POSITIVE_SPACE,
                IconPixelData::LISTENABLE,
                1u8,
            )
                .into(),
            bitmap_repr: HashMap::new(),
        }
    }
}

fn update(
    mut bitmap_repr: ResMut<BitmapRepr>,
    entity_store: Res<EntityStore>,
    primary_touch: Res<PrimaryTouch>,
    mut text: Query<(&mut TextValue)>,
    mut icons: Query<(Entity, &mut Color)>,
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
        let new_coverage = bitmap_repr.fill_data.data[0].checked_add(1);
        bitmap_repr.fill_data.data[0] = match new_coverage {
            None => 255u8,
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
        let new_coverage = bitmap_repr.fill_data.data[0].checked_sub(1);
        bitmap_repr.fill_data.data[0] = match new_coverage {
            None => 0u8,
            Some(new_fill) => new_fill,
        }
    }
    if let Ok(mut coverage_text_value) = text.get_mut(coverage_display_entity) {
        coverage_text_value.0 = bitmap_repr.fill_data.data[0].to_string();
    }
    let bitmap_panel_entity = entity_store.get("bitmap-panel").unwrap();
    let panel_currently_touched =
        if let Ok((_, _, location, pressed)) = touchables.get(bitmap_panel_entity) {
            pressed.currently_pressed()
        } else {
            false
        };
    if panel_currently_touched {
        // send fill data to queue
        // send color change to entity
        let panel_touch_location = primary_touch.touch;
    }
}
impl Attach for BitmapRepr {
    fn attach(visualizer: &mut Visualizer) {
        visualizer
            .job
            .task(Visualizer::TASK_MAIN)
            .add_systems((update.in_set(UserSpaceSyncPoint::Process), ));
        let mut bitmap_repr_data = vec![];
        // map entity to location in the grid offset from left/top
        let bitmap_panel_left = 1.near().raw_offset(2);
        let bitmap_panel_right = 4.far().raw_offset(-2);
        let bitmap_panel_top = 2.near().raw_offset(2);
        let bitmap_panel_bottom = 5.far().raw_offset(-2);
        let increment_amount = 2;
        visualizer.add_entities(vec![IconBitmapRequest::from((
            "something",
            IconBitmap::bundled(BundledIcon::Something),
        ))]);
        for x in 0..20 {
            for y in 0..20 {
                let bundle = Icon::new(
                    "something",
                    ResponsiveGridPoint::all_same((
                        bitmap_panel_left.raw_offset(increment_amount * x + 1),
                        bitmap_panel_top.raw_offset(increment_amount * y + 1),
                    )),
                    IconScale::Custom(RawMarker::PX as u32 * 2),
                    2,
                    Color::OFF_WHITE,
                    Color::OFF_BLACK,
                );
                bitmap_repr_data.push(Request::new(bundle));
            }
        }
        let mut bitmap_repr = BitmapRepr::new();
        let ids = visualizer.add_entities(bitmap_repr_data);
        let mut index = 0;
        for x in 0..20 {
            for y in 0..20 {
                bitmap_repr
                    .bitmap_repr
                    .insert(BitmapLocation::new(x, y), *ids.get(index).unwrap());
            }
        }
        visualizer.add_named_entities(
            vec![EntityName::new("bitmap-panel")],
            vec![(
                Request::new(Panel::new(
                    ResponsiveGridView::all_same((
                        (bitmap_panel_left, bitmap_panel_right),
                        (bitmap_panel_top, bitmap_panel_bottom),
                    )),
                    PanelType::Border,
                    3,
                    Color::MEDIUM_GREY,
                    Color::OFF_WHITE,
                )),
                Touchable::new(TouchListener::on_press()),
            )],
        );
        let coverage_display_horizontal = (
            bitmap_panel_left.raw_offset(2),
            bitmap_panel_left.raw_offset(6),
        );
        let coverage_display_vertical = (
            bitmap_panel_top.raw_offset(-4),
            bitmap_panel_top.raw_offset(1),
        );
        let coverage_display_location =
            ResponsiveGridView::all_same((coverage_display_horizontal, coverage_display_vertical));
        visualizer.add_named_entities(
            vec![EntityName::new("coverage-display")],
            vec![Request::new(Text::new(
                coverage_display_location,
                3,
                "255",
                TextScaleAlignment::Medium,
                Color::GREY,
                TextWrapStyle::word(),
            ))],
        );
        let coverage_down_horizontal = (
            coverage_display_horizontal.1.raw_offset(1),
            coverage_display_horizontal.1.raw_offset(3),
        );
        let coverage_down_vertical = (
            coverage_display_vertical.0.raw_offset(0),
            coverage_display_vertical.0.raw_offset(2),
        );
        let coverage_down_location =
            ResponsiveGridView::all_same((coverage_down_horizontal, coverage_down_vertical));
        let coverage_down = Panel::new(
            coverage_down_location,
            PanelType::Panel,
            3,
            Color::RED_ORANGE,
            Color::OFF_WHITE,
        );
        let coverage_up_horizontal = (
            coverage_down_horizontal.1.raw_offset(1),
            coverage_down_horizontal.1.raw_offset(3),
        );
        let coverage_up_vertical = coverage_down_vertical;
        let coverage_up_location =
            ResponsiveGridView::all_same((coverage_up_horizontal, coverage_up_vertical));
        let coverage_up = Panel::new(
            coverage_up_location,
            PanelType::Panel,
            3,
            Color::GREEN,
            Color::OFF_WHITE,
        );
        visualizer.add_named_entities(
            vec!["coverage-up".into(), "coverage-down".into()],
            vec![
                (
                    Request::new(coverage_up),
                    Touchable::new(TouchListener::on_press()),
                ),
                (
                    Request::new(coverage_down),
                    Touchable::new(TouchListener::on_press()),
                ),
            ],
        );
        let pos_neg_space_button_horizontal = (
            coverage_up_horizontal.1.raw_offset(10),
            coverage_up_horizontal.1.raw_offset(12),
        );
        let pos_neg_space_button_vertical = (
            coverage_display_vertical.0.raw_offset(0),
            coverage_display_vertical.0.raw_offset(2),
        );
        let pos_neg_space_button_location = ResponsiveGridView::all_same((
            pos_neg_space_button_horizontal,
            pos_neg_space_button_vertical,
        ));
        let pos_neg_space_button = Request::new(Panel::new(
            pos_neg_space_button_location,
            PanelType::Panel,
            3,
            Color::MEDIUM_GREY,
            Color::OFF_WHITE,
        ));
        let pos_neg_space_button_text_horizontal = (
            pos_neg_space_button_horizontal.1.raw_offset(2),
            pos_neg_space_button_horizontal.1.raw_offset(12),
        );
        let pos_neg_space_button_text_vertical = (
            pos_neg_space_button_vertical.0.raw_offset(0),
            pos_neg_space_button_vertical.0.raw_offset(2),
        );
        let pos_neg_space_button_text_location = ResponsiveGridView::all_same((
            pos_neg_space_button_text_horizontal,
            pos_neg_space_button_text_vertical,
        ));
        let pos_neg_space_button_text = Request::new(Text::new(
            pos_neg_space_button_text_location,
            2,
            "positive",
            TextScaleAlignment::Small,
            Color::OFF_WHITE,
            TextWrapStyle::word(),
        ));
        visualizer.add_named_entities(
            vec![EntityName::new("pos-neg-space-button")],
            vec![pos_neg_space_button],
        );
        visualizer.add_named_entities(
            vec![EntityName::new("pos-neg-space-button-text")],
            vec![pos_neg_space_button_text],
        );
        // need to add other button for listen/not-listen
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

    fn handle_response(visualizer: &mut Visualizer, response: Self::Response) {}

    fn exit_action() -> Self::Action {
        Action::ExitRequest
    }

    fn exit_response() -> Self::Response {
        Response::ExitConfirmed
    }

    async fn handle_action(engen: Arc<Mutex<Self>>, action: Self::Action) -> Self::Response {
        match action {
            Action::ExitRequest => return Response::ExitConfirmed,
            Action::SendUpdates(data) => {
                // add to engen.
                Response::UpdatesSaved
            }
            Action::WriteOut => {
                // engen.write_out();
                Response::WrittenOut
            }
        }
    }
}

fn main() {
    let mut visualizer = Visualizer::new(
        Theme::new(ThemeDescriptor::new().with_background(Color::OFF_BLACK)),
        GfxOptions::native_defaults().with_msaa(4),
    );
    visualizer.add_attachment::<BitmapRepr>();
    Runner::new()
        .with_desktop_dimensions((400, 600))
        .native_run::<Engen>(visualizer);
}