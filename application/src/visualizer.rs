use crate::system;
use workflow_visualizer::bevy_ecs::prelude::IntoSystemConfig;
use workflow_visualizer::{
    Area, Color, Focus, FocusInputListener, GfxOptions, Layer, Position, Request, TextRequest,
    TextScaleAlignment, TextWrapStyle, Theme, ThemeDescriptor, TouchListener, Touchable,
    UserSpaceSyncPoint, Visualizer,
};

pub fn visualizer() -> Visualizer {
    let theme_desc = ThemeDescriptor::new().with_background(Color::DARK_CYAN);
    let mut visualizer = Visualizer::new(Theme::new(theme_desc), GfxOptions::native_defaults());
    visualizer
        .job
        .task(Visualizer::TASK_MAIN)
        .add_systems((system::send_event.in_set(UserSpaceSyncPoint::Process),));
    visualizer.add_entities(vec![Request::new(TextRequest::new(
        Position::new(10.0, 100.0),
        Area::new(100.0, 30.0),
        Layer::new(1.0),
        "hello",
        TextScaleAlignment::Large,
        Color::GREEN,
        TextWrapStyle::word(),
    ))]);
    visualizer.add_entities(vec![(
        Request::new(TextRequest::new(
            Position::new(10.0, 130.0),
            Area::new(100.0, 30.0),
            Layer::new(1.0),
            "world.",
            TextScaleAlignment::Large,
            Color::GREEN,
            TextWrapStyle::word(),
        )),
        Touchable::new(TouchListener::on_press()),
        Focus::new(),
        FocusInputListener {},
    )]);
    visualizer
}