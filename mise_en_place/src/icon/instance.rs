use crate::coord::Panel;
use crate::icon::mesh::ColorInvert;
use crate::{Area, Color, Depth, DeviceView, IconKey, Position, Section, UIView};

pub(crate) struct IconAdd {
    pub(crate) key: IconKey,
    pub(crate) panel: Panel<DeviceView>,
    pub(crate) color: Color,
    pub(crate) secondary_color: Color,
    pub(crate) color_invert: ColorInvert,
}

impl IconAdd {
    pub(crate) fn new(
        key: IconKey,
        position: Position<UIView>,
        area: Area<UIView>,
        depth: Depth,
        color: Color,
        secondary_color: Color,
        color_invert: ColorInvert,
        scale_factor: f64,
    ) -> Self {
        Self {
            key,
            panel: Panel::<DeviceView>::new(
                Section::<DeviceView>::new(
                    position.to_device(scale_factor),
                    Area::<DeviceView>::new(area.width, area.height),
                ),
                depth,
            ),
            color,
            secondary_color,
            color_invert,
        }
    }
}
