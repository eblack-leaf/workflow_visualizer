use crate::{Attach, Engen, FrontEndStages};
pub use crate::coord::area::Area;
pub(crate) use crate::coord::area::GpuArea;
pub use crate::coord::area_adjust::AreaAdjust;
pub use crate::coord::depth::Depth;
pub use crate::coord::depth_adjust::DepthAdjust;
pub use crate::coord::location::Location;
pub(crate) use crate::coord::position::GpuPosition;
pub use crate::coord::position::Position;
pub use crate::coord::position_adjust::PositionAdjust;
pub use crate::coord::section::Section;

mod area;
mod area_adjust;
mod depth;
mod depth_adjust;
mod location;
mod panel;
mod position;
mod position_adjust;
mod section;

pub(crate) struct Coords;

pub trait CoordContext
    where
        Self: Copy + Clone + Send + Sync + 'static,
{}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Logical {}

impl CoordContext for Logical {}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct View {}

impl CoordContext for View {}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Device {}

impl CoordContext for Device {}

impl Attach for Coords {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::CoordAdjust,
            position_adjust::position_adjust::<View>,
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::CoordAdjust,
            area_adjust::area_adjust::<View>,
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::CoordAdjust, depth_adjust::depth_adjust);
    }
}
