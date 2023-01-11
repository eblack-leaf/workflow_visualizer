use crate::text::rasterization::RasterizationHandler;

pub(crate) fn resolve(rasterization: &mut RasterizationHandler) {
    // references are done now so can read and determine if should remove and shrink binding buffer
    let mut removals = Vec::new();
    for (glyph_hash, reference) in rasterization.references.iter() {
        if reference.count == 0 {
            if !rasterization.retain_glyphs.contains(glyph_hash) {
                removals.push(*glyph_hash);
            }
        }
    }
    // combine removals
    for glyph_hash in removals.iter() {}
    // if shrunk need to update placement descriptor starts to reflect amount moved
    // send update_attr for placement descriptor to coordinator or buffer that will be sent

    // if this needs to swap write placement descriptor, then spot write the coordinator buffer
    // (or send message to do so)
}
