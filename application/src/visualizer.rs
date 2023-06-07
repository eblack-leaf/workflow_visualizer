use crate::system;
use workflow_visualizer::bevy_ecs::prelude::IntoSystemConfig;
use workflow_visualizer::{
    Area, Color, ContentBias, Focus, FocusInputListener, GfxOptions, Layer, Position, RelativeUnit,
    Request, ResponsiveContentView, ResponsiveUnit, TextBundle, TextScaleAlignment, TextWrapStyle,
    Theme, ThemeDescriptor, TouchListener, Touchable, UserSpaceSyncPoint, Visualizer,
};

pub fn visualizer() -> Visualizer {
    let theme_desc = ThemeDescriptor::new().with_background(Color::DARK_CYAN);
    let mut visualizer = Visualizer::new(Theme::new(theme_desc), GfxOptions::native_defaults());
    visualizer
        .job
        .task(Visualizer::TASK_MAIN)
        .add_systems((system::send_event.in_set(UserSpaceSyncPoint::Process),));
    visualizer.add_entities(vec![Request::new(TextBundle::new(
        ResponsiveContentView::from(((1.near().relative(12), 2.far()), (1.near(), 2.far()))),
        1,
        "hello",
        TextScaleAlignment::Small,
        Color::GREEN,
        TextWrapStyle::word(),
    ))]);
    visualizer.add_entities(vec![(
        Request::new(TextBundle::new(
            ResponsiveContentView::from(((2.near(), 2.far()), (3.near(), 4.far()))),
            1,
            "world.",
            TextScaleAlignment::Small,
            Color::GREEN,
            TextWrapStyle::word(),
        )),
        Touchable::new(TouchListener::on_press()),
        Focus::new(),
        FocusInputListener {},
    )]);
    visualizer
}
