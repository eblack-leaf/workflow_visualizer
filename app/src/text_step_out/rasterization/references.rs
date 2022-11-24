use crate::text_step_out::attributes::add::Adds;
use crate::text_step_out::attributes::remove::Removes;
use crate::text_step_out::attributes::write::Writes;
use crate::text_step_out::rasterization::rasterizations::RasterizedGlyphHash;
use crate::RasterizationPlacement;
use bevy_ecs::prelude::{Res, ResMut};
use std::cmp::max;
use std::collections::HashMap;
pub struct RasterizationReferences {
    pub references: HashMap<RasterizedGlyphHash, u32>,
}
impl RasterizationReferences {
    pub fn new() -> Self {
        Self {
            references: HashMap::new(),
        }
    }
    pub fn add_ref(&mut self, hash: RasterizedGlyphHash) {
        if let Some(ref_count) = self.references.get_mut(&hash) {
            *ref_count += 1;
        } else {
            self.references.insert(hash, 0);
        }
    }
    pub fn remove_ref(&mut self, hash: RasterizedGlyphHash) {
        if let Some(ref_count) = self.references.get_mut(&hash) {
            if *ref_count == 0 {
                return;
            }
            *ref_count -= 1;
        } else {
            self.references.insert(hash, 0);
        }
    }
}
