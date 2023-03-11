use crate::{Color, InterfaceContext, Location, TextGridGuide, TextScaleAlignment};

pub struct TextInputRequest {
    pub hint_text: String,
    pub alignment: TextScaleAlignment,
    pub grid_guide: TextGridGuide,
    pub location: Location<InterfaceContext>,
    pub text_color: Color,
    pub background_color: Color,
}

impl TextInputRequest {
    pub fn new<L: Into<Location<InterfaceContext>>, C: Into<Color>>(
        hint_text: String,
        alignment: TextScaleAlignment,
        grid_guide: TextGridGuide,
        location: L,
        color: C,
        background_color: C,
    ) -> Self {
        Self {
            hint_text,
            alignment,
            grid_guide,
            location: location.into(),
            text_color: color.into(),
            background_color: background_color.into(),
        }
    }
}
