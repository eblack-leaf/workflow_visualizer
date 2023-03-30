use crate::coord::Coordinate;
use crate::icon::mesh::ColorInvert;
use crate::icon::IconKey;
use crate::{Area, Color, DeviceContext, InterfaceContext, Layer, Position, Section};

pub(crate) struct IconAdd {
    pub(crate) key: IconKey,
    pub(crate) panel: Coordinate<DeviceContext>,
    pub(crate) color: Color,
    pub(crate) secondary_color: Color,
    pub(crate) color_invert: ColorInvert,
}

impl IconAdd {
    pub(crate) fn new(
        key: IconKey,
        position: Position<InterfaceContext>,
        area: Area<InterfaceContext>,
        layer: Layer,
        color: Color,
        secondary_color: Color,
        color_invert: ColorInvert,
        scale_factor: f64,
    ) -> Self {
        Self {
            key,
            panel: Coordinate::<DeviceContext>::new(
                Section::<DeviceContext>::new(
                    position.to_device(scale_factor),
                    Area::<DeviceContext>::new(area.width, area.height),
                ),
                layer,
            ),
            color,
            secondary_color,
            color_invert,
        }
    }
}
