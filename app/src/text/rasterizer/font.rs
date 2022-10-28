use std::path::Path;

use fontdue::{Font, FontSettings};

use crate::text::scale::TextScale;

#[cfg(test)]
#[test]
pub fn font() {
    let font = TextFont::new(
        "/home/omi-voshuli/note-ifications/app/fonts/JetBrainsMono-Medium.ttf",
        13u32,
    );
}

pub struct TextFont {
    pub font: Font,
}

impl TextFont {
    pub fn new<V: AsRef<Path>, T: Into<TextScale>>(path: V, opt_scale: T) -> Self {
        Self {
            font: Font::from_bytes(
                std::fs::read(path).expect("invalid font path read"),
                FontSettings {
                    scale: opt_scale.into().px(),
                    ..FontSettings::default()
                },
            )
                .expect("text font creation"),
        }
    }
}
