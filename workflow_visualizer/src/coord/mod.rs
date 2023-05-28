use bevy_ecs::bundle::Bundle;

use crate::{Layer, Section};

pub mod area;
pub mod layer;
pub mod location;
pub mod position;
pub mod section;
/// Coordinate Context - allows the struct Pos/Area/Layer... to be in different contexts to
/// differentiate how the data should be handled.
pub trait CoordContext
where
    Self: Send + Sync + 'static + Copy + Clone,
{
}
/// Device Context is used for actual physical positions such as hardware sizes
#[derive(Copy, Clone, PartialOrd, PartialEq, Default, Debug)]
pub struct DeviceContext;
/// Interface Context is used for logical coordinates after accounting for
/// scale factor
#[derive(Copy, Clone, PartialOrd, PartialEq, Default, Debug)]
pub struct InterfaceContext;
/// Numerical Context is used for numbers that are not associated with
/// device/logical sizes
#[derive(Copy, Clone, PartialOrd, PartialEq, Default, Debug)]
pub struct NumericalContext;
impl CoordContext for DeviceContext {}
impl CoordContext for InterfaceContext {}
impl CoordContext for NumericalContext {}
/// Coordinate is a bundle of Section + Layer to denote coordinates in the world
#[derive(Bundle, Copy, Clone, Default)]
pub struct Coordinate<Context: CoordContext> {
    #[bundle]
    pub section: Section<Context>,
    pub layer: Layer,
}

impl<Context: CoordContext> Coordinate<Context> {
    pub fn new<S: Into<Section<Context>>, L: Into<Layer>>(section: S, layer: L) -> Self {
        Self {
            section: section.into(),
            layer: layer.into(),
        }
    }
}
