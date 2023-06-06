use crate::system;
use workflow_visualizer::bevy_ecs::prelude::IntoSystemConfig;
use workflow_visualizer::{
    Area, Color, ContentOffset, Focus, FocusInputListener, GfxOptions, Layer, Position, Request,
    ResponsiveContentView, TextBundle, TextScaleAlignment, TextWrapStyle, Theme, ThemeDescriptor,
    TouchListener, Touchable, UserSpaceSyncPoint, Visualizer,
};

pub fn visualizer() -> Visualizer {
    let theme_desc = ThemeDescriptor::new().with_background(Color::DARK_CYAN);
    let mut visualizer = Visualizer::new(Theme::new(theme_desc), GfxOptions::native_defaults());
    visualizer
        .job
        .task(Visualizer::TASK_MAIN)
        .add_systems((system::send_event.in_set(UserSpaceSyncPoint::Process),));
    visualizer.add_entities(vec![Request::new(TextBundle::new(
        ResponsiveContentView::from((
            ((1, ContentOffset::Near), (2, ContentOffset::Far)),
            ((1, ContentOffset::Near), (2, ContentOffset::Far)),
        )),
        Layer::new(1.0),
        "hello",
        TextScaleAlignment::Small,
        Color::GREEN,
        TextWrapStyle::word(),
    ))]);
    visualizer.add_entities(vec![(
        Request::new(TextBundle::new(
            ResponsiveContentView::from((
                ((3, ContentOffset::Near), (4, ContentOffset::Far)),
                ((1, ContentOffset::Near), (2, ContentOffset::Far)),
            )),
            Layer::new(1.0),
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
