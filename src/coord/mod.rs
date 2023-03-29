use crate::{Layer, Section};
use bevy_ecs::bundle::Bundle;

pub mod area;
pub mod layer;
pub mod location;
pub mod position;
pub mod section;

pub trait CoordContext
where
    Self: Send + Sync + 'static + Copy + Clone,
{
}
#[derive(Copy, Clone, PartialOrd, PartialEq, Default, Debug)]
pub struct DeviceContext;
#[derive(Copy, Clone, PartialOrd, PartialEq, Default, Debug)]
pub struct InterfaceContext;
#[derive(Copy, Clone, PartialOrd, PartialEq, Default, Debug)]
pub struct NumericalContext;
impl CoordContext for DeviceContext {}
impl CoordContext for InterfaceContext {}
impl CoordContext for NumericalContext {}

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
