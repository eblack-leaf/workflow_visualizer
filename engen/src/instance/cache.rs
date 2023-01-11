use crate::task::Container;
use bevy_ecs::prelude::Resource;
use std::marker::PhantomData;

#[derive(Resource)]
pub struct Cache<Key> {
    pub container: Container,
    _key: PhantomData<Key>,
}
impl<Key> Cache<Key> {
    pub fn new() -> Self {
        Self {
            container: Container::new(),
            _key: PhantomData,
        }
    }
}
