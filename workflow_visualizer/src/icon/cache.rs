use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Component, Entity, Resource};

use crate::{Area, Color, InterfaceContext, Layer, Position};
use crate::icon::renderer::{ColorInvert, IconId, TextureCoordinates};

pub(crate) struct Attributes {
    pub(crate) icon_id: Option<IconId>,
    pub(crate) position: Option<Position<InterfaceContext>>,
    pub(crate) area: Option<Area<InterfaceContext>>,
    pub(crate) positive_space_color: Option<Color>,
    pub(crate) negative_space_color: Option<Color>,
    pub(crate) layer: Option<Layer>,
    pub(crate) color_invert: Option<ColorInvert>,
}

#[derive(Component)]
pub(crate) struct Cache {
    pub(crate) attributes: Attributes,
}

#[derive(Component)]
pub(crate) struct Difference {
    pub(crate) attributes: Attributes,
}

#[derive(Resource)]
pub(crate) struct Extraction {
    pub(crate) ext: HashMap<Entity, Difference>,
    pub(crate) removed: HashSet<Entity>,
}