use crate::{
    Color, InterfaceContext, Layer, Location, TextScaleAlignment, ViewArea,
    ViewPosition,
};

pub struct TextInputRequest {
    pub hint_text: String,
    pub alignment: TextScaleAlignment,
    pub view_position: ViewPosition,
    pub view_area: ViewArea,
    pub layer: Layer,
    pub text_color: Color,
    pub background_color: Color,
}

impl TextInputRequest {
    pub fn new<C: Into<Color>>(
        view_position: ViewPosition,
        view_area: ViewArea,
        layer: Layer,
        hint_text: String,
        alignment: TextScaleAlignment,
        color: C,
        background_color: C,
    ) -> Self {
        Self {
            hint_text,
            alignment,
            view_position,
            view_area,
            layer,
            text_color: color.into(),
            background_color: background_color.into(),
        }
    }
}
