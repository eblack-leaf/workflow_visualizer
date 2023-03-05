use bevy_ecs::prelude::{Bundle, Commands, Component, Entity, IntoSystemDescriptor, Query, Res};
use bevy_ecs::query::Changed;

use crate::clickable::ClickSystems;
use crate::focus::{Focus, FocusSystems, FocusedEntity};
use crate::text::{AlignedFonts, TextBound, TextScale};
use crate::{
    Attach, ClickListener, ClickState, Clickable, Color, ColorInvert, Engen, FrontEndStages, Icon,
    IconBundle, IconDescriptors, IconMeshAddRequest, IconSize, LetterStyle, Location, Position,
    Request, ScaleFactor, Text, TextBundle, TextGridGuide, TextLine, TextScaleAlignment,
    TextScaleLetterDimensions, Theme, UIView, VirtualKeyboardAdapter, VirtualKeyboardType,
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
        let text = cmd
            .spawn(TextBundle::new(
                Text::new(vec![TextLine::from((
                    inner_req.hint_text,
                    inner_req.text_color,
                    LetterStyle::REGULAR,
                ))]),
                inner_req.location,
                inner_req.alignment,
            ))
            .insert(inner_req.grid_guide)
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

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
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
    pub cached_location: TextGridLocation,
    pub cached_letter_color: Option<Color>,
}

impl Cursor {
    pub(crate) fn new() -> Self {
        Self {
            location: TextGridLocation::new(0, 0),
            cached_location: TextGridLocation::new(0, 0),
            cached_letter_color: None,
        }
    }
}

pub(crate) fn read_area_from_text_bound(
    text_inputs: Query<(Entity, &TextBound), Changed<TextBound>>,
    mut cmd: Commands,
) {
    for (entity, bound) in text_inputs.iter() {
        cmd.entity(entity).insert(bound.area.clone());
    }
}

pub(crate) fn open_virtual_keyboard(
    virtual_keyboard: Res<VirtualKeyboardAdapter>,
    focus_changed: Query<(&Focus, &VirtualKeyboardType), Changed<Focus>>,
) {
    let mut should_close = true;
    let mut keyboard = VirtualKeyboardType::Keyboard;
    for (focus, v_key_type) in focus_changed.iter() {
        if focus.focused() {
            should_close = false;
            keyboard = *v_key_type;
        }
    }
    if should_close {
        virtual_keyboard.close();
    } else {
        virtual_keyboard.open(keyboard);
    }
}

pub(crate) fn read_input_if_focused(focused: Query<&Focus>, focused_entity: Res<FocusedEntity>) {
    if let Some(entity) = focused_entity.entity {
        if let Ok(focus) = focused.get(entity) {
            // limit text input by max characters here
        }
    }
}

pub(crate) fn set_cursor_location(
    mut clicked: Query<(
        &ClickState,
        &CursorIcon,
        &mut Cursor,
        &mut Text,
        &TextScaleLetterDimensions,
    )>,
    theme: Res<Theme>,
    mut cmd: Commands,
) {
    for (click_state, cursor_icon, mut cursor, mut text, character_dimensions) in clicked.iter_mut()
    {
        if click_state.clicked() {
            let line_clicked = (click_state.click_location.unwrap().y
                / character_dimensions.dimensions.height)
                .floor() as usize;
            let line = text.lines.get_mut(line_clicked).unwrap();
            let click_x = click_state.click_location.unwrap().x;
            let x_letter_location =
                (click_x / character_dimensions.dimensions.width).floor() as u32;
            let x_letter_location = x_letter_location.max(line.letters.len() as u32);
            let location = TextGridLocation::new(x_letter_location, line_clicked as u32);
            if location != cursor.cached_location {
                cmd.entity(cursor_icon.entity)
                    .insert(Position::<UIView>::new(
                        x_letter_location as f32 * character_dimensions.dimensions.width,
                        line_clicked as f32 * character_dimensions.dimensions.height,
                    ));
                let cached_x = cursor.cached_location.x as usize;
                line.letters.get_mut(cached_x).unwrap().metadata.color = cursor
                    .cached_letter_color
                    .unwrap_or(Color::OFF_WHITE.into());
                cursor.location = location;
                cursor.cached_location = location;
                let current_color = line
                    .letters
                    .get(x_letter_location as usize)
                    .unwrap()
                    .metadata
                    .color;
                cursor.cached_letter_color.replace(current_color);
                line.letters
                    .get_mut(x_letter_location as usize)
                    .unwrap()
                    .metadata
                    .color = theme.background;
            }
        }
    }
}

#[derive(Component)]
pub(crate) struct TextInputText {
    pub(crate) entity: Entity,
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
        }
    }
}

pub struct TextInputPlugin;

impl Attach for TextInputPlugin {
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
            set_cursor_location.after(ClickSystems::RegisterClick),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Prepare,
            open_virtual_keyboard.after(FocusSystems::SetFocused),
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Spawn, spawn);
    }
}
