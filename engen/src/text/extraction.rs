use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Changed, Entity, Query, ResMut, Resource};

use crate::text::difference::Difference;
use crate::visibility::Visibility;
use crate::{Area, Color, Depth, Position};

#[derive(Resource, Clone)]
pub(crate) struct Extraction {
    pub(crate) added_render_groups: HashMap<Entity, (usize, Position, Depth, Color, Area, usize)>,
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
