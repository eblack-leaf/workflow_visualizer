use std::ops::{Add, Sub};
use bevy_ecs::component::Component;
use std::marker::PhantomData;
use bytemuck::{Pod, Zeroable};
use crate::coord::{CoordContext, NumericalContext};

#[derive(Component, Copy, Clone)]
pub struct Position<Context: CoordContext> {
    pub x: f32,
    pub y: f32,
    _context: PhantomData<Context>,
}

impl<Context: CoordContext> Position<Context> {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            _context: PhantomData,
        }
    }
    pub fn as_numerical(&self) -> Position<NumericalContext> {
        Position::<NumericalContext>::new(self.x, self.y)
    }
    pub fn as_raw(&self) -> RawPosition {
        RawPosition {
            x: self.x,
            y: self.y,
        }
    }
}

impl<Context: CoordContext> Add for Position<Context> {
    type Output = Position<Context>;
    fn add(self, rhs: Self) -> Self::Output {
        Position::<Context>::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<Context: CoordContext> Sub for Position<Context> {
    type Output = Position<Context>;
    fn sub(self, rhs: Self) -> Self::Output {
        Position::<Context>::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
pub struct RawPosition {
    x: f32,
    y: f32,
}

impl RawPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }
}
