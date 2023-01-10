use crate::text::rasterization::RasterizationHandler;

pub(crate) fn resolve(rasterization: &mut RasterizationHandler) {
    // references are done now so can read and determine if should remove and shrink binding buffer
    // if shrunk need to update placement descriptor starts to reflect amount moved
}
