use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Component, Entity, Resource};

use crate::icon::component::IconId;
use crate::{Area, Color, InterfaceContext, Layer, Position};

pub(crate) struct Attributes {
    pub(crate) icon_id: Option<IconId>,
    pub(crate) position: Option<Position<InterfaceContext>>,
    pub(crate) area: Option<Area<InterfaceContext>>,
    pub(crate) positive_space_color: Option<Color>,
    pub(crate) layer: Option<Layer>,
}

impl Attributes {
    fn new() -> Attributes {
        Self {
            icon_id: None,
            position: None,
            area: None,
            positive_space_color: None,
            layer: None,
        }
    }
}

#[derive(Component)]
pub(crate) struct Cache {
    pub(crate) attributes: Attributes,
}

impl Cache {
    pub(crate) fn new() -> Cache {
        Self {
            attributes: Attributes::new(),
        }
    }
}

#[derive(Component)]
pub(crate) struct Difference {
    pub(crate) attributes: Attributes,
    pub(crate) create: bool,
    pub(crate) remove: bool,
}

impl Difference {
    pub(crate) fn new() -> Difference {
        Self {
            attributes: Attributes::new(),
            create: false,
            remove: false,
        }
    }
}
