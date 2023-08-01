use std::collections::HashMap;

use workflow_visualizer::{
    BundlePlacement, Button, ButtonDespawn, ButtonType, Color, Grid, Line, Panel,
    PanelType, ResponsiveGridView, ResponsivePathView, Sender, Text, TextScaleAlignment, TextValue,
    TextWrapStyle,
};
use workflow_visualizer::{AnimationManager, TouchTrigger};
use workflow_visualizer::bevy_ecs::event::EventReader;
use workflow_visualizer::bevy_ecs::prelude::{
    Commands, DetectChanges, NonSend, Query, Res,
};
use workflow_visualizer::bevy_ecs::system::ResMut;

use crate::slots::{
    AddButton, CurrentOtpValue, OtpRead, PageLeftButton, PageRightButton, Slot, SlotBlueprint,
    SlotFadeIn, SlotFillEvent, SlotFills, SlotFillsCache, SlotPaging, SlotPool, Slots,
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
    let page_left_button_id = cmd
        .spawn(
            Button::new(
                ButtonType::Press,
                4,
                Color::OFF_WHITE,
                Color::OFF_BLACK,
                "edit",
                "",
                0,
                blueprint.button_icon_scale,
            )
            .responsively_viewed(ResponsiveGridView::all_same(blueprint.page_left_view)),
        )
        .id();
    let page_right_button_id = cmd
        .spawn(
            Button::new(
                ButtonType::Press,
                4,
                Color::OFF_WHITE,
                Color::OFF_BLACK,
                "edit",
                "",
                0,
                blueprint.button_icon_scale,
            )
            .responsively_viewed(ResponsiveGridView::all_same(blueprint.page_right_view)),
        )
        .id();
    cmd.insert_resource(AddButton(add_button_id));
    cmd.insert_resource(PageLeftButton(page_left_button_id));
    cmd.insert_resource(PageRightButton(page_right_button_id));
    cmd.insert_resource(blueprint);
    cmd.insert_resource(CurrentOtpValue(HashMap::new()));
}

pub fn update_blueprint(mut blueprint: ResMut<SlotBlueprint>, grid: Res<Grid>) {
    if grid.is_changed() {
        *blueprint = SlotBlueprint::new(&grid);
    }
}

