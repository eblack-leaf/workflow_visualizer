use std::hash::Hash;

pub struct Coordinator<Key: Eq + Hash + PartialEq + Copy + Clone, Request> {}
