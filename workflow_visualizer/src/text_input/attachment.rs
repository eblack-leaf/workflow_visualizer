use bevy_ecs::prelude::IntoSystemConfig;

use crate::text_input::system::{
    cursor_letter_color_filter, open_virtual_keyboard, position_ties, read_input_if_focused,
    reconfigure_text_input, set_cursor_location, spawn, update_cursor_pos,
};
use crate::{panel, text, Attach, Engen, IconDescriptors, IconMeshAddRequest, SyncPoint, r_text};

pub struct TextInputAttachment;

impl Attach for TextInputAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.add_systems((
            read_input_if_focused.in_set(SyncPoint::Preparation),
            open_virtual_keyboard.in_set(SyncPoint::Preparation),
            spawn.in_set(SyncPoint::Spawn),
            position_ties.in_set(SyncPoint::Reconfigure),
            set_cursor_location
                .in_set(SyncPoint::Reconfigure)
                .after(reconfigure_text_input),
            cursor_letter_color_filter
                .in_set(SyncPoint::Reconfigure)
                .after(set_cursor_location),
            update_cursor_pos
                .in_set(SyncPoint::Reconfigure)
                .after(set_cursor_location),
            reconfigure_text_input
                .in_set(SyncPoint::Reconfigure)
                .after(r_text::scale_change)
                .before(panel::calc_content_area)
        ));
        engen
            .frontend
            .container
            .spawn(IconMeshAddRequest::new(IconDescriptors::Cursor, 5));
    }
}
