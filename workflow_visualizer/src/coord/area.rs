use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Add, Mul, Sub};

use bevy_ecs::component::Component;
use bevy_ecs::prelude::Query;
use bytemuck::{Pod, Zeroable};

use crate::coord::{CoordinateContext, NumericalContext, WindowAppearanceContext};
use crate::{
    Animate, Animation, DeviceContext, InterfaceContext, Interpolation, WindowAppearanceFactor,
};
/// Area is for width/height of an entity
#[derive(Component, Copy, Clone, PartialOrd, PartialEq, Default)]
pub struct Area<Context: CoordinateContext> {
    pub width: f32,
    pub height: f32,
    _context: PhantomData<Context>,
}
impl<Context: CoordinateContext> Debug for Area<Context> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Area: width:{:.2} height:{:.2}", self.width, self.height)
    }
}
impl<Context: CoordinateContext> Area<Context> {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            _context: PhantomData,
        }
    }
    /// return a copy as just a number
    pub fn as_numerical(&self) -> Area<NumericalContext> {
        Area::<NumericalContext>::new(self.width, self.height)
    }
    /// return a copy as raw struct for gpu interactions
    pub fn as_raw(&self) -> RawArea {
        RawArea {
            width: self.width,
            height: self.height,
        }
    }
}
impl Area<InterfaceContext> {
    /// accounts for scale factor to convert this to device area
    pub fn to_device(&self, scale_factor: f32) -> Area<DeviceContext> {
        Area::<DeviceContext>::new(self.width * scale_factor, self.height * scale_factor)
    }
}

impl Area<DeviceContext> {
    /// accounts for scale factor to convert this to interface area
    pub fn to_interface(&self, scale_factor: f32) -> Area<InterfaceContext> {
        Area::<InterfaceContext>::new(self.width / scale_factor, self.height / scale_factor)
    }
    pub fn to_window_appearance(
        &self,
        window_appearance_factor: &WindowAppearanceFactor,
    ) -> Area<WindowAppearanceContext> {
        Area::new(
            self.width * window_appearance_factor.x_factor(),
            self.height * window_appearance_factor.y_factor(),
        )
    }
}
impl Area<WindowAppearanceContext> {
    pub fn to_actual(
        &self,
        window_appearance_factor: &WindowAppearanceFactor,
    ) -> Area<DeviceContext> {
        Area::new(
            self.width / window_appearance_factor.x_factor(),
            self.height / window_appearance_factor.y_factor(),
        )
    }
}
/// Raw area defined in C representation for interacting with C (vulkan mostly)
#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Default)]
pub struct RawArea {
    width: f32,
    height: f32,
}

impl RawArea {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
impl<Context: CoordinateContext> From<(usize, usize)> for Area<Context> {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0 as f32, value.1 as f32)
    }
}

impl<Context: CoordinateContext> From<(i32, i32)> for Area<Context> {
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0 as f32, value.1 as f32)
    }
}

impl<Context: CoordinateContext> From<(f32, f32)> for Area<Context> {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<Context: CoordinateContext> From<(u32, u32)> for Area<Context> {
    fn from(value: (u32, u32)) -> Self {
        Self::new(value.0 as f32, value.1 as f32)
    }
}

impl<Context: CoordinateContext> Mul for Area<Context> {
    type Output = Area<Context>;
    fn mul(self, rhs: Self) -> Self::Output {
        Area::<Context>::new(self.width * rhs.width, self.height * rhs.height)
    }
}
impl<Context: CoordinateContext> Add for Area<Context> {
    type Output = Area<Context>;
    fn add(self, rhs: Self) -> Self::Output {
        Area::<Context>::new(self.width + rhs.width, self.height + rhs.height)
    }
}
impl<Context: CoordinateContext> Sub for Area<Context> {
    type Output = Area<Context>;
    fn sub(self, rhs: Self) -> Self::Output {
        Area::<Context>::new(self.width - rhs.width, self.height - rhs.height)
    }
}
impl<Context: CoordinateContext> Animate for Area<Context> {
    fn interpolations(&self, end: Self) -> Vec<Interpolation> {
        vec![
            Interpolation::new(end.width - self.width),
            Interpolation::new(end.height - self.height),
        ]
    }
}
pub(crate) fn apply_animation<Context: CoordinateContext>(
    mut positions: Query<(&mut Area<Context>, &mut Animation<Area<Context>>)>,
) {
    for (mut pos, mut anim) in positions.iter_mut() {
        let extracts = anim.extractions();
        if let Some(extract) = extracts.get(0).unwrap() {
            pos.width += extract.0;
        }
        if let Some(extract) = extracts.get(1).unwrap() {
            pos.height += extract.0;
        }
    }
}
