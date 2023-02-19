use crate::coord::Panel;
use crate::{Area, Color, Depth, DeviceView, IconKey, Position, Section, UIView};

pub(crate) struct IconAdd {
    pub(crate) key: IconKey,
    pub(crate) panel: Panel<DeviceView>,
    pub(crate) color: Color,
}

impl IconAdd {
    pub(crate) fn new(
        key: IconKey,
        position: Position<UIView>,
        area: Area<UIView>,
        depth: Depth,
        color: Color,
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
        }
    }
}
