use crate::text_step_out::scale::Scale;
use fontdue::{Font as fdFont, FontSettings};
use std::path::Path;

#[cfg(test)]
#[test]
pub fn font() {
    let font = Font::new(
        "/home/omi-voshuli/note-ifications/app/fonts/JetBrainsMono-Medium.ttf",
        13u32,
    );
}

pub struct Font {
    pub font_storage: [fdFont; 1],
}

impl Font {
    pub fn new<V: AsRef<Path>, T: Into<Scale>>(path: V, opt_scale: T) -> Self {
        Self {
            font_storage: [fdFont::from_bytes(
                std::fs::read(path).expect("invalid font path read"),
                FontSettings {
                    scale: opt_scale.into().px(),
                    ..FontSettings::default()
                },
            )
            .expect("text font creation")],
        }
    }
    pub fn font_slice(&self) -> &[fdFont] {
        self.font_storage.as_slice()
    }
    pub fn font(&self) -> &fdFont {
        &self.font_storage[0]
    }
    pub fn index() -> usize {
        0
    }
}
