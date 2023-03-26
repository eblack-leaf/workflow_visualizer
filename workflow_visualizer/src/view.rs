use bevy_ecs::prelude::Component;

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
