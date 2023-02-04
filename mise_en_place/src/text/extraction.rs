use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Changed, Entity, Query, ResMut, Resource};

use crate::coord::{Area, Depth, Position};
use crate::text::difference::Difference;
use crate::visibility::{Visibility, VisibleSection};
use crate::{Color, TextScaleAlignment};

#[derive(Resource, Clone)]
pub(crate) struct Extraction {
    pub(crate) added_render_groups: HashMap<
        Entity,
        (
            usize,
            Position,
            VisibleSection,
            Depth,
            Color,
            Area,
            usize,
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
