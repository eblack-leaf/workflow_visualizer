use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::instance::{AttributeUpdates, EntityKey, RemoveHandler, RequestHandler};
use crate::task::Container;
use crate::text::{GlyphOffset, RequestData};
use bevy_ecs::prelude::Resource;
use std::collections::{HashMap, HashSet};

#[derive(Resource)]
pub struct Extractor {
    pub request_handler: RequestHandler<EntityKey<GlyphOffset>, RequestData>,
    pub remove_handler: RemoveHandler<EntityKey<GlyphOffset>>,
}
impl Extractor {
    pub fn new() -> Self {
        Self {
            request_handler: RequestHandler::new(),
            remove_handler: RemoveHandler::new(),
        }
    }
}
