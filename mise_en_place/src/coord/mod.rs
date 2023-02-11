pub use crate::coord::area::Area;
pub use crate::coord::area::ScaledArea;
pub use crate::coord::area_adjust::{AreaAdjust, ScaledAreaAdjust};
pub use crate::coord::depth::Depth;
pub use crate::coord::depth_adjust::DepthAdjust;
pub use crate::coord::position::Position;
pub use crate::coord::position::ScaledPosition;
pub use crate::coord::position_adjust::{PositionAdjust, ScaledPositionAdjust};
pub use crate::coord::section::ScaledSection;
pub use crate::coord::section::Section;
use crate::{Attach, Engen, FrontEndStages};

mod area;
mod area_adjust;
mod depth;
mod depth_adjust;
mod position;
mod position_adjust;
mod section;

pub(crate) struct Coords;

impl Attach for Coords {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::CoordAdjust,
            position_adjust::position_adjust,
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::CoordAdjust, area_adjust::area_adjust);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::CoordAdjust, depth_adjust::depth_adjust);
    }
}
