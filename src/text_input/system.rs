use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Commands, Or, Query, Res};
use bevy_ecs::query::{With, Without};
use fontdue::layout::WrapStyle;

use crate::focus::{Focus, FocusedEntity};
use crate::panel::{Panel, PanelContentArea};
use crate::text::{AlignedFonts, TextGridPlacement, TextScale};
use crate::text_input::components::{MaxCharacters, TextContentPanel, TextInput, TextInputText};
use crate::text_input::cursor::{Cursor, CursorIcon};
use crate::text_input::request::TextInputRequest;
use crate::text_input::{TextBackgroundColor, TextColor};
use crate::touch::{TouchLocation, Touched};
use crate::{
    text, Area, Color, ColorInvert, Icon, IconDescriptors, IconKey, IconSecondaryColor, IconSize,
    InterfaceContext, Layer, Location, PanelType, Position, Request, ScaleFactor, Text,
    TextGridLocation, TextLetterDimensions, TextLineStructure, TextRequest, TextWrapStyle,
    ViewArea, ViewPosition, VirtualKeyboardAdapter, VirtualKeyboardType,
};

pub(crate) fn spawn(
    mut requests: Query<(Entity, &mut Request<TextInputRequest>)>,
    mut cmd: Commands,
    fonts: Res<AlignedFonts>,
    scale_factor: Res<ScaleFactor>,
) {
    for (entity, mut request) in requests.iter_mut() {
        let inner_req = request.req.take().unwrap();
        let text_scale = TextScale::from_alignment(inner_req.alignment, scale_factor.factor);
        let character_dimensions = fonts
            .fonts
            .get(&inner_req.alignment)
            .unwrap()
            .character_dimensions('a', text_scale.px());
        let text = cmd
            .spawn(TextRequest::new(
                inner_req.view_position,
                inner_req.view_area,
                inner_req.layer,
                inner_req.hint_text.clone(),
                inner_req.alignment,
                inner_req.text_color,
                TextWrapStyle(WrapStyle::Letter),
            ))
            .id();
        let cursor_icon = cmd
            .spawn(Icon::new(
                IconDescriptors::Cursor.key(),
                inner_req.view_position,
                inner_req.view_area,
                inner_req.layer + Layer::from(1u32),
                IconSize::Custom((character_dimensions.width, character_dimensions.height)),
                inner_req.text_color,
                IconSecondaryColor::new(Color::BLANK),
            ))
            .insert(ColorInvert::on())
            .remove::<ViewPosition>()
            .remove::<ViewArea>()
            .id();
        let content_panel = cmd
            .spawn(Panel::new(
                PanelType::BorderedPanel,
                inner_req.view_position,
                inner_req.view_area,
                inner_req.layer + Layer::from(2u32),
                inner_req.background_color,
                inner_req.text_color,
            ))
            .id();
        cmd.entity(entity).insert(TextInput::new(
            inner_req.view_position,
            inner_req.view_area,
            inner_req.layer,
            TextInputText::new(text),
            CursorIcon::new(cursor_icon),
            TextContentPanel(content_panel),
            inner_req.alignment,
            inner_req.text_color,
            inner_req.background_color,
            inner_req.hint_text.len() as u32,
        ));
        cmd.entity(entity).remove::<Request<TextInputRequest>>();
    }
}
pub(crate) fn position_ties(
    moved: Query<(&ViewPosition, &TextInputText, &TextContentPanel), Changed<ViewPosition>>,
    mut text: Query<
        &mut ViewPosition,
        (
            With<Text>,
            Without<TextInputText>,
            Without<PanelContentArea>,
        ),
    >,
    mut content_panels: Query<
        &mut ViewPosition,
        (
            With<PanelContentArea>,
            Without<TextInputText>,
            Without<Text>,
        ),
    >,
) {
    for (pos, text_input_text, text_content_panel) in moved.iter() {
        let mut view_position = text.get_mut(text_input_text.entity).unwrap();
        let mut panel_view_position = content_panels.get_mut(text_content_panel.0).unwrap();
        *view_position = *pos;
        *panel_view_position = *pos;
    }
}

