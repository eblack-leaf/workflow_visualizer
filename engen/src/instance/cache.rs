use std::marker::PhantomData;

use bevy_ecs::prelude::Resource;

use crate::task::Container;

// compute side cache to ameliorate the extraction and subsequent processing
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
