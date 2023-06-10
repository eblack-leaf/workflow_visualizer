use workflow_visualizer::{
    Area, Color, Focus, FocusInputListener, GfxOptions, GridMarkerBias, Layer, Line, PathViewPoint,
    Position, Request, ResponsiveGridView, ResponsivePathView, ResponsiveUnit, Text,
    TextScaleAlignment, TextWrapStyle, Theme, ThemeDescriptor, Touchable, TouchListener,
    UserSpaceSyncPoint, Visualizer,
};
use workflow_visualizer::bevy_ecs::prelude::IntoSystemConfig;

use crate::system;

pub fn visualizer() -> Visualizer {
    let theme_desc = ThemeDescriptor::new().with_background(Color::DARK_GREEN);
    let mut visualizer = Visualizer::new(Theme::new(theme_desc), GfxOptions::native_defaults());
    visualizer
        .job
        .task(Visualizer::TASK_MAIN)
        .add_systems((system::send_event.in_set(UserSpaceSyncPoint::Process), ));
    visualizer.add_entities(vec![Request::new(Text::new(
        ResponsiveGridView::all_same(((1.near(), 4.far()), (1.near(), 1.far().raw_offset(-4)))),
        4,
        "hello",
        TextScaleAlignment::Small, // need to add pub type ResponsiveTextScaleAlignment = ResponsiveView<TextScaleAlignment>; + handler
        Color::RED_ORANGE,
        TextWrapStyle::word(),
    ))]);
    visualizer.add_entities(vec![(
        Request::new(Text::new(
            ResponsiveGridView::all_same(((1.near(), 4.far()), (1.far().raw_offset(-2), 2.far()))),
            3,
            "world.",
            TextScaleAlignment::Small,
            Color::RED_ORANGE,
            TextWrapStyle::word(),
        )),
        Touchable::new(TouchListener::on_press()),
        Focus::new(),
        FocusInputListener {},
    )]);
    visualizer.add_entities(vec![(Line::new(
        ResponsivePathView::all_same(vec![
            (1.near(), 1.far().raw_offset(-3)).into(),
            (4.far(), 1.far().raw_offset(-3)).into(),
        ]),
        1,
        Color::RED_ORANGE,
    ), )]);
    visualizer
}