pub(crate) fn area_ties(
    text_inputs: Query<(&TextInputText, &TextContentPanel, &ViewArea), Changed<ViewArea>>,
    mut text: Query<
        &mut ViewArea,
        (
            With<Text>,
            Without<TextInputText>,
            Without<PanelContentArea>,
        ),
    >,
    mut content_panels: Query<
        &mut ViewArea,
        (
            With<PanelContentArea>,
            Without<TextInputText>,
            Without<Text>,
        ),
    >,
) {
    for (text_input_text, content_panel, area) in text_inputs.iter() {
        let mut text_area = text.get_mut(text_input_text.entity).unwrap();
        *text_area = *area;
        let mut content_panel_area = content_panels.get_mut(content_panel.0).unwrap();
        *content_panel_area = *area;
    }
}

pub(crate) fn open_virtual_keyboard(
    virtual_keyboard: Res<VirtualKeyboardAdapter>,
    mut focus_changed: Query<
        (
            &Focus,
            &VirtualKeyboardType,
            &CursorIcon,
            &mut Cursor,
            &TextInputText,
            &TextColor,
        ),
        Changed<Focus>,
    >,
    mut text_query: Query<(&mut text::Difference, &TextGridPlacement)>,
    mut cmd: Commands,
) {
    let mut should_close = true;
    let mut keyboard = VirtualKeyboardType::Keyboard;
    let mut system_ran = false;
    for (focus, v_key_type, cursor_icon, mut cursor, text_input_text, text_color) in
        focus_changed.iter_mut()
    {
        system_ran = true;
        if focus.focused() {
            should_close = false;
            keyboard = *v_key_type;
            cmd.entity(cursor_icon.entity).insert(ColorInvert::off());
        } else {
            if let Some(cached) = cursor.cached_location.take() {
                if let Ok((mut difference, grid_placement)) =
                    text_query.get_mut(text_input_text.entity)
                {
                    if let Some(placed_letter) = grid_placement.0.get(&cached) {
                        difference
                            .glyph_color_update
                            .insert(*placed_letter, text_color.0);
                    }
                }
            }
            cmd.entity(cursor_icon.entity).insert(ColorInvert::on());
        }
    }
    if system_ran {
        if should_close {
            virtual_keyboard.close();
        } else {
            virtual_keyboard.open(keyboard);
        }
    }
}

pub(crate) fn read_input_if_focused(
    mut focused: Query<(
        &Focus,
        &mut Cursor,
        &MaxCharacters,
        &TextInputText,
        &TextColor,
    )>,
    focused_entity: Res<FocusedEntity>,
    mut text_query: Query<(
        Entity,
        &mut Text,
        &TextLineStructure,
        &TextGridPlacement,
        &mut text::Difference,
    )>,
) {
    if let Some(entity) = focused_entity.entity {
        if let Ok((focus, mut cursor, max_characters, text_input_text, text_color)) =
            focused.get_mut(entity)
        {
            if focus.focused() {
                let (_entity, mut text, line_structure, grid_placement, mut difference) =
                    text_query.get_mut(text_input_text.entity).unwrap();
                let num_letters = text.0.len() as u32;
                if num_letters < max_characters.0 && num_letters + 32 < 126 {
                    let character = char::from(num_letters as u8 + 32);
                    text.0.push(character);
                    if cursor.location.x + 1 >= line_structure.horizontal_character_max() {
                        if cursor.location.y >= line_structure.line_max() - 1 {
                        } else {
                            cursor.location.x = 0;
                            cursor.location.y += 1;
                        }
                    } else {
                        cursor.location.x += 1;
                    }
                    let current_location = cursor.location;
                    if let Some(cached) = cursor.cached_location {
                        if let Some(letter) = grid_placement.0.get(&cached) {
                            difference.glyph_color_update.insert(*letter, text_color.0);
                        }
                    }
                    cursor.cached_location.replace(current_location);
                }
            }
        }
    }
}

