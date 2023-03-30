use std::collections::{HashMap, HashSet};

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Resource;

use crate::icon::mesh::ColorInvert;
use crate::icon::IconKey;
use crate::{Area, Color, InterfaceContext, Layer, Position};

#[derive(Resource)]
pub(crate) struct Cache {
    pub(crate) icon_key: HashMap<Entity, IconKey>,
    pub(crate) depth: HashMap<Entity, Layer>,
    pub(crate) position: HashMap<Entity, Position<InterfaceContext>>,
    pub(crate) area: HashMap<Entity, Area<InterfaceContext>>,
    pub(crate) color: HashMap<Entity, Color>,
    pub(crate) secondary_color: HashMap<Entity, Color>,
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
            secondary_color: HashMap::new(),
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
            Position<InterfaceContext>,
            Area<InterfaceContext>,
            Layer,
            Color,
            Color,
            ColorInvert,
        ),
    >,
    pub(crate) icon_removes: HashSet<Entity>,
    pub(crate) depth: HashMap<Entity, Layer>,
    pub(crate) position: HashMap<Entity, Position<InterfaceContext>>,
    pub(crate) area: HashMap<Entity, Area<InterfaceContext>>,
    pub(crate) color: HashMap<Entity, Color>,
    pub(crate) secondary_color: HashMap<Entity, Color>,
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
            secondary_color: HashMap::new(),
            color_invert: HashMap::new(),
        }
    }
    pub(crate) fn clean(&mut self) {
        let removed_entities = self.icon_removes.clone();
        self.clean_entity(removed_entities);
        let added_entities = self
            .icon_adds
            .iter()
            .map(|a| *a.0)
            .collect::<HashSet<Entity>>();
        self.clean_entity(added_entities);
    }

    fn clean_entity(&mut self, removed_entities: HashSet<Entity>) {
        for entity in removed_entities {
            self.position.remove(&entity);
            self.area.remove(&entity);
            self.depth.remove(&entity);
            self.color.remove(&entity);
            self.secondary_color.remove(&entity);
            self.color_invert.remove(&entity);
        }
    }
}
