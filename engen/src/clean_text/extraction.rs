use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Changed, Entity, Query, ResMut, Resource};

use crate::canvas::Visibility;
use crate::clean_text::difference::Difference;

#[derive(Resource)]
pub(crate) struct Extraction {
    pub(crate) added_render_groups: HashMap<Entity, ()>,
    pub(crate) removed_render_groups: HashSet<Entity>,
}

impl Extraction {
    pub(crate) fn new() -> Self {
        Self {
            added_render_groups: HashMap::new(),
            removed_render_groups: HashSet::new(),
        }
    }
}

pub(crate) fn pull_differences(
    mut extraction: ResMut<Extraction>,
    differences: Query<(Entity, &mut Difference, &Visibility), Changed<Difference>>,
) {
    // drain from diffs into extraction if visible
}