pub(crate) fn set_cursor_location(
    mut clicked: Query<(
        &Position<InterfaceContext>,
        &Touched,
        &TouchLocation,
        &mut Cursor,
        &TextInputText,
        &TextLetterDimensions,
        &TextColor,
        &TextBackgroundColor,
    )>,
    mut text_entities: Query<(
        &TextLineStructure,
        &TextGridPlacement,
        &mut text::Difference,
    )>,
    scale_factor: Res<ScaleFactor>,
) {
    for (
        pos,
        touched,
        touch_location,
        mut cursor,
        text_input_text,
        character_dimensions,
        text_color,
        text_background_color,
    ) in clicked.iter_mut()
    {
        if touched.touched() {
            let click_location = touch_location.0.unwrap();
            let click_offset_x = click_location.x - pos.x;
            let click_offset_y = click_location.y - pos.y;
            let (line_structure, grid_placement, mut difference) =
                text_entities.get_mut(text_input_text.entity).unwrap();
            let ui_letter_dimensions = character_dimensions.0.to_ui(scale_factor.factor);
            let mut line_clicked = (click_offset_y / ui_letter_dimensions.height).floor() as usize;
            let potential_letter_count = line_structure
                .0
                .get(line_clicked)
                .cloned()
                .unwrap_or_default();
            if (line_clicked >= line_structure.0.len() || potential_letter_count == 0)
                && line_clicked != 0
            {
                let mut next_line_up = line_clicked - 1;
                let mut next_line_count = 0;
                while next_line_up != 0
                    && next_line_up >= line_structure.0.len() - 1
                    && next_line_count == 0
                {
                    next_line_count = line_structure
                        .0
                        .get(next_line_up)
                        .cloned()
                        .unwrap_or_default();
                    if next_line_count == 0 {
                        next_line_up -= 1;
                    }
                }
                line_clicked = next_line_up
            }
            let x_letter_location = (click_offset_x / ui_letter_dimensions.width).floor() as u32;
            let current_line_letter_count = line_structure
                .0
                .get(line_clicked)
                .cloned()
                .unwrap_or_default();
            let mut was_over = false;
            if x_letter_location > current_line_letter_count {
                was_over = true;
            }
            let x_letter_location = x_letter_location.min(current_line_letter_count);
            let x_letter_location = x_letter_location
                + (x_letter_location < (line_structure.horizontal_character_max() - 1)
                    && was_over
                    && current_line_letter_count != 0) as u32;
            let location = TextGridLocation::new(x_letter_location, line_clicked as u32);
            if let Some(cached_location) = cursor.cached_location {
                if location != cached_location {
                    if let Some(letter) = grid_placement.0.get(&cached_location) {
                        difference.glyph_color_update.insert(*letter, text_color.0);
                    }
                }
            }
            cursor.location = location;
            cursor.cached_location.replace(location);
            println!("setting cursor location: {:?}", cursor.location);
            if let Some(letter) = grid_placement.0.get(&location) {
                difference
                    .glyph_color_update
                    .insert(*letter, text_background_color.0);
            }
        }
    }
}

pub(crate) fn cursor_letter_color_filter(
    polled: Query<(&Cursor, &TextInputText, &Focus, &TextBackgroundColor)>,
    mut changed_text_buffers: Query<(&mut text::Difference, &TextGridPlacement), Changed<Color>>,
) {
    for (cursor, text_input_text, focus, text_background_color) in polled.iter() {
        if focus.focused() {
            if let Ok((mut difference, grid_placement)) =
                changed_text_buffers.get_mut(text_input_text.entity)
            {
                if let Some(letter) = grid_placement.0.get(&cursor.location) {
                    difference
                        .glyph_color_update
                        .insert(*letter, text_background_color.0);
                }
            }
        }
    }
}

pub(crate) fn update_cursor_pos(
    updated: Query<
        (
            Entity,
            &Position<InterfaceContext>,
            &Cursor,
            &TextLetterDimensions,
            &CursorIcon,
        ),
        (
            Or<(Changed<Cursor>, Changed<Position<InterfaceContext>>)>,
            Without<IconKey>,
        ),
    >,
    mut icons: Query<&mut Position<InterfaceContext>, With<IconKey>>,
    scale_factor: Res<ScaleFactor>,
) {
    for (_entity, pos, cursor, letter_dimensions, cursor_icon) in updated.iter() {
        let ui_letter_dimensions = letter_dimensions.0.to_ui(scale_factor.factor);
        let mut icon_pos = icons.get_mut(cursor_icon.entity).unwrap();
        *icon_pos = cursor_coords(*pos, cursor, ui_letter_dimensions);
    }
}

fn cursor_coords(
    pos: Position<InterfaceContext>,
    cursor: &Cursor,
    ui_letter_dimensions: Area<InterfaceContext>,
) -> Position<InterfaceContext> {
    Position::<InterfaceContext>::new(
        pos.x + cursor.location.x as f32 * ui_letter_dimensions.width,
        pos.y + cursor.location.y as f32 * ui_letter_dimensions.height,
    )
}
