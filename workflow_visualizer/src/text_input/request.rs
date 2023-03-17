use crate::panel::Padding;
use crate::{Area, Color, InterfaceContext, Location, TextGridDescriptor, TextScaleAlignment};

pub struct TextInputRequest {
    pub hint_text: String,
    pub alignment: TextScaleAlignment,
    pub grid_guide: TextGridDescriptor,
    pub location: Location<InterfaceContext>,
    pub text_color: Color,
    pub background_color: Color,
    pub padding: Padding,
}

impl TextInputRequest {
    pub fn new<
        L: Into<Location<InterfaceContext>>,
        C: Into<Color>,
        A: Into<Area<InterfaceContext>>,
    >(
        hint_text: String,
        alignment: TextScaleAlignment,
        grid_guide: TextGridDescriptor,
        location: L,
        color: C,
        background_color: C,
        padding: A,
    ) -> Self {
        Self {
            hint_text,
            alignment,
            grid_guide,
            location: location.into(),
            text_color: color.into(),
            background_color: background_color.into(),
            padding: Padding(padding.into()),
        }
    }
}
