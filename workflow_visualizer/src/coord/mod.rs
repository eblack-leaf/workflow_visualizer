use bevy_ecs::bundle::Bundle;
use bevy_ecs::prelude::IntoSystemConfigs;

use crate::{Area, Attach, Layer, Position, Section, SyncPoint, Visualizer};

pub mod area;
pub mod layer;
pub mod location;
pub mod position;
pub mod section;
/// Coordinate Context - allows the struct Pos/Area/Layer... to be in different contexts to
/// differentiate how the data should be handled.
pub trait CoordinateContext
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
impl CoordinateContext for DeviceContext {}
impl CoordinateContext for InterfaceContext {}
impl CoordinateContext for NumericalContext {}
/// Coordinate is a bundle of Section + Layer to denote coordinates in the world
#[derive(Bundle, Copy, Clone, Default)]
pub struct Coordinate<Context: CoordinateContext> {
    pub section: Section<Context>,
    pub layer: Layer,
}

impl<Context: CoordinateContext> Coordinate<Context> {
    pub fn new<S: Into<Section<Context>>, L: Into<Layer>>(section: S, layer: L) -> Self {
        Self {
            section: section.into(),
            layer: layer.into(),
        }
    }
}
pub(crate) struct CoordinateAttachment;
impl Attach for CoordinateAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.register_animation::<Position<InterfaceContext>>();
        visualizer.register_animation::<Area<InterfaceContext>>();
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            position::apply_animation::<InterfaceContext>.in_set(SyncPoint::Animation),
            area::apply_animation::<InterfaceContext>.in_set(SyncPoint::Animation),
        ));
    }
}
