use bevy_ecs::prelude::{Bundle, Commands, Component, Entity, Query, Res, ResMut, SystemStage};
use bevy_ecs::query::Changed;

use crate::{
    Attach, Clickable, ClickListener, Color, Engen, FrontEndStages, Location, Signal, Text,
    TextBoundGuide, TextBundle, TextPartition, TextScaleAlignment, UIView, VirtualKeyboardAdapter,
    VirtualKeyboardType,
};
use crate::engen::Container;
use crate::focus::{Focus, FocusedEntity};
use crate::text::{TextBound, TextStages};

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
    mut virtual_keyboard: ResMut<VirtualKeyboardAdapter>,
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
        let input_listener = focused.get(entity).unwrap();
        // limit text input by max characters here
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
        }
    }
    pub fn spawn_with<C: Into<Color>, L: Into<Location<UIView>>>(
        container: &mut Container,
        text_color: C,
        location: L,
        alignment: TextScaleAlignment,
        text_bound_guide: TextBoundGuide,
    ) -> Entity {
        let text_color = text_color.into();
        let location = location.into();
        // make text grid to place cursor at locations easier for editing
        let text = container
            .spawn(TextBundle::new(
                Text::new(vec![TextPartition::from(("", (text_color, 0)))]),
                location,
                alignment,
            ))
            .insert(text_bound_guide)
            .id();
        container
            .spawn(TextInput::new(
                TextInputText::new(text),
                alignment,
                text_bound_guide,
                location,
            ))
            .id()
    }
}

pub struct TextInputPlugin;

impl Attach for TextInputPlugin {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.add_stage_after(
            TextStages::PlacementPreparation,
            "read_bound",
            SystemStage::single(read_area_from_text_bound),
        );
        engen.frontend.main.add_stage_after(
            FrontEndStages::PreProcessResolve,
            "open_keyboard",
            SystemStage::single(open_virtual_keyboard),
        );
    }
}
