use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Commands, Or, Query, Res};

use crate::clickable::ClickState;
use crate::focus::{Focus, FocusedEntity};
use crate::text::{AlignedFonts, TextBound, TextScale};
use crate::text_input::components::{MaxCharacters, TextBackgroundIcon, TextInput, TextInputText};
use crate::text_input::cursor::{Cursor, CursorIcon};
use crate::text_input::request::TextInputRequest;
use crate::text_input::{TextBackgroundColor, TextColor};
use crate::window::ScaleFactor;
use crate::window::{VirtualKeyboardAdapter, VirtualKeyboardType};
use crate::{
    Area, ColorInvert, Icon, IconBundle, IconDescriptors, IconSize, Letter, LetterStyle, Location,
    Position, Request, TextBuffer, TextBundle, TextContent, TextContentView, TextGridGuide,
    TextGridLocation, TextLineStructure, TextScaleLetterDimensions, UIView,
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
        let content = TextContent::new(inner_req.hint_text);
        let view = TextContentView::new(
            0,
            inner_req.grid_guide.horizontal_character_max * inner_req.grid_guide.line_max,
            inner_req.text_color,
        );
        let text = cmd
            .spawn(TextBundle::new(
                content,
                view,
                inner_req.location,
                inner_req.alignment,
                inner_req.grid_guide,
            ))
            .id();
        let cursor_icon = cmd
            .spawn(IconBundle::new(
                Icon::new(inner_req.background_color),
                IconSize::Custom((character_dimensions.width, character_dimensions.height)),
                IconDescriptors::Cursor.key(),
                Location::from((
                    inner_req.location.position,
                    inner_req.location.depth.adjusted(1u32),
                )),
                inner_req.text_color,
            ))
            .insert(ColorInvert::on())
            .id();
        let background_icon = cmd
            .spawn(IconBundle::new(
                Icon::new(inner_req.text_color),
                IconSize::Custom((character_dimensions.width, character_dimensions.height)),
                IconDescriptors::Panel.key(),
                Location::from((
                    inner_req.location.position,
                    inner_req.location.depth.adjusted(2u32),
                )),
                inner_req.background_color,
            ))
            .id();
        cmd.entity(entity).insert(TextInput::new(
            TextInputText::new(text),
            CursorIcon::new(cursor_icon),
            TextBackgroundIcon(background_icon),
            inner_req.alignment,
            inner_req.grid_guide,
            inner_req.location,
            inner_req.text_color,
            inner_req.background_color,
        ));
        cmd.entity(entity).remove::<Request<TextInputRequest>>();
    }
}

pub(crate) fn position_ties(
    moved: Query<
        (
            &Position<UIView>,
            &TextInputText,
            &TextBackgroundIcon,
            &CursorIcon,
            &TextScaleLetterDimensions,
            &Cursor,
        ),
        Changed<Position<UIView>>,
    >,
    mut cmd: Commands,
    scale_factor: Res<ScaleFactor>,
) {
    for (pos, text_input_text, text_background_icon, cursor_icon, letter_dimensions, cursor) in
        moved.iter()
    {
        cmd.entity(text_input_text.entity).insert(*pos);
        cmd.entity(text_background_icon.0).insert(*pos);
        cmd.entity(cursor_icon.entity).insert(cursor_coords(
            pos,
            cursor,
            letter_dimensions.dimensions.to_ui(scale_factor.factor),
        ));
    }
}

