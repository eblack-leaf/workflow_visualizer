use workflow_visualizer::{bevy_ecs, Button, ButtonBorder, ButtonType, Color, Idle, ResponsiveGridView, ResponsiveUnit, Sender, Triggered};
use workflow_visualizer::bevy_ecs::change_detection::{DetectChanges, Res, ResMut};
use workflow_visualizer::bevy_ecs::entity::Entity;
use workflow_visualizer::bevy_ecs::prelude::{Commands, Local, NonSend, Query, Resource};

use crate::Engen;
use crate::entry_list::{EntryListLayout, ListDimensions};
use crate::entry_list::EntryScale;
use crate::paging::{PageLeft, PageRight};
use crate::workflow::{Action, Token, TokenName};

#[derive(Resource)]
pub(crate) struct AddButton(pub(crate) Entity);

pub(crate) fn setup_bottom_panel_buttons(mut cmd: Commands, entry_scale: Res<EntryScale>) {
    let add_button_entity = cmd
        .spawn(Button::new(
            ButtonType::Press,
            5,
            Color::RED_ORANGE,
            Color::DARK_GREY,
            "add",
            "",
            15,
            entry_scale.button_icon_scale,
            ButtonBorder::None,
        ))
        .id();
    let add_button = AddButton(add_button_entity);
    cmd.insert_resource(add_button);
    let page_left_entity = cmd
        .spawn(Button::new(
            ButtonType::Press,
            5,
            Color::OFF_WHITE,
            Color::DARK_GREY,
            "page_left",
            "",
            15,
            entry_scale.button_icon_scale,
            ButtonBorder::None,
        ))
        .id();
    let page_right_entity = cmd
        .spawn(Button::new(
            ButtonType::Press,
            5,
            Color::OFF_WHITE,
            Color::DARK_GREY,
            "page_right",
            "",
            15,
            entry_scale.button_icon_scale,
            ButtonBorder::None,
        ))
        .id();
    cmd.insert_resource(PageLeftButton(page_left_entity));
    cmd.insert_resource(PageRightButton(page_right_entity));
}

pub(crate) fn place_bottom_panel_buttons(
    add_button: Res<AddButton>,
    entry_list_layout: Res<EntryListLayout>,
    page_left_button: Res<PageLeftButton>,
    page_right_button: Res<PageRightButton>,
    list_dimensions: Res<ListDimensions>,
    mut cmd: Commands,
) {
    if entry_list_layout.is_changed() {
        let horizontal_start =
            entry_list_layout.horizontal_markers.0 / 2 - list_dimensions.entry.0 / 2;
        let horizontal = (
            1.near().raw_offset(horizontal_start),
            1.near()
                .raw_offset(horizontal_start + list_dimensions.entry.0),
        );
        let vertical_start = entry_list_layout.vertical_markers.0 + list_dimensions.padding.0;
        let vertical = (
            1.near().raw_offset(vertical_start),
            1.near()
                .raw_offset(vertical_start + list_dimensions.entry.0),
        );
        let view = (horizontal, vertical);
        cmd.entity(add_button.0)
            .insert(ResponsiveGridView::all_same(view));
        let page_left_horizontal = (
            horizontal
                .0
                .raw_offset(-list_dimensions.padding.0 - list_dimensions.entry.0),
            horizontal.0.raw_offset(-list_dimensions.padding.0),
        );
        let page_right_horizontal = (
            horizontal.1.raw_offset(list_dimensions.padding),
            horizontal
                .1
                .raw_offset(list_dimensions.padding + list_dimensions.entry),
        );
        let page_left_view = (page_left_horizontal, vertical);
        let page_right_view = (page_right_horizontal, vertical);
        cmd.entity(page_left_button.0)
            .insert(ResponsiveGridView::all_same(page_left_view));
        cmd.entity(page_right_button.0)
            .insert(ResponsiveGridView::all_same(page_right_view));
    }
}

#[derive(Resource)]
pub(crate) struct PageLeftButton(pub(crate) Entity);

#[derive(Resource)]
pub(crate) struct PageRightButton(pub(crate) Entity);

pub(crate) fn process_bottom_panel_buttons(
    mut page_right: ResMut<PageRight>,
    mut page_left: ResMut<PageLeft>,
    add: Res<AddButton>,
    page_left_button: Res<PageLeftButton>,
    page_right_button: Res<PageRightButton>,
    buttons: Query<&Triggered>,
    sender: NonSend<Sender<Engen>>,
    mut counter: Local<u32>,
    mut _idle: ResMut<Idle>,
) {
    #[cfg(target_family = "wasm")] {
        _idle.can_idle = false;
    }
    if let Ok(trigger) = buttons.get(add.0) {
        if trigger.active() {
            // add logic
            let name = "hi".to_string() + counter.to_string().as_str();
            *counter += 1;
            sender.send(Action::AddToken((TokenName(name), Token("362452".into()))));
        }
    }
    if let Ok(trigger) = buttons.get(page_left_button.0) {
        if trigger.active() {
            page_left.0 = true;
        }
    }
    if let Ok(trigger) = buttons.get(page_right_button.0) {
        if trigger.active() {
            page_right.0 = true;
        }
    }
}
