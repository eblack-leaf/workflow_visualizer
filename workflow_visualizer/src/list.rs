use std::collections::HashMap;
use crate::{GridPoint, GridView, InterfaceContext, Position, RawMarker};
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Commands;

pub struct List {
    pub entries: Vec<Entity>,
    pub positions: Vec<Position<InterfaceContext>>,
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
impl List {
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
            positions: vec![],
            page: 0,
            page_left: false,
            page_right: false,
            page_max: 0,
            entries_per_page: EntriesPerPage::new(
                vertical_markers,
                &entry_descriptor,
            ),
            anchor,
            entry_descriptor,
        }
    }
    /// Enable/Disable entries
    pub fn enablement(&self, cmd: &mut Commands) {
        todo!()
    }
    pub fn positions(&self) -> HashMap<Entity, GridPoint> {
        todo!()
    }
    pub fn insert(&mut self, index: usize, entity: Entity) {
        self.entries.insert(index, entity);
        // repage and position
    }
    pub fn add(&mut self, entity: Entity) {
        self.entries.push(entity);
        // repage and position
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
