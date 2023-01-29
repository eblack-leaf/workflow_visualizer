pub use crate::coord::area::Area;
pub(crate) use crate::coord::area::ScaledArea;
pub use crate::coord::depth::Depth;
pub use crate::coord::position::Position;
pub(crate) use crate::coord::position::ScaledPosition;
pub(crate) use crate::coord::section::ScaledSection;
pub use crate::coord::section::Section;

mod area;
mod depth;
mod position;
mod section;
