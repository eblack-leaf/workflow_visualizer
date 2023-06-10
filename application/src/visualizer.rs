use workflow_visualizer::bevy_ecs::prelude::IntoSystemConfig;
use workflow_visualizer::{
    Area, Color, EntityName, Focus, FocusInputListener, GfxOptions, GridMarkerBias, Layer, Line,
    Panel, PanelType, PathViewPoint, Position, Request, ResponsiveGridView, ResponsivePathView,
    ResponsiveUnit, Text, TextScaleAlignment, TextWrapStyle, Theme, ThemeDescriptor, TouchListener,
    Touchable, UserSpaceSyncPoint, Visualizer,
};

use crate::system;

pub fn visualizer() -> Visualizer {
    let theme_desc = ThemeDescriptor::new().with_background(Color::DARK_GREEN);
    let mut visualizer = Visualizer::new(Theme::new(theme_desc), GfxOptions::native_defaults());
    visualizer
        .job
        .task(Visualizer::TASK_MAIN)
        .add_systems((system::send_event.in_set(UserSpaceSyncPoint::Process),));
    let header_placement =
        ResponsiveGridView::all_same(((1.near(), 4.far()), (1.near(), 1.near().raw_offset(4))));
    let panel_view =
        ResponsiveGridView::all_same(((1.near(), 4.far()), (1.near().raw_offset(5), 4.far())));
    let first_text_placement = ResponsiveGridView::all_same((
        (1.near().raw_offset(1), 4.far()),
        (1.near().raw_offset(6), 2.near()),
    ));
    let second_text_placement = ResponsiveGridView::all_same((
        (1.near().raw_offset(1), 4.far()),
        (2.near().raw_offset(2), 3.far()),
    ));
    let line_y = 2.near().raw_offset(1);
    let line_view = ResponsivePathView::all_same(vec![
        (1.near().raw_offset(1), line_y).into(),
        (4.far().raw_offset(-1), line_y).into(),
    ]);
    let second_line_y = 4.near();
    visualizer.add_entities(vec![Request::new(Text::new(
        first_text_placement,
        4,
        "hello",
        TextScaleAlignment::Small, // need to add pub type ResponsiveTextScaleAlignment = ResponsiveView<TextScaleAlignment>; + handler
        Color::MEDIUM_GREEN,
        TextWrapStyle::word(),
    ))]);
    visualizer.add_named_entities(
        vec!["header".into()],
        vec![Request::new(Text::new(
            header_placement,
            4,
            "Otp-Manager",
            TextScaleAlignment::Medium, // need to add pub type ResponsiveTextScaleAlignment = ResponsiveView<TextScaleAlignment>; + handler
            Color::GREY,
            TextWrapStyle::word(),
        ))],
    );
    visualizer.add_entities(vec![(
        Request::new(Text::new(
            second_text_placement,
            3,
            "world.",
            TextScaleAlignment::Small,
            Color::MEDIUM_GREEN,
            TextWrapStyle::word(),
        )),
        Touchable::new(TouchListener::on_press()),
        Focus::new(),
        FocusInputListener {},
    )]);
    visualizer.add_entities(vec![(Line::new(line_view, 1, Color::GREY),)]);
    visualizer.add_entities(vec![Panel::new(
        panel_view,
        PanelType::Panel,
        Layer::new(5.0),
        Color::DARK_GREY,
        Color::GREY,
    )]);
    visualizer
}
