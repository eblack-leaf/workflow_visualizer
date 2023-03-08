use crate::{Area, DeviceView};

#[derive(Clone, Copy)]
pub struct WindowResize {
    pub size: Area<DeviceView>,
    pub scale_factor: f64,
}

impl WindowResize {
    pub(crate) fn new(size: Area<DeviceView>, scale_factor: f64) -> Self {
        Self { size, scale_factor }
    }
}