fn create_slot(
    cmd: &mut Commands,
    blueprint: &SlotBlueprint,
    index: usize,
    name: String,
    otp_val: String,
) -> Slot {
    let placements = blueprint.placements(index);
    let info_panel = cmd
        .spawn(
            Panel::new(
                PanelType::Panel,
                5,
                Color::from(Color::MEDIUM_GREEN).with_alpha(1f32),
                Color::from(Color::MEDIUM_GREEN).with_alpha(1f32),
            )
            .responsively_viewed(ResponsiveGridView::all_same(placements.view("info-panel"))),
        )
        .id();
    let edit_panel = cmd
        .spawn(
            Panel::new(
                PanelType::Panel,
                6,
                Color::from(Color::MEDIUM_RED_ORANGE).with_alpha(1f32),
                Color::from(Color::MEDIUM_RED_ORANGE).with_alpha(1f32),
            )
            .responsively_viewed(ResponsiveGridView::all_same(placements.view("edit-panel"))),
        )
        .id();
    let delete_panel = cmd
        .spawn(
            Panel::new(
                PanelType::Panel,
                7,
                Color::from(Color::MEDIUM_RED).with_alpha(1f32),
                Color::from(Color::MEDIUM_RED).with_alpha(1f32),
            )
            .responsively_viewed(ResponsiveGridView::all_same(
                placements.view("delete-panel"),
            )),
        )
        .id();
    let name_text = cmd
        .spawn(
            Text::new(
                4,
                name,
                TextScaleAlignment::Custom(blueprint.info_text_scale.0),
                Color::from(Color::OFF_WHITE).with_alpha(1f32),
                TextWrapStyle::letter(),
            )
            .responsively_viewed(ResponsiveGridView::all_same(placements.view("name-text"))),
        )
        .id();
    let otp_text = cmd
        .spawn(
            Text::new(
                4,
                otp_val,
                TextScaleAlignment::Custom(blueprint.info_text_scale.0),
                Color::from(Color::OFF_WHITE).with_alpha(1f32),
                TextWrapStyle::letter(),
            )
            .responsively_viewed(ResponsiveGridView::all_same(placements.view("otp-text"))),
        )
        .id();
    let info_line = cmd
        .spawn(Line::new(
            ResponsivePathView::all_same(placements.path_view("info-line")),
            4,
            Color::from(Color::OFF_WHITE).with_alpha(0f32),
        ))
        .id();
    let generate_button = cmd
        .spawn(
            Button::new(
                ButtonType::Press,
                4,
                Color::from(Color::LIGHT_GREEN).with_alpha(1f32),
                Color::from(Color::DARK_GREEN).with_alpha(1f32),
                "edit",
                "",
                15,
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
                Color::from(Color::LIGHT_RED_ORANGE).with_alpha(1f32),
                Color::from(Color::DARK_RED_ORANGE).with_alpha(1f32),
                "edit",
                "",
                15,
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
                Color::from(Color::LIGHT_RED).with_alpha(1f32),
                Color::from(Color::DARK_RED).with_alpha(1f32),
                "edit",
                "",
                15,
                blueprint.button_icon_scale,
            )
            .responsively_viewed(ResponsiveGridView::all_same(
                placements.view("delete-button"),
            )),
        )
        .id();

    Slot {
        name_text,
        otp_text,
        generate_button,
        delete_button,
        info_line,
        edit_button,
        info_panel,
        edit_panel,
        delete_panel,
    }
}
fn animate_slot_fade(
    animation_manager: &mut AnimationManager<SlotFadeIn>,
    cmd: &mut Commands,
    slot: &Slot,
    fade_time: f32,
    delay_time: Option<f32>,
    starting_opacity: f32,
    interpolated_value: f32,
) {
    animation_manager.animate(
        slot.name_text,
        SlotFadeIn::new(interpolated_value),
        fade_time,
        delay_time,
    );
    animation_manager.animate(
        slot.otp_text,
        SlotFadeIn::new(interpolated_value),
        fade_time,
        delay_time,
    );
    animation_manager.animate(
        slot.info_line,
        SlotFadeIn::new(interpolated_value),
        fade_time,
        delay_time,
    );
    animation_manager.animate(
        slot.info_panel,
        SlotFadeIn::new(interpolated_value),
        fade_time,
        delay_time,
    );
    animation_manager.animate(
        slot.edit_panel,
        SlotFadeIn::new(interpolated_value),
        fade_time,
        delay_time,
    );
    animation_manager.animate(
        slot.delete_panel,
        SlotFadeIn::new(interpolated_value),
        fade_time,
        delay_time,
    );
    animation_manager.animate(
        slot.generate_button,
        SlotFadeIn::new(interpolated_value),
        fade_time,
        delay_time,
    );
    animation_manager.animate(
        slot.edit_button,
        SlotFadeIn::new(interpolated_value),
        fade_time,
        delay_time,
    );
    animation_manager.animate(
        slot.delete_button,
        SlotFadeIn::new(interpolated_value),
        fade_time,
        delay_time,
    );
    cmd.entity(slot.name_text)
        .insert((Color::from(Color::OFF_WHITE).with_alpha(starting_opacity), ));
    cmd.entity(slot.otp_text)
        .insert((Color::from(Color::OFF_WHITE).with_alpha(starting_opacity), ));
    cmd.entity(slot.info_panel)
        .insert((Color::from(Color::MEDIUM_GREEN).with_alpha(starting_opacity), ));
    cmd.entity(slot.edit_panel)
        .insert((Color::from(Color::MEDIUM_RED_ORANGE).with_alpha(starting_opacity), ));
    cmd.entity(slot.delete_panel)
        .insert((Color::from(Color::MEDIUM_RED).with_alpha(starting_opacity), ));
    cmd.entity(slot.generate_button)
        .insert((Color::from(Color::LIGHT_GREEN).with_alpha(starting_opacity), ));
    cmd.entity(slot.edit_button)
        .insert((Color::from(Color::LIGHT_RED_ORANGE).with_alpha(starting_opacity), ));
    cmd.entity(slot.delete_button)
        .insert((Color::from(Color::LIGHT_RED).with_alpha(starting_opacity), ));
    cmd.entity(slot.info_line)
        .insert((Color::from(Color::OFF_WHITE).with_alpha(starting_opacity), ));
}
fn delete_slot(cmd: &mut Commands, slot: &Slot) {
    cmd.entity(slot.name_text).despawn();
    cmd.entity(slot.otp_text).despawn();
    cmd.entity(slot.generate_button)
        .insert(ButtonDespawn::default());
    cmd.entity(slot.delete_button)
        .insert(ButtonDespawn::default());
    cmd.entity(slot.edit_button)
        .insert(ButtonDespawn::default());
    cmd.entity(slot.info_line).despawn();
    cmd.entity(slot.info_panel).despawn();
    cmd.entity(slot.edit_panel).despawn();
    cmd.entity(slot.delete_panel).despawn();
}
pub fn read_fill_event(mut events: EventReader<SlotFillEvent>, mut slot_pool: ResMut<SlotPool>) {
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
    current_otps: Res<CurrentOtpValue>,
    mut animation_manager: ResMut<AnimationManager<SlotFadeIn>>,
) {
    if slot_pool.is_changed() || slot_blueprint.is_changed() || paging.is_changed() {
        slot_fills.0.clear();
        let (start, end) = paging.range(slot_blueprint.slots_per_page);
        let mut zero_based_index = 0;
        let mut slot_despawns = vec![];
        let mut animation_delay = 0f32;
        for paged_index in start..end {
            let name = slot_pool.0.get(paged_index);
            if let Some(token_name) = name {
                if !token_name.0.is_empty() {
                    let otp_val = if let Some(current) = current_otps.0.get(token_name) {
                        current.0.clone()
                    } else {
                        "------".to_string()
                    };
                    slot_fills.0.push(token_name.clone());
                    let mut slot_needed = false;
                    if let Some(cached) = cache.0.get_mut(zero_based_index) {
                        if *cached != *token_name {
                            if let Some(slot) = slots.0.get(zero_based_index) {
                                if let Ok(mut text_val) = text_vals.get_mut(slot.name_text) {
                                    text_val.0 = token_name.0.clone();
                                }
                                if let Ok(mut text_val) = text_vals.get_mut(slot.otp_text) {
                                    text_val.0 = otp_val.clone();
                                }
                                animate_slot_fade(
                                    animation_manager.as_mut(),
                                    &mut cmd,
                                    slot,
                                    0.35,
                                    Some(animation_delay),
                                    1f32,
                                    -1f32,
                                );
                                animate_slot_fade(
                                    animation_manager.as_mut(),
                                    &mut cmd,
                                    slot,
                                    0.35,
                                    Some(0.5 + animation_delay),
                                    0f32,
                                    1f32,
                                );
                                animation_delay += 0.2;
                            } else {
                                slot_needed = true;
                            }
                        } else if slots.0.get(zero_based_index).is_none() {
                            slot_needed = true;
                        }
                        *cached = token_name.clone();
                    } else {
                        if slots.0.get(zero_based_index).is_none() {
                            slot_needed = true;
                        }
                        cache.0.insert(zero_based_index, token_name.clone());
                    }
                    if slot_needed {
                        let slot = create_slot(
                            &mut cmd,
                            &slot_blueprint,
                            zero_based_index,
                            token_name.0.clone(),
                            otp_val,
                        );
                        animate_slot_fade(
                            animation_manager.as_mut(),
                            &mut cmd,
                            &slot,
                            0.35,
                            Some(animation_delay),
                            0f32,
                            1f32,
                        );
                        animation_delay += 0.2;
                        slots.0.insert(zero_based_index, slot);
                    }
                }
            } else {
                if let Some(old) = slots.0.get(zero_based_index) {
                    slot_despawns.push(zero_based_index);
                    delete_slot(&mut cmd, old);
                }
            }
            zero_based_index += 1;
        }
        slot_despawns.sort();
        slot_despawns.reverse();
        for index in slot_despawns {
            slots.0.remove(index);
            cache.0.remove(index);
        }
    }
}
pub fn read_otp(
    mut events: EventReader<OtpRead>,
    slots: Res<Slots>,
    slot_fills: Res<SlotFills>,
    mut current_otps: ResMut<CurrentOtpValue>,
    mut text: Query<&mut TextValue>,
) {
    for event in events.iter() {
        current_otps.0.insert(event.name.clone(), event.otp.clone());
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

pub(crate) fn process(
    slots: Res<Slots>,
    slot_fills: Res<SlotFills>,
    sender: NonSend<Sender<Engen>>,
    buttons: Query<&TouchTrigger>,
    add_button: Res<AddButton>,
    page_left_button: Res<PageLeftButton>,
    page_right_button: Res<PageRightButton>,
    mut slot_pool: ResMut<SlotPool>,
    mut paging: ResMut<SlotPaging>,
    slot_blueprint: Res<SlotBlueprint>,
    mut current_otps: ResMut<CurrentOtpValue>,
    _text: Query<&mut TextValue>,
) {
    // check buttons and send actions of each slot
    if let Ok(_trigger) = buttons.get(add_button.0) {
        // spawn input prompt for name / token + confirm_button
        // confirm button trigger despawns all these elements
    }
    if let Ok(trigger) = buttons.get(page_left_button.0) {
        if trigger.triggered() {
            if paging.0 > 0 {
                paging.0 -= 1;
            }
        }
    }
    if let Ok(trigger) = buttons.get(page_right_button.0) {
        if trigger.triggered() {
            let max_page = slot_pool.0.len() as f32 / slot_blueprint.slots_per_page as f32;
            let max_page = max_page.ceil() - 1f32;
            if paging.0 < max_page as u32 {
                paging.0 += 1;
            }
        }
    }
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
                if let Some(name) = slot_fills.0.get(index) {
                    slot_pool.0.retain(|e| !(*e == *name));
                    current_otps.0.remove(name);
                    sender.send(Action::RemoveToken(name.clone()));
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
}
pub(crate) fn animations(
    mut fades: Query<(&mut Color, ), ()>,
    mut animation_manager: ResMut<AnimationManager<SlotFadeIn>>,
) {
    for (entity, managed_animation) in animation_manager.managed_animations.iter_mut() {
        if let Some(current) = managed_animation.current.as_mut() {
            if let Some(delta) = current.delta() {
                if let Ok(mut color) = fades.get_mut(*entity) {
                    let extraction = current.animator.alpha_interpolator.extract(delta);
                    *color.0 = color.0.with_alpha(color.0.alpha + extraction.0 as f32);
                    if extraction.1 {
                        current.set_done();
                    }
                }
            }
        }
    }
}
