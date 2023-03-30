use bevy_ecs::prelude::IntoSystemConfig;

use crate::text_input::system::{
    area_ties, cursor_letter_color_filter, open_virtual_keyboard, position_ties,
    read_input_if_focused, set_cursor_location, spawn, update_cursor_pos,
};
use crate::view::set_from_view;
use crate::{icon, panel, text, Attach, Engen, IconDescriptors, IconMeshAddRequest, SyncPoint};

pub struct TextInputAttachment;

impl Attach for TextInputAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.add_systems((
            read_input_if_focused.in_set(SyncPoint::Preparation),
            open_virtual_keyboard.in_set(SyncPoint::Preparation),
            spawn.in_set(SyncPoint::Spawn),
            position_ties
                .in_set(SyncPoint::Reconfigure)
                .before(set_from_view),
            area_ties
                .in_set(SyncPoint::Reconfigure)
                .after(text::scale_change)
                .before(set_from_view),
            set_cursor_location
                .in_set(SyncPoint::PushDiff)
                .after(text::color_diff),
            cursor_letter_color_filter
                .in_set(SyncPoint::PushDiff)
                .after(text::color_diff)
                .after(set_cursor_location),
            update_cursor_pos
                .in_set(SyncPoint::PushDiff)
                .after(set_cursor_location)
                .before(icon::position_cache_check),
        ));
        engen
            .frontend
            .container
            .spawn(IconMeshAddRequest::new(IconDescriptors::Cursor, 5));
    }
}
