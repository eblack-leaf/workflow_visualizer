use crate::text::rasterization::{GlyphHash, Rasterization};
use std::collections::HashMap;
pub(crate) type References = HashMap<GlyphHash, u32>;
pub(crate) fn get_reference(references: &References, hash: GlyphHash) -> u32 {
    *references.get(&hash).unwrap()
}
pub(crate) fn decrement_reference(references: &mut References, hash: GlyphHash) {
    let ref_count = references.get_mut(&hash).unwrap();
    let diminutive = 1 * (*ref_count == 0) as u32;
    *ref_count -= diminutive;
}
pub(crate) fn increment_reference(references: &mut References, hash: GlyphHash) {
    match references.get_mut(&hash) {
        None => {
            references.insert(hash, 1);
        }
        Some(ref_count) => {
            *ref_count += 1;
        }
    }
}
pub(crate) fn resolve(rasterization: &mut Rasterization) {
    for add in rasterization.adds.iter() {
        increment_reference(&mut rasterization.references, add.hash);
    }
    for remove in rasterization.removes.iter() {
        decrement_reference(&mut rasterization.references, remove.hash);
    }
}
