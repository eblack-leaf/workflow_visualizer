use crate::text::rasterization::{PlacementDescriptor, RasterizationHandler};
use crate::Canvas;
pub(crate) fn write(rasterization: &mut RasterizationHandler, canvas: &Canvas) {
    // write bitmap to buffer and return placement_descriptor for integration into coordinator
    for write in rasterization.write_requests.iter() {
        let glyph = rasterization.cached_rasterized_glyphs.get(write).expect("");
        let start = rasterization.binding.queue_bitmap(glyph.bitmap.clone());
        let placement_descriptor =
            PlacementDescriptor::new(start, glyph.metrics.width, glyph.metrics.height);
        rasterization
            .placement_descriptors
            .insert(*write, placement_descriptor);
    }
    rasterization.binding.write_queued(canvas);
}
