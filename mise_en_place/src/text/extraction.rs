use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Entity, Resource};

use crate::Color;
use crate::coord::{Depth, Position, Unscaled};
use crate::text::atlas::AtlasBlock;
use crate::text::difference::Difference;
use crate::text::render_group::{RenderGroupMax, RenderGroupUniqueGlyphs};
use crate::text::scale::TextScaleAlignment;
use crate::visibility::VisibleSection;

#[derive(Resource, Clone)]
pub(crate) struct Extraction {
    pub(crate) added_render_groups: HashMap<
        Entity,
        (
            RenderGroupMax,
            Position<Unscaled>,
            VisibleSection,
            Depth,
            AtlasBlock,
            RenderGroupUniqueGlyphs,
            TextScaleAlignment,
        ),
    >,
    pub(crate) removed_render_groups: HashSet<Entity>,
    pub(crate) differences: HashMap<Entity, Difference>,
}

impl Extraction {
    pub(crate) fn new() -> Self {
        Self {
            added_render_groups: HashMap::new(),
            removed_render_groups: HashSet::new(),
            differences: HashMap::new(),
        }
    }
}
