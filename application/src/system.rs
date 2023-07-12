use workflow_visualizer::{Area, BundledIcon, BundleExtension, BundlePlacement, Color, EntityStore, Focus, FocusInputListener, Icon, IconBitmap, IconBitmapRequest, IconScale, InterfaceContext, Line, Panel, PlacementReference, Position, ResponsiveGridPoint, ResponsiveGridView, ResponsivePathView, ResponsiveUnit, ScaleFactor, Sender, Text, TextScaleAlignment, TextValue, TextWrapStyle, Touchable, TouchListener, TouchTrigger, Workflow};
use workflow_visualizer::bevy_ecs::prelude::{Commands, Local, NonSend, Query, Res};

use crate::workflow::{Engen, TokenName};

pub(crate) fn setup(mut cmd: Commands) {
    let mut placement_ref = PlacementReference::new();
    placement_ref.add_view(
        "header",
        ((1.near(), 4.far()), (1.near(), 1.near().raw_offset(8))),
    );
    placement_ref.add_view(
        "panel",
        ((1.near(), 4.far()), (1.near().raw_offset(10), 4.far())),
    );
    placement_ref.add_horizontal_range(
        "list_text",
        (1.near().raw_offset(2), 1.far()),
    );
    placement_ref.add_view(
        "first_text",
        (placement_ref
             .horizontal("list_text"),
         (1.near().raw_offset(12), 1.near().raw_offset(18))),
    );
    placement_ref.add_point("icon", (
        2.near(),
        placement_ref
            .vertical("first_text")
            .begin
            .raw_offset(1),
    ));
    let second_text_vertical = (
        1.near().raw_offset(22),
        placement_ref
            .vertical("first_text")
            .end
            .raw_offset(10),
    );
    let icon_point_2 =
        ResponsiveGridPoint::all_same((2.near(), second_text_vertical.0.raw_offset(1)));
    let second_text_placement = ResponsiveGridView::all_same((
        placement_ref
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
            placement_ref.view_get("panel").build().unwrap(),
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
                placement_ref.view_get("header").build().unwrap(),
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
            placement_ref.view_get("first_text").build().unwrap(),
        ))]);
    visualizer.add_entities(vec![(Line::new(line_view, 1, Color::MEDIUM_GREY), )]);
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
        placement_ref
            .view_get("first_text")
            .vertical()
            .unwrap()
            .begin,
        placement_ref
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
}

pub(crate) fn send_event(
    sender: NonSend<Sender<Engen>>,
    mut text: Query<(
        &mut TextValue,
        &Position<InterfaceContext>,
        &Area<InterfaceContext>,
    )>,
    buttons: Query<(&TouchTrigger)>,
    entity_store: Res<EntityStore>,
    mut limiter: Local<bool>,
    scale_factor: Res<ScaleFactor>,
) {
    if !*limiter {
        let action = <Engen as Workflow>::Action::GenerateOtp(TokenName("not there".to_string()));
        sender.send(action);
        *limiter = true;
    }
    for (mut t, pos, area) in text.iter_mut() {}
    if let Some(btn) = entity_store.get("edit-button") {
        if let Ok(btn_trigger) = buttons.get(btn) {
            if btn_trigger.triggered() {
                let action = <Engen as Workflow>::Action::GenerateOtp(TokenName(
                    "editing token".to_string(),
                ));
                sender.send(action);
            }
        }
    }
}
