use crate::color::Color;

pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub secondary: Color,
}
impl Theme {
    pub fn new<C: Into<Color>>(background: C, foreground: C, secondary: C) -> Self {
        Self {
            background: background.into(),
            foreground: foreground.into(),
            secondary: secondary.into(),
        }
    }
}
