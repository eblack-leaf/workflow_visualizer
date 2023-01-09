use bevy_ecs::prelude::Resource;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct Index {
    pub(crate) value: usize,
}
impl Index {
    pub(crate) fn new(value: usize) -> Self {
        Self { value }
    }
}
impl From<usize> for Index {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}
#[derive(Resource)]
pub(crate) struct IndexHandler<Key: Eq + Hash + PartialEq + Copy + Clone + 'static> {
    pub(crate) indices: HashMap<Key, Index>,
    pub(crate) count: usize,
    pub(crate) max: usize,
    pub(crate) holes: HashSet<Index>,
}
impl<Key: Eq + Hash + PartialEq + Copy + Clone + 'static> IndexHandler<Key> {
    pub(crate) fn new(max: usize) -> Self {
        Self {
            indices: HashMap::new(),
            count: 0,
            max,
            holes: HashSet::new(),
        }
    }
    pub(crate) fn max(&self) -> usize {
        self.max
    }
    pub(crate) fn count(&self) -> usize {
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
        self.indices.remove(key)
    }
    pub(crate) fn get_index(&self, key: Key) -> Option<Index> {
        self.indices.get(&key).copied()
    }
}
