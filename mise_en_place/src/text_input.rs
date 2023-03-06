use bevy_ecs::prelude::{
    Bundle, Commands, Component, Entity, IntoSystemDescriptor, Or, Query, Res, SystemLabel,
};
use bevy_ecs::query::Changed;

use crate::clickable::ClickSystems;
use crate::focus::{Focus, FocusSystems, FocusedEntity};
use crate::text::{AlignedFonts, TextBound, TextContent, TextContentView, TextScale};
use crate::{
    Attach, ClickListener, ClickState, Clickable, Color, ColorInvert, Engen, FrontEndStages, Icon,
    IconBundle, IconDescriptors, IconMeshAddRequest, IconSize, Letter, LetterStyle, Location,
    Position, Request, ScaleFactor, TextBuffer, TextBundle, TextGridGuide, TextLineStructure,
    TextScaleAlignment, TextScaleLetterDimensions, Theme, UIView, VirtualKeyboardAdapter,
    VirtualKeyboardType, Visibility,
};

pub struct TextInputRequest {
    pub hint_text: String,
    pub alignment: TextScaleAlignment,
    pub grid_guide: TextGridGuide,
    pub location: Location<UIView>,
    pub text_color: Color,
}

impl TextInputRequest {
    pub fn new<L: Into<Location<UIView>>, C: Into<Color>>(
        hint_text: String,
        alignment: TextScaleAlignment,
        grid_guide: TextGridGuide,
        location: L,
        color: C,
    ) -> Self {
        Self {
            hint_text,
            alignment,
            grid_guide,
            location: location.into(),
            text_color: color.into(),
        }
    }
}

pub(crate) fn spawn(
    mut requests: Query<(Entity, &mut Request<TextInputRequest>)>,
    mut cmd: Commands,
    fonts: Res<AlignedFonts>,
    scale_factor: Res<ScaleFactor>,
    theme: Res<Theme>,
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
            Color::OFF_WHITE,
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
                Icon::new(theme.background),
                IconSize::Custom((
                    character_dimensions.width as u32,
                    character_dimensions.height as u32,
                )),
                IconDescriptors::Cursor.key(),
                Location::from((
                    inner_req.location.position,
                    inner_req.location.depth.adjusted(1u32),
                )),
                inner_req.text_color,
            ))
            .insert(ColorInvert::on())
            .id();
        cmd.entity(entity).insert(TextInput::new(
            TextInputText::new(text),
            CursorIcon::new(cursor_icon),
            inner_req.alignment,
            inner_req.grid_guide,
            inner_req.location,
        ));
        cmd.entity(entity).remove::<Request<TextInputRequest>>();
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub struct TextGridLocation {
    pub x: u32,
    pub y: u32,
}

