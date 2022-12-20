use crate::text::r_rasterization::{get_reference, GlyphHash, Rasterization};
#[derive(Clone)]
pub(crate) struct Remove {
    pub(crate) hash: GlyphHash,
}
pub(crate) fn remove(rasterization: &mut Rasterization) {
    for remove in rasterization.removes.iter() {
        if get_reference(&rasterization.placement_references, remove.hash) == 0
            && !rasterization.retain_glyphs.contains(&remove.hash)
        {
            // remove request so can aggregate and insert / update swap
        }
    }
    rasterization.removes.clear();
}
