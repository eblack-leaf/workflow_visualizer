use std::collections::{HashMap, HashSet};

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Resource;

use crate::icon::mesh::ColorInvert;
use crate::{Area, Color, Depth, IconKey, Position, UIView};

#[derive(Resource)]
pub(crate) struct Cache {
    pub(crate) icon_key: HashMap<Entity, IconKey>,
    pub(crate) depth: HashMap<Entity, Depth>,
    pub(crate) position: HashMap<Entity, Position<UIView>>,
    pub(crate) area: HashMap<Entity, Area<UIView>>,
    pub(crate) color: HashMap<Entity, Color>,
    pub(crate) color_invert: HashMap<Entity, ColorInvert>,
}

impl Cache {
    pub(crate) fn new() -> Self {
        Self {
            icon_key: HashMap::new(),
            depth: HashMap::new(),
            position: HashMap::new(),
            area: HashMap::new(),
            color: HashMap::new(),
            color_invert: HashMap::new(),
        }
    }
}

#[derive(Resource)]
pub(crate) struct DifferenceHolder {
    pub(crate) differences: Option<Differences>,
}

impl DifferenceHolder {
    pub(crate) fn new() -> Self {
        Self {
            differences: Some(Differences::new()),
        }
    }
}

#[derive(Resource)]
pub(crate) struct Differences {
    pub(crate) icon_adds: HashMap<
        Entity,
        (
            IconKey,
            Position<UIView>,
            Area<UIView>,
            Depth,
            Color,
            ColorInvert,
        ),
    >,
    pub(crate) icon_removes: HashSet<Entity>,
    pub(crate) depth: HashMap<Entity, Depth>,
    pub(crate) position: HashMap<Entity, Position<UIView>>,
    pub(crate) area: HashMap<Entity, Area<UIView>>,
    pub(crate) color: HashMap<Entity, Color>,
    pub(crate) color_invert: HashMap<Entity, ColorInvert>,
}

impl Differences {
    pub(crate) fn new() -> Self {
        Self {
            icon_adds: HashMap::new(),
            icon_removes: HashSet::new(),
            depth: HashMap::new(),
            position: HashMap::new(),
            area: HashMap::new(),
            color: HashMap::new(),
            color_invert: HashMap::new(),
        }
    }
    pub(crate) fn clean(&mut self) {
        let removed_entities = self.icon_removes.clone();
        for entity in removed_entities {
            self.position.remove(&entity);
            self.area.remove(&entity);
            self.depth.remove(&entity);
            self.color.remove(&entity);
            self.color_invert.remove(&entity);
        }
        let added_entities = self
            .icon_adds
            .iter()
            .map(|a| *a.0)
            .collect::<HashSet<Entity>>();
        for entity in added_entities {
            self.position.remove(&entity);
            self.area.remove(&entity);
            self.depth.remove(&entity);
            self.color.remove(&entity);
            self.color_invert.remove(&entity);
        }
    }
}
