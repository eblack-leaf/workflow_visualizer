use crate::enable;
use crate::enable::EntryEnabled;
use crate::entry_list::EntryIndex;
use crate::entry_list::{EntriesPerPage, TotalEntries};
use crate::positioning::EntryListPosition;
use workflow_visualizer::bevy_ecs;
use workflow_visualizer::bevy_ecs::change_detection::{DetectChanges, Res, ResMut};
use workflow_visualizer::bevy_ecs::prelude::{Commands, Query, Resource};

pub(crate) fn page_range(
    entries_per: Res<EntriesPerPage>,
    mut range: ResMut<PageRange>,
    mut entries: Query<(&mut EntryEnabled, &mut EntryListPosition, &EntryIndex)>,
    page: Res<Page>,
) {
    if entries_per.is_changed() || page.is_changed() {
        range.set(page.0, entries_per.0);
        for (mut enabled, mut pos, index) in entries.iter_mut() {
            enable::enable_entry(&range, &mut enabled, index, &mut pos);
        }
    }
}

#[derive(Resource, Copy, Clone)]
pub(crate) struct PageMax(pub(crate) u32);

pub(crate) fn set_max_page(
    mut page_max: ResMut<PageMax>,
    total_entries: Res<TotalEntries>,
    entries_per: Res<EntriesPerPage>,
) {
    if total_entries.is_changed() {
        page_max.0 = (total_entries.0 as f32 / entries_per.0 as f32).floor() as u32;
    }
}

#[derive(Resource)]
pub(crate) struct Page(pub(crate) u32);

#[derive(Resource)]
pub(crate) struct PageLeft(pub(crate) bool);

#[derive(Resource)]
pub(crate) struct PageRight(pub(crate) bool);

#[derive(Resource)]
pub(crate) struct PageRange(pub(crate) u32, pub(crate) u32);

impl PageRange {
    pub(crate) fn set(&mut self, page: u32, entries_per: u32) {
        self.0 = page * (entries_per - 1);
        self.1 = self.0 + (entries_per - 1);
    }
    pub(crate) fn contains(&self, index: EntryIndex) -> bool {
        let mut found = false;
        for i in self.0..self.1 {
            if i == index.0 {
                found = true;
            }
        }
        found
    }
    pub(crate) fn normalized(&self, index: EntryIndex) -> u32 {
        let diff = self.1 - self.0;
        let normal = index.0 % diff;
        normal
    }
}

pub(crate) fn setup_paging(mut cmd: Commands) {
    let page_max = PageMax(0);
    let page = Page(0);
    let page_left = PageLeft(false);
    let page_right = PageRight(false);
    let page_range = PageRange(0, 0);
    cmd.insert_resource(page_max);
    cmd.insert_resource(page);
    cmd.insert_resource(page_left);
    cmd.insert_resource(page_right);
    cmd.insert_resource(page_range);
}

pub(crate) fn page_change(
    mut entry_list_page: ResMut<Page>,
    mut page_left: ResMut<PageLeft>,
    mut page_right: ResMut<PageRight>,
    page_max: Res<PageMax>,
) {
    if page_left.0 {
        if entry_list_page.0 > 0 {
            entry_list_page.0 -= 1;
        }
        page_left.0 = false;
    }
    if page_right.0 {
        if entry_list_page.0 < page_max.0 {
            entry_list_page.0 += 1;
        }
        page_right.0 = false;
    }
}
