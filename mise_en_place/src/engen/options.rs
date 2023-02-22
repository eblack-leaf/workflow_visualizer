use crate::{Area, DeviceView, Theme};

pub struct EngenOptions {
    pub native_dimensions: Option<Area<DeviceView>>,
    pub theme: Theme,
}

impl EngenOptions {
    pub fn new() -> Self {
        Self {
            native_dimensions: None,
            theme: Theme::default(),
        }
    }
    pub fn with_native_dimensions<A: Into<Area<DeviceView>>>(mut self, dimensions: A) -> Self {
        self.native_dimensions.replace(dimensions.into());
        self
    }
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
