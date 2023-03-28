use crate::viewport::ViewportHandle;
use crate::{Area, Attach, Engen, InterfaceContext, Position};
use bevy_ecs::prelude::{Changed, Component, IntoSystemConfig, Query, Res};

#[derive(Component, Copy, Clone)]
pub struct RelativePoint {
    pub value: f32,
}
impl RelativePoint {
    pub fn new(value: f32) -> Self {
        assert!(value >= 0.0);
        assert!(value <= 1.0);
        Self { value }
    }
}
#[derive(Component, Copy, Clone)]
pub struct FixedBreakPoint(pub f32);
#[derive(Component, Copy, Clone)]
pub struct ViewPoint {
    pub relative_point: RelativePoint,
    pub fixed_breakpoint: Option<FixedBreakPoint>,
}
impl ViewPoint {
    pub fn new(relative: RelativePoint, fixed: Option<FixedBreakPoint>) -> Self {
        Self {
            relative_point: relative,
            fixed_breakpoint: fixed,
        }
    }
}
impl From<(f32, Option<f32>)> for ViewPoint {
    fn from(value: (f32, Option<f32>)) -> Self {
        let fixed = match value.1 {
            None => None,
            Some(f) => Some(FixedBreakPoint(f)),
        };
        ViewPoint::new(RelativePoint::new(value.0), fixed)
    }
}
#[derive(Component, Copy, Clone)]
pub struct ViewPosition {
    pub x: ViewPoint,
    pub y: ViewPoint,
}
#[derive(Component, Copy, Clone)]
pub struct ViewArea {
    pub width: ViewPoint,
    pub height: ViewPoint,
}
pub(crate) fn set_from_view(
    mut positions: Query<(&mut Position<InterfaceContext>, &ViewPosition), Changed<ViewPosition>>,
    mut areas: Query<(&mut Area<InterfaceContext>, &ViewArea), Changed<ViewArea>>,
    viewport_handle: Res<ViewportHandle>,
) {
    for (mut position, view_position) in positions.iter_mut() {
        let mut x = view_position.x.relative_point.value * viewport_handle.section.area.width;
        if let Some(breakpoint) = view_position.x.fixed_breakpoint {
            if breakpoint.0 > x {
                x = breakpoint.0;
            }
        }
        let mut y = view_position.y.relative_point.value * viewport_handle.section.area.height;
        if let Some(breakpoint) = view_position.y.fixed_breakpoint {
            if breakpoint.0 > y {
                y = breakpoint.0;
            }
        }
        *position = Position::from((x, y));
    }
    for (mut area, view_area) in areas.iter_mut() {
        let mut w = view_area.width.relative_point.value * viewport_handle.section.area.width;
        if let Some(breakpoint) = view_area.width.fixed_breakpoint {
            if breakpoint.0 > w {
                w = breakpoint.0;
            }
        }
        let mut h = view_area.height.relative_point.value * viewport_handle.section.area.height;
        if let Some(breakpoint) = view_area.height.fixed_breakpoint {
            if breakpoint.0 > h {
                h = breakpoint.0;
            }
        }
        *area = Area::from((w, h));
    }
}

pub struct ViewAttachment;
impl Attach for ViewAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.add_systems((set_from_view.in_set(),));
    }
}