impl TextGridLocation {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
pub struct Cursor {
    pub location: TextGridLocation,
    pub cached_location: Option<TextGridLocation>,
}

impl Cursor {
    pub(crate) fn new() -> Self {
        Self {
            location: TextGridLocation::new(0, 0),
            cached_location: None,
        }
    }
}

pub(crate) fn read_area_from_text_bound(
    text_inputs: Query<
        (Entity, &TextBound, &TextInputText, &TextGridGuide),
        Or<(Changed<TextBound>, Changed<TextGridGuide>)>,
    >,
    text: Query<&TextScaleLetterDimensions>,
    mut cmd: Commands,
) {
    for (entity, bound, text_input_text, grid_guide) in text_inputs.iter() {
        let (letter_dimensions) = text.get(text_input_text.entity).unwrap();
        cmd.entity(entity).insert((bound.area, *letter_dimensions));
        let view = TextContentView::new(
            0,
            grid_guide.horizontal_character_max * grid_guide.line_max,
            Color::OFF_WHITE,
        );
        cmd.entity(text_input_text.entity).insert(view);
    }
}

pub(crate) fn open_virtual_keyboard(
    virtual_keyboard: Res<VirtualKeyboardAdapter>,
    focus_changed: Query<(&Focus, &VirtualKeyboardType, &CursorIcon), Changed<Focus>>,
    mut cmd: Commands,
) {
    let mut should_close = true;
    let mut keyboard = VirtualKeyboardType::Keyboard;
    for (focus, v_key_type, cursor_icon) in focus_changed.iter() {
        if focus.focused() {
            should_close = false;
            keyboard = *v_key_type;
            cmd.entity(cursor_icon.entity).insert(ColorInvert::off());
        } else {
            cmd.entity(cursor_icon.entity).insert(ColorInvert::on());
        }
    }
    if should_close {
        virtual_keyboard.close();
    } else {
        virtual_keyboard.open(keyboard);
    }
}

pub(crate) fn read_input_if_focused(
    mut focused: Query<(
        &Focus,
        &mut Cursor,
        &MaxCharacters,
        &TextInputText,
        &TextGridGuide,
    )>,
    focused_entity: Res<FocusedEntity>,
    mut text_query: Query<(Entity, &mut TextBuffer)>,
) {
    if let Some(entity) = focused_entity.entity {
        if let Ok((focus, mut cursor, max_characters, text_input_text, grid_guide)) =
            focused.get_mut(entity)
        {
            if focus.focused() {
                let (entity, mut text) = text_query.get_mut(text_input_text.entity).unwrap();
                if text.num_letters() < max_characters.0 {
                    text.letters.insert(
                        cursor.location,
                        Letter::new('a', Color::OFF_WHITE, LetterStyle::REGULAR),
                    );
                    cursor.location.x += 1;
                    if cursor.location.x > grid_guide.horizontal_character_max {
                        if cursor.location.y >= grid_guide.line_max {
                            cursor.location.x -= 1;
                        } else {
                            cursor.location.x = 0;
                            cursor.location.y += 1;
                        }
                    }
                    let current_location = cursor.location;
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
    )>,
    mut text_entities: Query<(&mut TextBuffer, &TextLineStructure)>,
    theme: Res<Theme>,
) {
    for (pos, click_state, mut cursor, text_input_text, character_dimensions, grid_guide) in
        clicked.iter_mut()
    {
        if click_state.clicked() {
            let (mut text, line_structure) = text_entities.get_mut(text_input_text.entity).unwrap();
            let click_location = click_state.click_location.unwrap();
            let mut line_clicked = ((click_location.y - pos.y)
                / character_dimensions.dimensions.height)
                .floor() as usize;
            let potential_letter_count = line_structure
                .letter_count
                .get(line_clicked)
                .cloned()
                .unwrap_or_default();
            if line_clicked > line_structure.letter_count.len() || potential_letter_count == 0 {
                if line_clicked != 0 {
                    let mut next_line_up = line_clicked - 1;
                    let mut next_line_count = 0;
                    while next_line_up != 0
                        && next_line_up >= line_structure.letter_count.len() - 1
                        && next_line_count == 0
                    {
                        next_line_count = *line_structure.letter_count.get(next_line_up).unwrap();
                        if next_line_count == 0 {
                            next_line_up -= 1;
                        }
                    }
                    line_clicked = next_line_up
                }
            }
            let click_x = click_location.x - pos.x;
            let x_letter_location =
                (click_x / character_dimensions.dimensions.width).floor() as u32;
            let x_letter_location =
                x_letter_location.min(*line_structure.letter_count.get(line_clicked).unwrap());
            let x_letter_location = x_letter_location
                + 1 * (x_letter_location < grid_guide.horizontal_character_max) as u32; // try to add one to get next available spot on that line
            let location = TextGridLocation::new(x_letter_location, line_clicked as u32);
            if let Some(cached_location) = cursor.cached_location {
                if location != cached_location {
                    if let Some(letter) = text.letters.get_mut(&cached_location) {
                        letter.metadata.color = Color::OFF_WHITE.into();
                    }
                }
            }
            cursor.location = location;
            cursor.cached_location.replace(location);
            if let Some(letter) = text.letters.get_mut(&location) {
                letter.metadata.color = theme.background;
            }
        }
    }
}

pub(crate) fn cursor_letter_color_filter(
    polled: Query<(&Cursor, &TextInputText)>,
    mut changed_text_buffers: Query<&mut TextBuffer, Changed<TextBuffer>>,
    theme: Res<Theme>,
) {
    for (cursor, text_input_text) in polled.iter() {
        if let Ok(mut text) = changed_text_buffers.get_mut(text_input_text.entity) {
            if let Some(letter) = text.letters.get_mut(&cursor.location) {
                letter.metadata.color = theme.background;
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
) {
    for (entity, pos, cursor, letter_dimensions, cursor_icon) in updated.iter() {
        cmd.entity(cursor_icon.entity)
            .insert(Position::<UIView>::new(
                pos.x + cursor.location.x as f32 * letter_dimensions.dimensions.width,
                pos.y + cursor.location.y as f32 * letter_dimensions.dimensions.height,
            ));
    }
}

#[derive(Component)]
pub struct TextInputText {
    pub entity: Entity,
}

impl TextInputText {
    pub(crate) fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

#[derive(Component)]
pub(crate) struct MaxCharacters(pub(crate) u32);

#[derive(Bundle)]
pub struct TextInput {
    pub(crate) text_input_text: TextInputText,
    pub(crate) cursor_icon: CursorIcon,
    pub(crate) alignment: TextScaleAlignment,
    pub(crate) bound_guide: TextGridGuide,
    #[bundle]
    pub(crate) location: Location<UIView>,
    #[bundle]
    pub(crate) clickable: Clickable,
    pub(crate) max_characters: MaxCharacters,
    pub(crate) focus: Focus,
    pub(crate) keyboard_type: VirtualKeyboardType,
    pub(crate) cursor: Cursor,
    pub(crate) visibility: Visibility,
}

#[derive(Component)]
pub(crate) struct CursorIcon {
    pub(crate) entity: Entity,
}

impl CursorIcon {
    pub(crate) fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

impl TextInput {
    pub(crate) fn new(
        text_input_text: TextInputText,
        cursor_icon: CursorIcon,
        alignment: TextScaleAlignment,
        bound_guide: TextGridGuide,
        location: Location<UIView>,
    ) -> Self {
        Self {
            text_input_text,
            cursor_icon,
            alignment,
            bound_guide,
            location,
            clickable: Clickable::new(ClickListener::on_press(), false),
            max_characters: MaxCharacters(
                bound_guide.horizontal_character_max * bound_guide.line_max,
            ),
            focus: Focus::new(),
            keyboard_type: VirtualKeyboardType::Keyboard,
            cursor: Cursor::new(),
            visibility: Visibility::new(),
        }
    }
}

pub struct TextInputAttachment;

#[derive(SystemLabel)]
pub enum TextInputSystems {
    CursorLocation,
    ReadInput,
}

impl Attach for TextInputAttachment {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Resolve, read_area_from_text_bound);
        engen
            .frontend
            .container
            .spawn(IconMeshAddRequest::new(IconDescriptors::Cursor, 5));
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Prepare,
            set_cursor_location
                .label(TextInputSystems::CursorLocation)
                .after(ClickSystems::RegisterClick),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Prepare,
            read_input_if_focused
                .label(TextInputSystems::ReadInput)
                .after(TextInputSystems::CursorLocation)
                .after(FocusSystems::SetFocused),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Prepare,
            open_virtual_keyboard.after(FocusSystems::SetFocused),
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::CoordPrepare, update_cursor_pos);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Spawn, spawn);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::ResolvePrepare, cursor_letter_color_filter);
    }
}
