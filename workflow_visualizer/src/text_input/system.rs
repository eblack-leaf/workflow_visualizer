use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Commands, Or, Query, Res, Without};

use crate::focus::{Focus, FocusedEntity};
use crate::panel::{ContentArea, Padding, Panel};
use crate::text::{AlignedFonts, TextBound, TextScale};
use crate::text_input::components::{MaxCharacters, TextContentPanel, TextInput, TextInputText};
use crate::text_input::cursor::{Cursor, CursorIcon};
use crate::text_input::request::TextInputRequest;
use crate::text_input::{TextBackgroundColor, TextColor};
use crate::touch::{TouchLocation, Touched};
use crate::{
    Area, Color, ColorInvert, Icon, IconDescriptors, IconSecondaryColor, IconSize,
    InterfaceContext, Layer, Letter, LetterStyle, Location, Position, Request, ScaleFactor, Text,
    TextBuffer, TextContent, TextContentView, TextGridDescriptor, TextGridLocation,
    TextLineStructure, TextScaleLetterDimensions, VirtualKeyboardAdapter, VirtualKeyboardType,
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
        let padding = inner_req.padding;
        let padding_pos = Position::new(padding.0.width, padding.0.height);
        let padding_area = padding.0;
        let text = cmd
            .spawn(Text::new(
                content,
                view,
                Location::from((
                    inner_req.location.position + padding_pos,
                    inner_req.location.layer,
                )),
                inner_req.alignment,
                inner_req.grid_guide,
            ))
            .id();
        let cursor_icon = cmd
            .spawn(Icon::new(
                IconDescriptors::Cursor.key(),
                Location::from((
                    inner_req.location.position + padding_pos,
                    inner_req.location.layer + Layer::from(1u32),
                )),
                IconSize::Custom((character_dimensions.width, character_dimensions.height)),
                inner_req.text_color,
                IconSecondaryColor::new(inner_req.background_color),
            ))
            .insert(ColorInvert::on())
            .id();
        let content_panel = cmd
            .spawn(Panel::new(
                Location::from((
                    inner_req.location.position,
                    inner_req.location.layer + Layer::from(2u32),
                )),
                inner_req.background_color,
                padding_area,
                1,
                Color::OFF_WHITE.into(),
            ))
            .id();
        cmd.entity(entity).insert(TextInput::new(
            TextInputText::new(text),
            CursorIcon::new(cursor_icon),
            TextContentPanel(content_panel),
            inner_req.alignment,
            inner_req.grid_guide,
            Location::from((
                inner_req.location.position + padding_pos,
                inner_req.location.layer,
            )),
            inner_req.text_color,
            inner_req.background_color,
        ));
        cmd.entity(entity).remove::<Request<TextInputRequest>>();
    }
}
pub(crate) fn read_padding_change(
    mut text_inputs: Query<(&TextContentPanel, &mut Position<InterfaceContext>), Without<Padding>>,
    changed_content_panels: Query<(&Padding, &Position<InterfaceContext>), Changed<Padding>>,
) {
    for (content_panel, mut pos) in text_inputs.iter_mut() {
        if let Ok((padding, content_panel_pos)) = changed_content_panels.get(content_panel.0) {
            *pos = *content_panel_pos + Position::new(padding.0.width, padding.0.height);
        }
    }
}
pub(crate) fn position_ties(
    moved: Query<
        (
            &Position<InterfaceContext>,
            &TextInputText,
            &TextContentPanel,
            &CursorIcon,
            &TextScaleLetterDimensions,
            &Cursor,
        ),
        Changed<Position<InterfaceContext>>,
    >,
    padding_read: Query<&Padding>,
    mut cmd: Commands,
    scale_factor: Res<ScaleFactor>,
) {
    for (pos, text_input_text, text_content_panel, cursor_icon, letter_dimensions, cursor) in
        moved.iter()
    {
        let padding = padding_read.get(text_content_panel.0).unwrap();
        cmd.entity(text_input_text.entity).insert(*pos);
        cmd.entity(text_content_panel.0)
            .insert(*pos - Position::new(padding.0.width, padding.0.height));
        cmd.entity(cursor_icon.entity).insert(cursor_coords(
            *pos,
            cursor,
            letter_dimensions.dimensions.to_ui(scale_factor.factor),
        ));
    }
}

pub(crate) fn reconfigure_text_input(
    text_inputs: Query<
        (
            &TextInputText,
            &TextGridDescriptor,
            &TextColor,
            &TextContentPanel,
            &Area<InterfaceContext>,
        ),
        Or<(Changed<TextBound>, Changed<TextGridDescriptor>)>,
    >,
    mut text: Query<&mut TextContentView>,
    mut content_panels: Query<&mut ContentArea>,
) {
    for (text_input_text, grid_guide, text_color, content_panel, area) in text_inputs.iter() {
        let mut text_content_view = text.get_mut(text_input_text.entity).unwrap();
        let view = TextContentView::new(
            0,
            grid_guide.horizontal_character_max * grid_guide.line_max,
            text_color.0,
        );
        *text_content_view = view;
        let mut content_panel_area = content_panels.get_mut(content_panel.0).unwrap();
        *content_panel_area = ContentArea(*area);
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
        &TextGridDescriptor,
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
                if num_letters < max_characters.0 && num_letters + 32 < 126 {
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
        &Position<InterfaceContext>,
        &Touched,
        &TouchLocation,
        &mut Cursor,
        &TextInputText,
        &TextScaleLetterDimensions,
        &TextGridDescriptor,
        &TextColor,
        &TextBackgroundColor,
    )>,
    mut text_entities: Query<(&mut TextBuffer, &TextLineStructure)>,
    scale_factor: Res<ScaleFactor>,
) {
    for (
        pos,
        touched,
        touch_location,
        mut cursor,
        text_input_text,
        character_dimensions,
        grid_guide,
        text_color,
        text_background_color,
    ) in clicked.iter_mut()
    {
        if touched.touched() {
            let click_location = touch_location.0.unwrap();
            let click_offset_x = click_location.x - pos.x;
            let click_offset_y = click_location.y - pos.y;
            let (mut text, line_structure) = text_entities.get_mut(text_input_text.entity).unwrap();
            let ui_letter_dimensions = character_dimensions.dimensions.to_ui(scale_factor.factor);
            let mut line_clicked = (click_offset_y / ui_letter_dimensions.height).floor() as usize;
            let potential_letter_count = line_structure
                .letter_count
                .get(line_clicked)
                .cloned()
                .unwrap_or_default();
            if (line_clicked >= line_structure.letter_count.len() || potential_letter_count == 0)
                && line_clicked != 0
            {
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
            let x_letter_location = (click_offset_x / ui_letter_dimensions.width).floor() as u32;
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
                + (x_letter_location < (grid_guide.horizontal_character_max - 1)
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
            &Position<InterfaceContext>,
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
            .insert(cursor_coords(*pos, cursor, ui_letter_dimensions));
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
