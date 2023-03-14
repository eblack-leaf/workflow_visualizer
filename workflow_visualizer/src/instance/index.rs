use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use bevy_ecs::prelude::Component;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Index {
    pub value: u32,
}

impl Index {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl From<u32> for Index {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

#[derive(Component)]
pub struct Indexer<Key: Eq + Hash + PartialEq + Copy + Clone + 'static> {
    pub indices: HashMap<Key, Index>,
    pub count: u32,
    pub max: u32,
    pub holes: HashSet<Index>,
}

impl<Key: Eq + Hash + PartialEq + Copy + Clone + 'static> Indexer<Key> {
    pub fn new(max: u32) -> Self {
        Self {
            indices: HashMap::new(),
            count: 0,
            max,
            holes: HashSet::new(),
        }
    }
    pub fn max(&self) -> u32 {
        self.max
    }
    pub fn count(&self) -> u32 {
        self.count
    }
    pub fn next(&mut self, key: Key) -> Index {
        let index = match self.holes.is_empty() {
            true => {
                self.count += 1;
                Index::new(self.count - 1)
            }
            false => {
                let index = *self.holes.iter().next().unwrap();
                self.holes.remove(&index);
                index
            }
        };
        self.indices.insert(key, index);
        index
    }
    pub fn remove(&mut self, key: Key) -> Option<Index> {
        if let Some(index) = self.indices.remove(&key) {
            self.holes.insert(index);
            return Some(index);
        }
        None
    }
    pub fn get_index(&self, key: Key) -> Option<Index> {
        self.indices.get(&key).copied()
    }
    pub fn should_grow(&mut self) -> bool {
        if self.count > self.max {
            while self.count > self.max {
                self.max += 1; // replace with growth factor
            }
            return true;
        }
        false
    }
}
