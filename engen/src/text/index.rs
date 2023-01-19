use bevy_ecs::prelude::Resource;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct Index {
    pub(crate) value: u32,
}
impl Index {
    pub(crate) fn new(value: u32) -> Self {
        Self { value }
    }
}
impl From<u32> for Index {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}
#[derive(Resource)]
pub(crate) struct Indexer<Key: Eq + Hash + PartialEq + Copy + Clone + 'static> {
    pub(crate) indices: HashMap<Key, Index>,
    pub(crate) count: u32,
    pub(crate) max: u32,
    pub(crate) holes: HashSet<Index>,
}
impl<Key: Eq + Hash + PartialEq + Copy + Clone + 'static> Indexer<Key> {
    pub(crate) fn new(max: u32) -> Self {
        Self {
            indices: HashMap::new(),
            count: 0,
            max,
            holes: HashSet::new(),
        }
    }
    pub(crate) fn max(&self) -> u32 {
        self.max
    }
    pub(crate) fn count(&self) -> u32 {
        self.count
    }
    pub(crate) fn current_index(&self) -> Option<Index> {
        match self.count() > 0 {
            true => Some(Index::new(self.count - 1)),
            false => None,
        }
    }
    pub(crate) fn next(&mut self, key: Key) -> Index {
        let index = match self.holes.is_empty() {
            true => {
                self.count += 1;
                Index::new(self.count - 1)
            }
            false => *self.holes.iter().next().unwrap(),
        };
        self.indices.insert(key, index);
        index
    }
    pub(crate) fn exists(&self, key: Key) -> bool {
        self.indices.contains_key(&key)
    }
    pub(crate) fn should_grow(&self) -> bool {
        self.count > self.max
    }
    pub(crate) fn remove(&mut self, key: &Key) -> Option<Index> {
        if let Some(index) = self.indices.remove(key) {
            self.holes.insert(index);
            return Some(index);
        }
        None
    }
    pub(crate) fn get_index(&self, key: Key) -> Option<Index> {
        self.indices.get(&key).copied()
    }
    pub(crate) fn grow(&mut self, growth_factor: u32) {
        while self.count > self.max {
            self.max += growth_factor;
        }
    }
}
