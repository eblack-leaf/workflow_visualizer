use crate::area::Area;
use crate::coord::DeviceContext;

#[derive(Clone, Copy)]
pub struct WindowResize {
    pub size: Area<DeviceContext>,
    pub scale_factor: f64,
}

impl WindowResize {
    pub(crate) fn new(size: Area<DeviceContext>, scale_factor: f64) -> Self {
        Self { size, scale_factor }
    }
}