pub(crate) fn read_area_from_text_bound(
    text_inputs: Query<
        (
            Entity,
            &TextBound,
            &TextInputText,
            &TextGridGuide,
            &TextColor,
            &TextBackgroundIcon,
        ),
        Or<(Changed<TextBound>, Changed<TextGridGuide>)>,
    >,
    text: Query<&TextScaleLetterDimensions>,
    mut cmd: Commands,
) {
    for (entity, bound, text_input_text, grid_guide, text_color, background_icon) in
        text_inputs.iter()
    {
        let letter_dimensions = text.get(text_input_text.entity).unwrap();
        cmd.entity(entity).insert((bound.area, *letter_dimensions));
        let view = TextContentView::new(
            0,
            grid_guide.horizontal_character_max * grid_guide.line_max,
            text_color.0,
        );
        cmd.entity(text_input_text.entity).insert(view);
        cmd.entity(background_icon.0)
            .insert(IconSize::Custom((bound.area.width, bound.area.height)));
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
    mut text_query: Query<&mut TextBuffer>,
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
                if let Ok(mut text_buffer) = text_query.get_mut(text_input_text.entity) {
                    if let Some(letter) = text_buffer.letters.get_mut(&cached) {
                        letter.metadata.color = text_color.0;
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
        &TextGridGuide,
        &TextColor,
    )>,
    focused_entity: Res<FocusedEntity>,
    mut text_query: Query<(Entity, &mut TextBuffer)>,
) {
    if let Some(entity) = focused_entity.entity {
        if let Ok((focus, mut cursor, max_characters, text_input_text, grid_guide, text_color)) =
            focused.get_mut(entity)
        {
            if focus.focused() {
                let (_entity, mut text) = text_query.get_mut(text_input_text.entity).unwrap();
                let num_letters = text.num_letters();
                if num_letters < max_characters.0 {
                    let character = char::from(num_letters as u8 + 32);
                    text.letters.insert(
                        cursor.location,
                        Letter::new(character, text_color.0, LetterStyle::REGULAR),
                    );
                    if cursor.location.x + 1 >= grid_guide.horizontal_character_max {
                        if cursor.location.y >= grid_guide.line_max - 1 {
                        } else {
                            cursor.location.x = 0;
                            cursor.location.y += 1;
                        }
                    } else {
                        cursor.location.x += 1;
                    }
                    let current_location = cursor.location;
                    if let Some(cached) = cursor.cached_location {
                        if let Some(letter) = text.letters.get_mut(&cached) {
                            letter.metadata.color = text_color.0;
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
        &Position<UIView>,
        &ClickState,
        &mut Cursor,
        &TextInputText,
        &TextScaleLetterDimensions,
        &TextGridGuide,
        &TextColor,
        &TextBackgroundColor,
    )>,
    mut text_entities: Query<(&mut TextBuffer, &TextLineStructure)>,
    scale_factor: Res<ScaleFactor>,
) {
    for (
        pos,
        click_state,
        mut cursor,
        text_input_text,
        character_dimensions,
        grid_guide,
        text_color,
        text_background_color,
    ) in clicked.iter_mut()
    {
        if click_state.clicked() {
            let (mut text, line_structure) = text_entities.get_mut(text_input_text.entity).unwrap();
            let click_location = click_state.click_location.unwrap();
            let ui_letter_dimensions = character_dimensions.dimensions.to_ui(scale_factor.factor);
            let mut line_clicked =
                ((click_location.y - pos.y) / ui_letter_dimensions.height).floor() as usize;
            let potential_letter_count = line_structure
                .letter_count
                .get(line_clicked)
                .cloned()
                .unwrap_or_default();
            if line_clicked >= line_structure.letter_count.len() || potential_letter_count == 0 {
                if line_clicked != 0 {
                    let mut next_line_up = line_clicked - 1;
                    let mut next_line_count = 0;
                    while next_line_up != 0
                        && next_line_up >= line_structure.letter_count.len() - 1
                        && next_line_count == 0
                    {
                        next_line_count = line_structure
                            .letter_count
                            .get(next_line_up)
                            .cloned()
                            .unwrap_or_default();
                        if next_line_count == 0 {
                            next_line_up -= 1;
                        }
                    }
                    line_clicked = next_line_up
                }
            }
            let click_x = click_location.x - pos.x;
            let x_letter_location = (click_x / ui_letter_dimensions.width).floor() as u32;
            let current_line_letter_count = line_structure
                .letter_count
                .get(line_clicked)
                .cloned()
                .unwrap_or_default();
            let mut was_over = false;
            if x_letter_location > current_line_letter_count {
                was_over = true;
            }
            let x_letter_location = x_letter_location.min(current_line_letter_count);
            let x_letter_location = x_letter_location
                + 1 * (x_letter_location < (grid_guide.horizontal_character_max - 1)
                    && was_over
                    && current_line_letter_count != 0) as u32;
            let location = TextGridLocation::new(x_letter_location, line_clicked as u32);
            if let Some(cached_location) = cursor.cached_location {
                if location != cached_location {
                    if let Some(letter) = text.letters.get_mut(&cached_location) {
                        letter.metadata.color = text_color.0;
                    }
                }
            }
            cursor.location = location;
            cursor.cached_location.replace(location);
            if let Some(letter) = text.letters.get_mut(&location) {
                letter.metadata.color = text_background_color.0;
            }
        }
    }
}

pub(crate) fn cursor_letter_color_filter(
    polled: Query<(&Cursor, &TextInputText, &Focus, &TextBackgroundColor)>,
    mut changed_text_buffers: Query<&mut TextBuffer, Changed<TextBuffer>>,
) {
    for (cursor, text_input_text, focus, text_background_color) in polled.iter() {
        if focus.focused() {
            if let Ok(mut text) = changed_text_buffers.get_mut(text_input_text.entity) {
                if let Some(letter) = text.letters.get_mut(&cursor.location) {
                    letter.metadata.color = text_background_color.0;
                }
            }
        }
    }
}

pub(crate) fn update_cursor_pos(
    updated: Query<
        (
            Entity,
            &Position<UIView>,
            &Cursor,
            &TextScaleLetterDimensions,
            &CursorIcon,
        ),
        Changed<Cursor>,
    >,
    mut cmd: Commands,
    scale_factor: Res<ScaleFactor>,
) {
    for (_entity, pos, cursor, letter_dimensions, cursor_icon) in updated.iter() {
        let ui_letter_dimensions = letter_dimensions.dimensions.to_ui(scale_factor.factor);
        cmd.entity(cursor_icon.entity)
            .insert(cursor_coords(pos, cursor, ui_letter_dimensions));
    }
}

fn cursor_coords(
    pos: &Position<UIView>,
    cursor: &Cursor,
    ui_letter_dimensions: Area<UIView>,
) -> Position<UIView> {
    Position::<UIView>::new(
        pos.x + cursor.location.x as f32 * ui_letter_dimensions.width,
        pos.y + cursor.location.y as f32 * ui_letter_dimensions.height,
    )
}
