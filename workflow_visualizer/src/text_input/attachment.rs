use crate::text_input::system::{
    cursor_letter_color_filter, open_virtual_keyboard, position_ties, read_input_if_focused,
    read_padding_change, reconfigure_text_input, set_cursor_location, spawn, update_cursor_pos,
};
use crate::{content_panel, text, Attach, Engen, IconDescriptors, IconMeshAddRequest, SyncPoint};
use bevy_ecs::prelude::IntoSystemConfig;

pub struct TextInputAttachment;

impl Attach for TextInputAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.add_system(
            read_padding_change
                .in_set(SyncPoint::Reconfigure)
                .before(position_ties),
        );
        engen
            .frontend
            .main
            .add_system(position_ties.in_set(SyncPoint::Reconfigure));
        engen
            .frontend
            .container
            .spawn(IconMeshAddRequest::new(IconDescriptors::Cursor, 5));
        engen.frontend.main.add_system(
            set_cursor_location
                .in_set(SyncPoint::Reconfigure)
                .after(reconfigure_text_input),
        );
        engen.frontend.main.add_system(
            reconfigure_text_input
                .in_set(SyncPoint::Reconfigure)
                .after(text::calc_scale_from_alignment)
                .after(text::calc_bound_from_guide)
                .before(content_panel::calc_area_from_content_area)
                .before(text::update_content),
        );
        engen
            .frontend
            .main
            .add_system(read_input_if_focused.in_set(SyncPoint::Preparation));
        engen
            .frontend
            .main
            .add_system(open_virtual_keyboard.in_set(SyncPoint::Preparation));
        engen.frontend.main.add_system(
            update_cursor_pos
                .in_set(SyncPoint::Reconfigure)
                .after(set_cursor_location),
        );
        engen
            .frontend
            .main
            .add_system(spawn.in_set(SyncPoint::Spawn));
        engen.frontend.main.add_system(
            cursor_letter_color_filter
                .in_set(SyncPoint::Reconfigure)
                .after(set_cursor_location),
        );
    }
}
