use workflow_visualizer::bevy_ecs::prelude::IntoSystemConfig;
use workflow_visualizer::{BundleExtension, GridMarkerBias};
use workflow_visualizer::ResponsiveUnit;
use workflow_visualizer::{
    Area, BundlePlacement, BundledIcon, Button, ButtonType, Color, EntityName, Focus,
    FocusInputListener, GfxOptions, Grid, GridViewBuilder, HorizontalSpan, Icon, IconBitmap,
    IconBitmapRequest, IconScale, Layer, Line, Panel, PanelType, PlacementBuilder, Position,
    ResponsivePathView, Text, TextScaleAlignment, TextWrapStyle, Theme, ThemeDescriptor,
    TouchListener, Touchable, UserSpaceSyncPoint, Visualizer,
};
use workflow_visualizer::{ResponsiveGridPoint, ResponsiveGridView};

use crate::system;

pub fn visualizer() -> Visualizer {
    let theme_desc = ThemeDescriptor::new().with_background(Color::OFF_BLACK);
    let mut visualizer = Visualizer::new(
        Theme::new(theme_desc),
        GfxOptions::native_defaults().with_msaa(1),
    );
    visualizer
        .job
        .task(Visualizer::TASK_MAIN)
        .add_systems((system::send_event.in_set(UserSpaceSyncPoint::Process),));
    let mut placement_builder = PlacementBuilder::new();
    placement_builder.add(
        "header",
        ((1.near(), 4.far()), (1.near(), 1.near().raw_offset(8))),
    );
    placement_builder.add(
        "panel",
        ((1.near(), 4.far()), (1.near().raw_offset(10), 4.far())),
    );
    placement_builder.add(
        "list_text",
        GridViewBuilder::new().with_horizontal((1.near().raw_offset(2), 1.far())),
    );
    placement_builder.add(
        "first_text",
        GridViewBuilder::new()
            .with_horizontal(
                placement_builder
                    .view_get("list_text")
                    .horizontal()
                    .unwrap(),
            )
            .with_vertical((1.near().raw_offset(12), 1.near().raw_offset(18))),
    );
    placement_builder.add_point("icon", (
        2.near(),
        placement_builder
            .view_get("first_text")
            .vertical()
            .unwrap()
            .begin
            .raw_offset(1),
    ));
    let second_text_vertical = (
        1.near().raw_offset(22),
        placement_builder
            .view_get("first_text")
            .vertical()
            .unwrap()
            .end
            .raw_offset(10),
    );
    let icon_point_2 =
        ResponsiveGridPoint::all_same((2.near(), second_text_vertical.0.raw_offset(1)));
    let second_text_placement = ResponsiveGridView::all_same((
        placement_builder
            .view_get("list_text")
            .horizontal()
            .unwrap(),
        second_text_vertical.into(),
    ));
    let line_y = 1.near().raw_offset(20);
    let line_view = ResponsivePathView::all_same(vec![
        (1.near().raw_offset(2), line_y).into(),
        (4.far().raw_offset(-2), line_y).into(),
    ]);
    let second_line_y = 4.near();
    visualizer.add_entities(vec![Panel::new(
        PanelType::Panel,
        Layer::new(5.0),
        Color::DARK_GREY,
        Color::GREY,
    )
    .responsively_viewed(ResponsiveGridView::all_same(
        placement_builder.view_get("panel").build().unwrap(),
    ))]);
    visualizer.add_named_entities(
        vec!["header"],
        vec![Text::new(
            4,
            "credentials",
            TextScaleAlignment::Large, // need to add pub type ResponsiveTextScaleAlignment = ResponsiveView<TextScaleAlignment>; + handler
            Color::GREY,
            TextWrapStyle::word(),
        )
        .responsively_viewed(ResponsiveGridView::all_same(
            placement_builder.view_get("header").build().unwrap(),
        ))],
    );
    visualizer.add_entities(vec![Text::new(
        3,
        "work",
        TextScaleAlignment::Small, // need to add pub type ResponsiveTextScaleAlignment = ResponsiveView<TextScaleAlignment>; + handler
        Color::GREY,
        TextWrapStyle::word(),
    )
    .responsively_viewed(ResponsiveGridView::all_same(
        placement_builder.view_get("first_text").build().unwrap(),
    ))]);
    visualizer.add_entities(vec![(Line::new(line_view, 1, Color::MEDIUM_GREY),)]);
    visualizer.add_entities(vec![(
        Text::new(
            3,
            "school",
            TextScaleAlignment::Small,
            Color::GREY,
            TextWrapStyle::word(),
        )
        .responsively_viewed(second_text_placement)
            .extend(Touchable::new(TouchListener::on_press()))
            .extend(Focus::new())
            .extend(FocusInputListener::default()),
    )]);
    visualizer.add_entities(vec![IconBitmapRequest::from((
        "edit",
        IconBitmap::bundled(BundledIcon::Edit),
    ))]);
    let btn_horizontal = (2.near(), 2.near().raw_offset(16));
    let btn_vertical = (
        placement_builder
            .view_get("first_text")
            .vertical()
            .unwrap()
            .begin,
        placement_builder
            .view_get("first_text")
            .vertical()
            .unwrap()
            .begin
            .raw_offset(6),
    );
    let btn_view = (btn_horizontal, btn_vertical);
    let button_view = ResponsiveGridView::all_same(btn_view);
    visualizer.add_named_entities(
        vec!["edit-button"],
        vec![Button::new(
            ButtonType::Press,
            3,
            Color::RED_ORANGE,
            Color::DARK_GREY,
            "edit",
            "edit",
        )
        .responsively_viewed(button_view)],
    );
    // visualizer.add_entities(vec![IconBitmapRequest::from((
    //     "square",
    //     IconBitmap::bundled(BundledIcon::Square),
    // ))]);
    visualizer
        .add_entities(vec![Icon::new("edit", IconScale::Small, 3, Color::GREY)
            .responsively_point_viewed(icon_point_2)]);
    visualizer
}
