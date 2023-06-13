use workflow_visualizer::{
    Area, BundledIcon, Color, EntityName, Focus, FocusInputListener, GfxOptions, Grid,
    GridMarkerBias, HorizontalSpan, Icon, IconBitmap, IconBitmapRequest, IconScale, Layer, Line,
    Panel, PanelType, Position, Request, ResponsiveGridPoint, ResponsiveGridView,
    ResponsivePathView, ResponsiveUnit, Text, TextScaleAlignment, TextWrapStyle, Theme,
    ThemeDescriptor, Touchable, TouchListener, UserSpaceSyncPoint, Visualizer,
};
use workflow_visualizer::bevy_ecs::prelude::IntoSystemConfig;

use crate::system;

pub fn visualizer() -> Visualizer {
    let theme_desc = ThemeDescriptor::new().with_background(Color::OFF_BLACK);
    let mut visualizer = Visualizer::new(Theme::new(theme_desc), GfxOptions::native_defaults());
    visualizer
        .job
        .task(Visualizer::TASK_MAIN)
        .add_systems((system::send_event.in_set(UserSpaceSyncPoint::Process),));
    let header_placement =
        ResponsiveGridView::all_same(((1.near(), 4.far()), (1.near(), 1.near().raw_offset(4))));
    let panel_view =
        ResponsiveGridView::all_same(((1.near(), 4.far()), (1.near().raw_offset(5), 4.far())));
    let list_text_horizontal_range = (1.near().raw_offset(1), 1.far());
    let first_text_placement = ResponsiveGridView::all_same((
        list_text_horizontal_range,
        (1.near().raw_offset(6), 1.near().raw_offset(9)),
    ));
    let icon_point = ResponsiveGridPoint::all_same((2.near(), 1.near().raw_offset(6)));
    let second_text_placement = ResponsiveGridView::all_same((
        list_text_horizontal_range,
        (
            1.near().raw_offset(11),
            first_text_placement
                .get_span(&HorizontalSpan::Four)
                .vertical
                .end
                .raw_offset(Grid::text_height_markers() + 1),
        ),
    ));
    let line_y = 1.near().raw_offset(10);
    let line_view = ResponsivePathView::all_same(vec![
        (1.near().raw_offset(1), line_y).into(),
        (4.far().raw_offset(-1), line_y).into(),
    ]);
    let second_line_y = 4.near();
    visualizer.add_entities(vec![Panel::new(
        panel_view,
        PanelType::Panel,
        Layer::new(5.0),
        Color::DARK_GREY,
        Color::GREY,
    )]);
    visualizer.add_named_entities(
        vec!["header".into()],
        vec![Request::new(Text::new(
            header_placement,
            4,
            "credentials",
            TextScaleAlignment::Large, // need to add pub type ResponsiveTextScaleAlignment = ResponsiveView<TextScaleAlignment>; + handler
            Color::GREY,
            TextWrapStyle::word(),
        ))],
    );
    visualizer.add_entities(vec![Request::new(Text::new(
        first_text_placement,
        3,
        "work",
        TextScaleAlignment::Medium, // need to add pub type ResponsiveTextScaleAlignment = ResponsiveView<TextScaleAlignment>; + handler
        Color::GREY,
        TextWrapStyle::word(),
    ))]);
    visualizer.add_entities(vec![(Line::new(line_view, 1, Color::MEDIUM_GREY), )]);
    visualizer.add_entities(vec![(
        Request::new(Text::new(
            second_text_placement,
            3,
            "school",
            TextScaleAlignment::Medium,
            Color::GREY,
            TextWrapStyle::word(),
        )),
        Touchable::new(TouchListener::on_press()),
        Focus::new(),
        FocusInputListener {},
    )]);
    visualizer.add_entities(vec![IconBitmapRequest::from((
        "something",
        IconBitmap::bundled(BundledIcon::Something),
    ))]);
    visualizer.add_entities(vec![Request::new(Icon::new(
        "something",
        icon_point,
        IconScale::Medium,
        3,
        Color::OFF_WHITE,
        Color::OFF_BLACK,
    ))]);
    visualizer
}
