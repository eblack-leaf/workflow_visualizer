use workflow_visualizer::{
    BundlePlacement, Button, ButtonType, Color, Grid, Line, Panel, PanelType, ResponsiveGridView,
    ResponsivePathView, Sender, Text, TextScaleAlignment, TextValue, TextWrapStyle,
};
use workflow_visualizer::bevy_ecs::event::EventReader;
use workflow_visualizer::bevy_ecs::prelude::{Commands, DetectChanges, NonSend, Query, Res};
use workflow_visualizer::bevy_ecs::system::ResMut;
use workflow_visualizer::TouchTrigger;

use crate::slots::{
    AddButton, OtpRead, Slot, SlotBlueprint, SlotFillEvent, SlotFills, SlotFillsCache, SlotPaging,
    SlotPool, Slots,
};
use crate::workflow::{Action, Engen};

pub fn setup(mut cmd: Commands, grid: Res<Grid>, sender: NonSend<Sender<Engen>>) {
    let blueprint = SlotBlueprint::new(&grid);
    sender.send(Action::RequestTokenNames);
    cmd.insert_resource(SlotPool(vec![]));
    cmd.insert_resource(Slots(vec![]));
    cmd.insert_resource(SlotFills(vec![]));
    cmd.insert_resource(SlotFillsCache(vec![]));
    cmd.insert_resource(SlotPaging(0));
    let add_button_id = cmd
        .spawn(
            Button::new(
                ButtonType::Press,
                4,
                Color::RED_ORANGE,
                Color::OFF_BLACK,
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

pub fn update_blueprint(
    mut blueprint: ResMut<SlotBlueprint>,
    grid: Res<Grid>,
) {
    if grid.is_changed() {
        *blueprint = SlotBlueprint::new(&grid);
    }
}

fn create_slot(
    cmd: &mut Commands,
    blueprint: &SlotBlueprint,
    index: usize,
    name: String,
) -> Slot {
    let placements = blueprint.placements(index);
    let info_panel = cmd
        .spawn(
            Panel::new(
                PanelType::Panel,
                5,
                Color::MEDIUM_GREEN,
                Color::MEDIUM_GREEN,
            )
                .responsively_viewed(ResponsiveGridView::all_same(placements.view("info-panel"))),
        )
        .id();
    let edit_panel = cmd
        .spawn(
            Panel::new(PanelType::Panel, 6, Color::MEDIUM_RED_ORANGE, Color::MEDIUM_RED_ORANGE)
                .responsively_viewed(ResponsiveGridView::all_same(placements.view("edit-panel"))),
        )
        .id();
    let delete_panel = cmd
        .spawn(
            Panel::new(PanelType::Panel, 7, Color::MEDIUM_RED, Color::MEDIUM_RED).responsively_viewed(
                ResponsiveGridView::all_same(placements.view("delete-panel")),
            ),
        )
        .id();
    let name_text = cmd
        .spawn(
            Text::new(
                4,
                name,
                TextScaleAlignment::Custom(blueprint.info_text_scale.0),
                Color::OFF_WHITE,
                TextWrapStyle::letter(),
            )
                .responsively_viewed(ResponsiveGridView::all_same(placements.view("name-text"))),
        )
        .id();
    let otp_text = cmd
        .spawn(
            Text::new(
                4,
                "------",
                TextScaleAlignment::Custom(blueprint.info_text_scale.0),
                Color::OFF_WHITE,
                TextWrapStyle::letter(),
            )
                .responsively_viewed(ResponsiveGridView::all_same(placements.view("otp-text"))),
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
                Color::LIGHT_GREEN,
                Color::DARK_GREEN,
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
                Color::LIGHT_RED_ORANGE,
                Color::DARK_RED_ORANGE,
                "edit",
                "",
                0,
                blueprint.button_icon_scale,
            )
                .responsively_viewed(ResponsiveGridView::all_same(placements.view("edit-button"))),
        )
        .id();
    let delete_button = cmd
        .spawn(
            Button::new(
                ButtonType::Press,
                4,
                Color::LIGHT_RED,
                Color::DARK_RED,
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
    slot
}

fn delete_slot(cmd: &mut Commands, slot: &Slot) {
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
pub fn read_fill_event(
    mut events: EventReader<SlotFillEvent>,
    mut slot_pool: ResMut<SlotPool>,
) {
    for event in events.iter() {
        slot_pool.0 = event.tokens.clone();
    }
}

pub fn fill_slots(
    mut slot_fills: ResMut<SlotFills>,
    slot_pool: Res<SlotPool>,
    slot_blueprint: Res<SlotBlueprint>,
    paging: Res<SlotPaging>,
    mut cache: ResMut<SlotFillsCache>,
    mut slots: ResMut<Slots>,
    mut cmd: Commands,
    mut text_vals: Query<&mut TextValue>,
) {
    if slot_pool.is_changed() || slot_blueprint.is_changed() || paging.is_changed() {
        slot_fills.0.clear();
        let (start, end) = paging.range(slot_blueprint.slots_per_page);
        let mut zero_based_index = 0;
        for paged_index in start..end {
            let name = slot_pool.0.get(paged_index);
            if let Some(token_name) = name {
                slot_fills.0.push(token_name.clone());
                let mut slot_needed = false;
                if let Some(cached) = cache.0.get_mut(zero_based_index) {
                    if *cached != *token_name {
                        if let Some(slot) = slots.0.get(zero_based_index) {
                            if let Ok(mut text_val) = text_vals.get_mut(slot.name_text) {
                                text_val.0 = token_name.0.clone();
                            }
                        } else {
                            slot_needed = true;
                        }
                    } else {
                        if let None = slots.0.get(zero_based_index) {
                            slot_needed = true;
                        }
                    }
                    *cached = token_name.clone();
                } else {
                    slot_needed = true;
                    cache.0.insert(zero_based_index, token_name.clone());
                }
                if slot_needed {
                    let slot = create_slot(
                        &mut cmd,
                        &slot_blueprint,
                        zero_based_index,
                        token_name.0.clone(),
                    );
                    slots.0.insert(zero_based_index, slot);
                }
            } else {
                let mut should_remove = false;
                if let Some(old) = slots.0.get(zero_based_index) {
                    should_remove = true;
                    delete_slot(&mut cmd, old);
                }
                if should_remove {
                    slots.0.remove(zero_based_index);
                    cache.0.remove(zero_based_index);
                }
            }
            zero_based_index += 1;
        }
    }
}
pub fn read_otp(
    mut events: EventReader<OtpRead>,
    slots: Res<Slots>,
    slot_fills: Res<SlotFills>,
    mut text: Query<&mut TextValue>,
) {
    for event in events.iter() {
        let mut index = 0;
        for fill in slot_fills.0.iter() {
            if fill.0 == event.name.0 {
                break;
            }
            index += 1;
        }
        let slot = slots.0.get(index).expect("slot");
        if let Ok(mut text_val) = text.get_mut(slot.otp_text) {
            text_val.0 = event.otp.0.clone();
            // start 30 second timer to change back
            // circle timer element
            // for now just have to re-generate to get correct info
            // anim when done changes text back
        }
    }
}
pub fn process(
    slots: Res<Slots>,
    slot_fills: Res<SlotFills>,
    sender: NonSend<Sender<Engen>>,
    buttons: Query<&TouchTrigger>,
    _text: Query<&mut TextValue>,
) {
    // check buttons and send actions of each slot
    let mut index = 0;
    for slot in slots.0.iter() {
        if let Ok(trigger) = buttons.get(slot.generate_button) {
            if trigger.triggered() {
                if let Some(name) = slot_fills.0.get(index) {
                    sender.send(Action::GenerateOtp(name.clone()));
                }
            }
        }
        if let Ok(trigger) = buttons.get(slot.delete_button) {
            if trigger.triggered() {
                if let Some(_name) = slot_fills.0.get(index) {
                    // invalidate last slot
                    // remove slot fill for slot
                    // if filled replace last with replaced
                    // sender.send(Action::RemoveToken(name.clone()));
                }
            }
        }
        if let Ok(trigger) = buttons.get(slot.edit_button) {
            if trigger.triggered() {
                if let Some(_name) = slot_fills.0.get(index) {
                    // move info panel + invalidate elements
                    // spawn edit elements
                }
            }
        }
        index += 1;
    }
    // update text value with responses
}
