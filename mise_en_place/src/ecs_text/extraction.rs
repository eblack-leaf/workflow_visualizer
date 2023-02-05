use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Changed, Entity, Query, ResMut, Resource};

use crate::coord::{Area, Depth, Position};
use crate::ecs_text::atlas::AtlasBlock;
use crate::ecs_text::difference::Difference;
use crate::ecs_text::render_group::{RenderGroupMax, RenderGroupUniqueGlyphs};
use crate::ecs_text::scale::TextScaleAlignment;
use crate::visibility::{Visibility, VisibleSection};
use crate::Color;

#[derive(Resource, Clone)]
pub(crate) struct Extraction {
    pub(crate) added_render_groups: HashMap<
        Entity,
        (
            RenderGroupMax,
            Position,
            VisibleSection,
            Depth,
            Color,
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
