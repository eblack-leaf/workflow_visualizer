use bevy_ecs::prelude::{Bundle, Commands, Component, Entity, IntoSystemDescriptor, Query, Res};
use bevy_ecs::query::Changed;

use crate::{
    Attach, Clickable, ClickListener, Color, Engen, FrontEndStages, Location, Request, Text,
    TextBoundGuide, TextBundle, TextPartition, TextScaleAlignment, UIView, VirtualKeyboardAdapter,
    VirtualKeyboardType,
};
use crate::focus::{Focus, FocusedEntity, FocusSystems};
use crate::text::TextBound;

pub struct TextInputRequest {
    pub hint_text: String,
    pub alignment: TextScaleAlignment,
    pub bound_guide: TextBoundGuide,
    pub location: Location<UIView>,
    pub text_color: Color,
}

impl TextInputRequest {
    pub fn new<L: Into<Location<UIView>>, C: Into<Color>>(
        hint_text: String,
        alignment: TextScaleAlignment,
        bound_guide: TextBoundGuide,
        location: L,
        color: C,
    ) -> Self {
        Self {
            hint_text,
            alignment,
            bound_guide,
            location: location.into(),
            text_color: color.into(),
        }
    }
}

pub(crate) fn spawn(
    mut requests: Query<(Entity, &mut Request<TextInputRequest>)>,
    mut cmd: Commands,
) {
    for (entity, mut request) in requests.iter_mut() {
        let inner_req = request.req.take().unwrap();
        let text = cmd
            .spawn(TextBundle::new(
                Text::new(vec![TextPartition::from((
                    inner_req.hint_text,
                    (inner_req.text_color, 0),
                ))]),
                inner_req.location,
                inner_req.alignment,
            ))
            .insert(inner_req.bound_guide)
            .id();
        cmd.entity(entity).insert(TextInput::new(
            TextInputText::new(text),
            inner_req.alignment,
            inner_req.bound_guide,
            inner_req.location,
        ));
        cmd.entity(entity).remove::<Request<TextInputRequest>>();
    }
}

#[derive(Component)]
pub struct Cursor {
    pub location: TextGridLocation,
}

impl Cursor {
    pub(crate) fn new() -> Self {
        Self {
            location: TextGridLocation::new(0, 0),
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
    pub(crate) alignment: TextScaleAlignment,
    pub(crate) bound_guide: TextBoundGuide,
    #[bundle]
    pub(crate) location: Location<UIView>,
    #[bundle]
    pub(crate) clickable: Clickable,
    pub(crate) max_characters: MaxCharacters,
    pub(crate) focus: Focus,
    pub(crate) keyboard_type: VirtualKeyboardType,
    pub(crate) cursor: Cursor,
}

impl TextInput {
    pub(crate) fn new(
        text_input_text: TextInputText,
        alignment: TextScaleAlignment,
        bound_guide: TextBoundGuide,
        location: Location<UIView>,
    ) -> Self {
        Self {
            text_input_text,
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
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Resolve,
            read_area_from_text_bound,
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
