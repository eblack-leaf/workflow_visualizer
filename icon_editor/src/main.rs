use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use workflow_visualizer::{
    Attach, BundledIcon, Color, EntityName, GfxOptions, Icon, IconBitmap, IconBitmapRequest,
    IconPixelData, IconScale, Panel, PanelType, RawMarker, Request, ResponsiveGridPoint,
    ResponsiveGridView, ResponsiveUnit, Runner, Text, TextScaleAlignment, TextWrapStyle, Theme,
    ThemeDescriptor, Touchable, TouchListener, Visualizer, Workflow,
};
use workflow_visualizer::bevy_ecs;
use workflow_visualizer::bevy_ecs::prelude::{Entity, Resource};

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
    currently_drawing: bool,
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
            currently_drawing: false,
            bitmap_repr: HashMap::new(),
        }
    }
}

impl Attach for BitmapRepr {
    fn attach(visualizer: &mut Visualizer) {
        let mut bitmap_repr_data = vec![];
        // map entity to location in the grid offset from left/top
        let bitmap_panel_left = 1.near().raw_offset(3);
        let bitmap_panel_right = 4.far().raw_offset(-3);
        let bitmap_panel_top = 2.near().raw_offset(3);
        let bitmap_panel_bottom = 5.far().raw_offset(-3);
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
                        bitmap_panel_left.raw_offset(increment_amount * x),
                        bitmap_panel_top.raw_offset(increment_amount * y),
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
            bitmap_panel_top.raw_offset(-5),
            bitmap_panel_top.raw_offset(-2),
        );
        let coverage_display_location =
            ResponsiveGridView::all_same((coverage_display_horizontal, coverage_display_vertical));
        visualizer.add_named_entities(
            vec![EntityName::new("coverage-display")],
            vec![Request::new(Text::new(
                coverage_display_location,
                3,
                "255",
                TextScaleAlignment::Small,
                Color::GREY,
                TextWrapStyle::word(),
            ))],
        );
        let coverage_up_horizontal = ();
        let coverage_up_vertical = ();
        let coverage_up_location =
            ResponsiveGridView::all_same((coverage_up_horizontal, coverage_up_vertical));
        let coverage_up = Panel::new(
            coverage_up_location,
            PanelType::Panel,
            3,
            Color::GREEN,
            Color::OFF_WHITE,
        );
        let coverage_down_horizontal = ();
        let coverage_down_vertical = ();
        let coverage_down_location =
            ResponsiveGridView::all_same((coverage_down_horizontal, coverage_down_vertical));
        let coverage_down = Panel::new(
            coverage_down_location,
            PanelType::Panel,
            3,
            Color::RED_ORANGE,
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
            coverage_up_horizontal.1.raw_offset(1),
            coverage_up_horizontal.1.raw_offset(4),
        );
        let pos_neg_space_button_vertical = (
            coverage_display_vertical.1.raw_offset(-4),
            coverage_display_vertical.1,
        );
        let pos_neg_space_button_location = ResponsiveGridView::all_same((
            pos_neg_space_button_horizontal,
            pos_neg_space_button_vertical,
        ));
        let pos_neg_space_button = Request::new(Panel::new(
            pos_neg_space_button_location,
            PanelType::Border,
            3,
            Color::MEDIUM_GREY,
            Color::OFF_WHITE,
        ));
        let pos_neg_space_button_text_horizontal = (
            pos_neg_space_button_horizontal.0.raw_offset(1),
            pos_neg_space_button_horizontal.0.raw_offset(2),
        );
        let pos_neg_space_button_text_vertical = (
            pos_neg_space_button_vertical.0.raw_offset(1),
            pos_neg_space_button_vertical.1.raw_offset(-1),
        );
        let pos_neg_space_button_text_location = ResponsiveGridView::all_same((
            pos_neg_space_button_text_horizontal,
            pos_neg_space_button_text_vertical,
        ));
        let pos_neg_space_button_text = Request::new(Text::new(
            pos_neg_space_button_text_location,
            2,
            "P",
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
        GfxOptions::native_defaults(),
    );
    visualizer.add_attachment::<BitmapRepr>();
    Runner::new()
        .with_desktop_dimensions((400, 600))
        .native_run::<Engen>(visualizer);
}
