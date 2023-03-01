pub use crate::coord::area::Area;
pub use crate::coord::area::GpuArea;
pub use crate::coord::area_adjust::AreaAdjust;
pub use crate::coord::depth::Depth;
pub use crate::coord::depth_adjust::DepthAdjust;
pub use crate::coord::location::Location;
pub use crate::coord::panel::Panel;
pub use crate::coord::position::GpuPosition;
pub use crate::coord::position::Position;
pub use crate::coord::position_adjust::{PositionAdjust, PositionAdjustAnimator};
pub use crate::coord::section::Section;
use crate::engen::{Attach, Engen};
use crate::engen::FrontEndStages;

mod area;
mod area_adjust;
mod depth;
mod depth_adjust;
mod location;
mod panel;
mod position;
mod position_adjust;
mod section;

pub(crate) struct CoordAttachment;

pub trait CoordContext
    where
        Self: Copy + Clone + Send + Sync + 'static,
{}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Numerical;

impl CoordContext for Numerical {}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct UIView;

impl CoordContext for UIView {}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct DeviceView;

impl CoordContext for DeviceView {}

impl Attach for CoordAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::CoordAdjust,
            position_adjust::position_adjust::<UIView>,
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::CoordAdjust,
            area_adjust::area_adjust::<UIView>,
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::CoordAdjust, depth_adjust::depth_adjust);
    }
}
