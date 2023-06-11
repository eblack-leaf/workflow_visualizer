use workflow_visualizer::{
    Area, Color, EntityName, Focus, FocusInputListener, GfxOptions, GridMarkerBias, Layer, Line,
    Panel, PanelType, PathViewPoint, Position, Request, ResponsiveGridView, ResponsivePathView,
    ResponsiveUnit, Text, TextScaleAlignment, TextWrapStyle, Theme, ThemeDescriptor, Touchable,
    TouchListener, UserSpaceSyncPoint, Visualizer,
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
    let credential_text_horizontal_range = (1.near().raw_offset(1), 1.far());
    let first_text_placement = ResponsiveGridView::all_same((
        credential_text_horizontal_range,
        (1.near().raw_offset(6), 1.near().raw_offset(9)),
    ));
    let second_text_placement = ResponsiveGridView::all_same((
        credential_text_horizontal_range,
        (1.near().raw_offset(11), 1.near().raw_offset(14)),
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
            TextScaleAlignment::Medium, // need to add pub type ResponsiveTextScaleAlignment = ResponsiveView<TextScaleAlignment>; + handler
            Color::GREY,
            TextWrapStyle::word(),
        ))],
    );
    visualizer.add_entities(vec![Request::new(Text::new(
        first_text_placement,
        4,
        "work",
        TextScaleAlignment::Small, // need to add pub type ResponsiveTextScaleAlignment = ResponsiveView<TextScaleAlignment>; + handler
        Color::GREY,
        TextWrapStyle::word(),
    ))]);
    visualizer.add_entities(vec![(Line::new(line_view, 1, Color::MEDIUM_GREY), )]);
    visualizer.add_entities(vec![(
        Request::new(Text::new(
            second_text_placement,
            3,
            "school",
            TextScaleAlignment::Small,
            Color::GREY,
            TextWrapStyle::word(),
        )),
        Touchable::new(TouchListener::on_press()),
        Focus::new(),
        FocusInputListener {},
    )]);
    visualizer
}
