use crate::{GridPoint, RawMarker};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct List<Key: Copy + Clone + Hash + Eq + PartialEq> {
    pub entries: Vec<Key>,
    pub page: u32,
    pub page_left: bool,
    pub page_right: bool,
    pub page_max: u32,
    pub entries_per_page: EntriesPerPage,
    pub anchor: GridPoint,
    pub entry_descriptor: ListEntryDescriptor,
}
pub struct EntriesPerPage(u32);
impl EntriesPerPage {
    pub fn new(vertical_markers: RawMarker, entry_descriptor: &ListEntryDescriptor) -> Self {
        Self((vertical_markers / (entry_descriptor.height + entry_descriptor.padding)).0 as u32)
    }
    pub fn value(&self) -> u32 {
        self.0
    }
}
impl<Key: Copy + Clone + Hash + Eq + PartialEq> List<Key> {
    pub fn new(
        anchor: GridPoint,
        horizontal_markers: RawMarker,
        vertical_markers: RawMarker,
        entry_height: RawMarker,
        padding: RawMarker,
    ) -> Self {
        let entry_descriptor = ListEntryDescriptor::new(horizontal_markers, entry_height, padding);
        Self {
            entries: vec![],
            page: 0,
            page_left: false,
            page_right: false,
            page_max: 0,
            entries_per_page: EntriesPerPage::new(vertical_markers, &entry_descriptor),
            anchor,
            entry_descriptor,
        }
    }
    /// Enable/Disable entries
    pub fn enablement(&self) -> HashMap<Key, bool> {
        let mut mapping = HashMap::new();
        let range = self.range();
        let mut set = HashSet::new();
        for x in range.0..range.1 {
            if let Some(key) = self.entries.get(x) {
                set.insert(*key);
            }
        }
        for key in self.entries.iter() {
            mapping.insert(*key, if set.contains(key) { true } else { false });
        }
        mapping
    }
    pub fn entry_position(&self, index: i32) -> GridPoint {
        GridPoint::new(
            self.anchor.x.raw_offset(self.entry_descriptor.padding),
            self.anchor.y.raw_offset(
                (self.entry_descriptor.height + self.entry_descriptor.padding).0 * index,
            ),
        )
    }
    pub fn page_left(&mut self) {
        self.page = self.page.checked_sub(1).unwrap_or_default();
    }
    pub fn page_right(&mut self) {
        self.page = (self.page + 1).min(self.page_max);
    }
    fn range(&self) -> (usize, usize) {
        let amount = self.entries_per_page.value() - 1;
        let start = self.page * amount;
        let end = start + amount;
        (start as usize, end as usize)
    }
    pub fn positions(&self) -> HashMap<Key, GridPoint> {
        let mut index = 0;
        let mut mapping = HashMap::new();
        let (start, end) = self.range();
        for slot in start..end {
            if let Some(key) = self.entries.get(slot) {
                mapping.insert(*key, self.entry_position(index));
                index += 1;
            }
        }
        mapping
    }
    pub fn insert(&mut self, index: usize, key: Key) {
        self.entries.insert(index, key);
        self.calc_page_max();
    }
    pub fn add(&mut self, key: Key) {
        self.entries.push(key);
        self.calc_page_max();
    }
    pub fn remove(&mut self, key: Key) {
        self.entries.retain(|e| *e != key);
        self.calc_page_max();
    }
    fn calc_page_max(&mut self) {
        self.page_max = self.entries.len() as u32 / self.entries_per_page.value();
        if self.page > self.page_max {
            self.page = self.page_max;
        }
    }
}
pub struct ListEntryDescriptor {
    pub height: RawMarker,
    pub padding: RawMarker,
    pub width: RawMarker,
}
impl ListEntryDescriptor {
    pub fn new(list_width: RawMarker, entry_height: RawMarker, padding: RawMarker) -> Self {
        Self {
            height: entry_height,
            padding,
            width: list_width - RawMarker(2) * padding,
        }
    }
}
