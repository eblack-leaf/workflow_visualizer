use workflow_visualizer::bevy_ecs::prelude::IntoSystemConfig;
use workflow_visualizer::{
    Area, BundlePlacement, BundledIcon, Color, EntityName, Focus, FocusInputListener, GfxOptions,
    Grid, GridMarkerBias, HorizontalSpan, Icon, IconBitmap, IconBitmapRequest, IconScale, Layer,
    Line, Panel, PanelType, Position, ResponsiveGridPoint, ResponsiveGridView,
    ResponsivePathView, ResponsiveUnit, Text, TextScaleAlignment, TextWrapStyle, Theme,
    ThemeDescriptor, TouchListener, Touchable, UserSpaceSyncPoint, Visualizer,
};

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
    let header_placement =
        ResponsiveGridView::all_same(((1.near(), 4.far()), (1.near(), 1.near().raw_offset(8))));
    let panel_view =
        ResponsiveGridView::all_same(((1.near(), 4.far()), (1.near().raw_offset(10), 4.far())));
    let list_text_horizontal_range = (1.near().raw_offset(2), 1.far());
    let first_text_vertical = (1.near().raw_offset(12), 1.near().raw_offset(18));
    let first_text_placement =
        ResponsiveGridView::all_same((list_text_horizontal_range, first_text_vertical));
    let icon_point = ResponsiveGridPoint::all_same((2.near(), first_text_vertical.0.raw_offset(1)));
    let second_text_vertical = (
        1.near().raw_offset(22),
        first_text_placement
            .get_span(&HorizontalSpan::Four)
            .vertical
            .end
            .raw_offset(10),
    );
    let icon_point_2 =
        ResponsiveGridPoint::all_same((2.near(), second_text_vertical.0.raw_offset(1)));
    let second_text_placement =
        ResponsiveGridView::all_same((list_text_horizontal_range, second_text_vertical));
    let line_y = 1.near().raw_offset(20);
    let line_view = ResponsivePathView::all_same(vec![
        (1.near().raw_offset(2), line_y).into(),
        (4.far().raw_offset(-2), line_y).into(),
    ]);
    let second_line_y = 4.near();
    visualizer.add_entities(vec![
        Panel::new(
            PanelType::Panel,
            Layer::new(5.0),
            Color::DARK_GREY,
            Color::GREY,
        )
        .responsively_viewed(panel_view),
    ]);
    visualizer.add_named_entities(
        vec!["header".into()],
        vec![
            Text::new(
                4,
                "credentials",
                TextScaleAlignment::Large, // need to add pub type ResponsiveTextScaleAlignment = ResponsiveView<TextScaleAlignment>; + handler
                Color::GREY,
                TextWrapStyle::word(),
            )
            .responsively_viewed(header_placement),
        ],
    );
    visualizer.add_entities(vec![
        Text::new(
            3,
            "work",
            TextScaleAlignment::Small, // need to add pub type ResponsiveTextScaleAlignment = ResponsiveView<TextScaleAlignment>; + handler
            Color::GREY,
            TextWrapStyle::word(),
        )
        .responsively_viewed(first_text_placement),
    ]);
    visualizer.add_entities(vec![(Line::new(line_view, 1, Color::MEDIUM_GREY),)]);
    visualizer.add_entities(vec![(

            Text::new(
                3,
                "school",
                TextScaleAlignment::Small,
                Color::GREY,
                TextWrapStyle::word(),
            )
            .responsively_viewed(second_text_placement),
        Touchable::new(TouchListener::on_press()),
        Focus::new(),
        FocusInputListener::default(),
    )]);
    visualizer.add_entities(vec![IconBitmapRequest::from((
        "edit",
        IconBitmap::bundled(BundledIcon::Edit),
    ))]);
    // visualizer.add_entities(vec![IconBitmapRequest::from((
    //     "square",
    //     IconBitmap::bundled(BundledIcon::Square),
    // ))]);
    visualizer.add_entities(vec![
        Icon::new("edit", IconScale::Small, 3, Color::GREY).responsively_point_viewed(icon_point),
    ]);
    visualizer.add_entities(vec![
        Icon::new("edit", IconScale::Small, 3, Color::GREY).responsively_point_viewed(icon_point_2),
    ]);
    visualizer
}
