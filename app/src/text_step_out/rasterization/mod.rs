pub use crate::text_step_out::rasterization::rasterizations::Rasterizations;
pub use crate::text_step_out::rasterization::rasterizations::{
    rasterize, setup_rasterization, RasterizationRequest, RasterizationRequestCallPoint,
    RasterizedGlyphHash,
};

pub mod placement;
mod rasterizations;
pub(crate) mod references;

// rasterize writes - hook into writes and raster them
// integrate appended rasterizations + shrink removes
// so rewrite from cpu_buffer + appended rasterizations
// once resolved send rast. placements back to entity
// remove refs to removes
//
