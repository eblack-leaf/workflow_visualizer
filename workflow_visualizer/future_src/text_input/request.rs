use crate::text_input::components::MaxCharacters;
use crate::{Color, Layer, TextScaleAlignment, ViewArea, ViewPosition};

pub struct TextInputRequest {
    pub hint_text: String,
    pub alignment: TextScaleAlignment,
    pub view_position: ViewPosition,
    pub view_area: ViewArea,
    pub layer: Layer,
    pub text_color: Color,
    pub background_color: Color,
    pub border_color: Color,
    pub max_characters: MaxCharacters,
}

impl TextInputRequest {
    pub fn new<C: Into<Color>>(
        view_position: ViewPosition,
        view_area: ViewArea,
        layer: Layer,
        hint_text: String,
        alignment: TextScaleAlignment,
        text_color: C,
        background_color: C,
        border_color: C,
        max_characters: MaxCharacters,
    ) -> Self {
        Self {
            hint_text,
            alignment,
            view_position,
            view_area,
            layer,
            text_color: text_color.into(),
            background_color: background_color.into(),
            border_color: border_color.into(),
            max_characters,
        }
    }
}
