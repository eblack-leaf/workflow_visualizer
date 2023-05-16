use bevy_ecs::prelude::{Changed, Component, DetectChanges, IntoSystemConfig, Query, Res};

use crate::viewport::ViewportHandle;
use crate::{Area, Attach, Engen, InterfaceContext, Position, SyncPoint};

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
impl ViewPosition {
    pub fn new(x: ViewPoint, y: ViewPoint) -> Self {
        Self { x, y }
    }
}
#[derive(Component, Copy, Clone)]
pub struct ViewArea {
    pub width: ViewPoint,
    pub height: ViewPoint,
}
impl ViewArea {
    pub fn new(width: ViewPoint, height: ViewPoint) -> Self {
        Self { width, height }
    }
}
pub(crate) fn set_from_view(
    mut positions: Query<(&mut Position<InterfaceContext>, &ViewPosition), Changed<ViewPosition>>,
    mut areas: Query<(&mut Area<InterfaceContext>, &ViewArea), Changed<ViewArea>>,
    viewport_handle: Res<ViewportHandle>,
) {
    for (mut position, view_position) in positions.iter_mut() {
        let new_position = calc_view_pos(&viewport_handle, view_position);
        // println!("new position: {:?}", new_position);
        *position = new_position;
    }
    for (mut area, view_area) in areas.iter_mut() {
        let new_area = calc_view_area(&viewport_handle, view_area);
        // println!("new area: {:?}", new_area);
        *area = new_area;
    }
}

fn calc_view_area(
    viewport_handle: &ViewportHandle,
    view_area: &ViewArea,
) -> Area<InterfaceContext> {
    let mut w = view_area.width.relative_point.value * viewport_handle.section.area.width;
    if let Some(breakpoint) = view_area.width.fixed_breakpoint {
        if w > breakpoint.0 {
            w = breakpoint.0;
        }
    }
    let mut h = view_area.height.relative_point.value * viewport_handle.section.area.height;
    if let Some(breakpoint) = view_area.height.fixed_breakpoint {
        if h > breakpoint.0 {
            h = breakpoint.0;
        }
    }
    Area::from((w, h))
}

fn calc_view_pos(
    viewport_handle: &ViewportHandle,
    view_position: &ViewPosition,
) -> Position<InterfaceContext> {
    let mut x = view_position.x.relative_point.value * viewport_handle.section.area.width;
    if let Some(breakpoint) = view_position.x.fixed_breakpoint {
        if x > breakpoint.0 {
            x = breakpoint.0;
        }
    }
    let mut y = view_position.y.relative_point.value * viewport_handle.section.area.height;
    if let Some(breakpoint) = view_position.y.fixed_breakpoint {
        if y > breakpoint.0 {
            y = breakpoint.0;
        }
    }
    Position::from((x, y))
}

pub(crate) fn resize_recalc(
    viewport_handle: Res<ViewportHandle>,
    mut viewed: Query<(&mut Position<InterfaceContext>, &ViewPosition)>,
    mut viewed_area: Query<(&mut Area<InterfaceContext>, &ViewArea)>,
) {
    if viewport_handle.is_changed() {
        for (mut pos, view_pos) in viewed.iter_mut() {
            let new_position = calc_view_pos(&viewport_handle, view_pos);
            // println!("new position: {:?}", new_position);
            *pos = new_position;
        }
        for (mut area, view_area) in viewed_area.iter_mut() {
            let new_area = calc_view_area(&viewport_handle, view_area);
            // println!("new area: {:?}", new_area);
            *area = new_area;
        }
    }
}
pub struct ViewAttachment;
impl Attach for ViewAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.add_systems((
            set_from_view.in_set(SyncPoint::Reconfigure),
            resize_recalc.in_set(SyncPoint::Reconfigure),
        ));
    }
}
