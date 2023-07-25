use workflow_visualizer::{
    BundlePlacement, Button, ButtonType, Color, Grid,
    Line, Panel, PanelType, ResponsiveGridView, ResponsivePathView, Sender, Text,
    TextScaleAlignment, TextValue, TextWrapStyle, TouchTrigger,
};
use workflow_visualizer::bevy_ecs::event::EventReader;
use workflow_visualizer::bevy_ecs::prelude::{Commands, DetectChanges, NonSend, Query, Res};
use workflow_visualizer::bevy_ecs::system::ResMut;

use crate::slots::{
    AddButton, Slot, SlotBlueprint, SlotFillEvent, SlotFills, SlotPaging, SlotPool, Slots,
};
use crate::workflow::{Action, Engen};

pub(crate) fn setup(mut cmd: Commands, grid: Res<Grid>, sender: NonSend<Sender<Engen>>) {
    let blueprint = SlotBlueprint::new(&grid);
    sender.send(Action::RequestTokenNames);
    cmd.insert_resource(SlotPool(vec![]));
    cmd.insert_resource(Slots(vec![]));
    cmd.insert_resource(SlotFills(vec![]));
    cmd.insert_resource(SlotPaging(0));
    let add_button_id = cmd
        .spawn(
            Button::new(
                ButtonType::Press,
                4,
                Color::RED_ORANGE,
                Color::OFF_WHITE,
                "edit",
                "",
                0,
                blueprint.button_icon_scale,
            )
                .responsively_viewed(ResponsiveGridView::all_same(blueprint.add_button_view)),
        )
        .id();
    cmd.insert_resource(AddButton(add_button_id));
    cmd.insert_resource(blueprint);
}

pub(crate) fn update_blueprint(
    mut blueprint: ResMut<SlotBlueprint>,
    grid: Res<Grid>,
    mut slots: ResMut<Slots>,
    mut cmd: Commands,
) {
    if grid.is_changed() {
        *blueprint = SlotBlueprint::new(&grid);
        let current = slots.0.len();
        let needed = blueprint.slots_per_page;
        let diff = needed as i32 - current as i32;
        if diff > 0 {
            for i in 0..diff {
                let placements = blueprint.placements(current + i as usize);
                let info_panel = cmd
                    .spawn(
                        Panel::new(
                            PanelType::Panel,
                            5,
                            Color::MEDIUM_GREEN,
                            Color::MEDIUM_GREEN,
                        )
                            .responsively_viewed(ResponsiveGridView::all_same(
                                placements.view("info-panel"),
                            )),
                    )
                    .id();
                let edit_panel = cmd
                    .spawn(
                        Panel::new(PanelType::Panel, 6, Color::RED_ORANGE, Color::RED_ORANGE)
                            .responsively_viewed(ResponsiveGridView::all_same(
                                placements.view("edit-panel"),
                            )),
                    )
                    .id();
                let delete_panel = cmd
                    .spawn(
                        Panel::new(PanelType::Panel, 7, Color::RED, Color::RED)
                            .responsively_viewed(ResponsiveGridView::all_same(
                                placements.view("delete-panel"),
                            )),
                    )
                    .id();
                let name_text = cmd
                    .spawn(
                        Text::new(
                            4,
                            "",
                            TextScaleAlignment::Custom(blueprint.info_text_scale.0),
                            Color::OFF_WHITE,
                            TextWrapStyle::letter(),
                        )
                            .responsively_viewed(ResponsiveGridView::all_same(
                                placements.view("name-text"),
                            )),
                    )
                    .id();
                let otp_text = cmd
                    .spawn(
                        Text::new(
                            4,
                            "",
                            TextScaleAlignment::Custom(blueprint.info_text_scale.0),
                            Color::OFF_WHITE,
                            TextWrapStyle::letter(),
                        )
                            .responsively_viewed(ResponsiveGridView::all_same(
                                placements.view("otp-text"),
                            )),
                    )
                    .id();
                let info_line = cmd
                    .spawn(Line::new(
                        ResponsivePathView::all_same(placements.path_view("info-line")),
                        4,
                        Color::OFF_WHITE,
                    ))
                    .id();
                let generate_button = cmd
                    .spawn(
                        Button::new(
                            ButtonType::Press,
                            4,
                            Color::GREEN,
                            Color::OFF_WHITE,
                            "edit",
                            "",
                            0,
                            blueprint.button_icon_scale,
                        )
                            .responsively_viewed(ResponsiveGridView::all_same(
                                placements.view("generate-button"),
                            )),
                    )
                    .id();
                let edit_button = cmd
                    .spawn(
                        Button::new(
                            ButtonType::Press,
                            4,
                            Color::RED_ORANGE,
                            Color::OFF_WHITE,
                            "edit",
                            "",
                            0,
                            blueprint.button_icon_scale,
                        )
                            .responsively_viewed(ResponsiveGridView::all_same(
                                placements.view("edit-button"),
                            )),
                    )
                    .id();
                let delete_button = cmd
                    .spawn(
                        Button::new(
                            ButtonType::Press,
                            4,
                            Color::RED,
                            Color::OFF_WHITE,
                            "edit",
                            "",
                            0,
                            blueprint.button_icon_scale,
                        )
                            .responsively_viewed(ResponsiveGridView::all_same(
                                placements.view("delete-button"),
                            )),
                    )
                    .id();
                let slot = Slot {
                    name_text,
                    otp_text,
                    generate_button,
                    delete_button,
                    info_line,
                    edit_button,
                    info_panel,
                    edit_panel,
                    delete_panel,
                };
                slots.0.push(slot);
            }
        } else {
            for i in diff..0 {
                // remove slot and entities
                let slot = slots.0.remove((current as i32 + i) as usize);
                cmd.entity(slot.name_text).despawn();
                cmd.entity(slot.otp_text).despawn();
                cmd.entity(slot.generate_button).despawn();
                cmd.entity(slot.delete_button).despawn();
                cmd.entity(slot.edit_button).despawn();
                cmd.entity(slot.info_line).despawn();
                cmd.entity(slot.info_panel).despawn();
                cmd.entity(slot.edit_panel).despawn();
                cmd.entity(slot.delete_panel).despawn();
            }
        }
    }
}

pub(crate) fn read_fill_event(
    mut events: EventReader<SlotFillEvent>,
    mut slot_pool: ResMut<SlotPool>,
) {
    for event in events.iter() {
        slot_pool.0 = event.tokens.clone();
    }
}

pub(crate) fn fill_slots(
    mut slot_fills: ResMut<SlotFills>,
    slot_pool: Res<SlotPool>,
    slot_blueprint: Res<SlotBlueprint>,
    paging: Res<SlotPaging>,
) {
    if slot_pool.is_changed() || slot_blueprint.is_changed() || paging.is_changed() {
        slot_fills.0.clear();
        let (start, end) = paging.range(slot_blueprint.slots_per_page);
        for i in start..end {
            let name = slot_pool.0.get(i);
            if let Some(name) = name {
                slot_fills.0.push(name.clone());
            }
        }
    }
}

pub(crate) fn process(
    slots: Res<Slots>,
    sender: NonSend<Sender<Engen>>,
    buttons: Query<(&TouchTrigger)>,
    text: Query<(&mut TextValue)>,
) {
    // check buttons and send actions of each slot
    // update text value with responses
}
