use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Component, Entity, Resource};

use crate::{Area, Color, InterfaceContext, Layer, Position};
use crate::icon::bitmap::TextureCoordinates;
use crate::icon::component::{ColorInvert, IconId};

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
    pub(crate) create: bool,
    pub(crate) remove: bool,
}
