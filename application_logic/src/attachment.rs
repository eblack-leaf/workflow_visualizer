use workflow_visualizer::bevy_ecs::prelude::{IntoSystemConfigs, ResMut};
use workflow_visualizer::{
    Attach, BundledIcon, BundledSvg, IconBitmap, IconBitmapRequest, IconHandle, Idle,
    ResourceHandle, SvgRequest, SyncPoint, TessellatedSvg, Visualizer,
};

use crate::entry::{EntryAddToken, EntryRemoveToken, ReadOtp};
use crate::entry_list::{ListDimensions, ReceivedTokens};
use crate::{bottom_panel, enable, entry, entry_list, paging, positioning};

pub struct EntryAttachment;
#[repr(i32)]
#[derive(Copy, Clone)]
pub(crate) enum IconHandles {
    Edit,
    Add,
    PageLeft,
    PageRight,
    Run,
    Delete,
    Generate,
}
impl IconHandles {
    pub(crate) fn handle(&self) -> ResourceHandle {
        (*self as i32).into()
    }
}

impl Attach for EntryAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.add_event::<ReceivedTokens>();
        visualizer.add_event::<ReadOtp>();
        visualizer.add_event::<EntryAddToken>();
        visualizer.add_event::<EntryRemoveToken>();
        visualizer.spawn(SvgRequest::new(
            IconHandles::Edit.handle(),
            BundledSvg::Test.tessellation(),
        ));
        visualizer.spawn(SvgRequest::new(
            IconHandles::Add.handle(),
            BundledSvg::Activity.tessellation(),
        ));
        visualizer.spawn(SvgRequest::new(
            IconHandles::PageLeft.handle(),
            BundledSvg::Activity.tessellation(),
        ));
        visualizer.spawn(SvgRequest::new(
            IconHandles::PageRight.handle(),
            BundledSvg::Activity.tessellation(),
        ));
        visualizer.spawn(SvgRequest::new(
            IconHandles::Run.handle(),
            BundledSvg::Activity.tessellation(),
        ));
        visualizer.spawn(SvgRequest::new(
            IconHandles::Delete.handle(),
            BundledSvg::Activity.tessellation(),
        ));
        visualizer.spawn(SvgRequest::new(
            IconHandles::Generate.handle(),
            BundledSvg::Activity.tessellation(),
        ));
        visualizer
            .job
            .container
            .insert_resource(ListDimensions::default());
        visualizer.job.task(Visualizer::TASK_STARTUP).add_systems((
            entry::request_tokens.in_set(SyncPoint::PostInitialization),
            paging::setup_paging.in_set(SyncPoint::PostInitialization),
            positioning::setup_entry_list_placements.in_set(SyncPoint::PostInitialization),
            entry_list::setup_entry_scale.in_set(SyncPoint::PostInitialization),
            entry_list::setup_removed_entry_indices.in_set(SyncPoint::PostInitialization),
            entry_list::setup_total_entries.in_set(SyncPoint::PostInitialization),
            entry_list::setup_entry_list.in_set(SyncPoint::PostInitialization),
            bottom_panel::setup_bottom_panel_buttons.in_set(SyncPoint::PostResolve),
        ));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            entry_list::dimension_change.in_set(SyncPoint::PostInitialization),
            positioning::entry_list_placements
                .in_set(SyncPoint::Preparation)
                .after(entry_list::entry_list_layout),
            paging::page_range
                .in_set(SyncPoint::Spawn)
                .after(paging::page_change),
            paging::set_max_page
                .in_set(SyncPoint::Spawn)
                .before(paging::page_change),
            paging::page_change.in_set(SyncPoint::Spawn),
            entry::display_name.in_set(SyncPoint::Reconfigure),
            entry::read_otp.in_set(SyncPoint::PostInitialization),
            entry::display_otp.in_set(SyncPoint::Reconfigure),
            bottom_panel::process_bottom_panel_buttons.in_set(SyncPoint::Process),
            bottom_panel::place_bottom_panel_buttons.in_set(SyncPoint::Spawn),
            positioning::position
                .in_set(SyncPoint::Spawn)
                .after(entry_list::enable_by_index_change),
            enable::enable
                .in_set(SyncPoint::Spawn)
                .after(paging::page_range),
            entry_list::enable_by_index_change
                .in_set(SyncPoint::Spawn)
                .before(enable::enable),
            entry_list::removed_indices
                .in_set(SyncPoint::Spawn)
                .before(paging::set_max_page),
            entry_list::entry_list_layout.in_set(SyncPoint::Preparation),
        ));
        visualizer.job.task(Visualizer::TASK_MAIN).add_systems((
            entry_list::receive_tokens.in_set(SyncPoint::PostInitialization),
            entry::receive_add_token.in_set(SyncPoint::PostInitialization),
            entry::receive_remove_token.in_set(SyncPoint::PostInitialization),
            entry::process_entry_buttons.in_set(SyncPoint::Process),
            stop_idle.in_set(SyncPoint::Process),
        ));
    }
}

fn stop_idle(mut _idle: ResMut<Idle>) {
    #[cfg(target_family = "wasm")]
    {
        _idle.can_idle = false;
    }
}
