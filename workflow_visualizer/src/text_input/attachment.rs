use crate::text_input::system::{
    cursor_letter_color_filter, open_virtual_keyboard, position_ties, read_area_from_text_bound,
    read_input_if_focused, set_cursor_location, spawn, update_cursor_pos,
};
use crate::{Attach, Engen, IconDescriptors, IconMeshAddRequest};
use bevy_ecs::prelude::IntoSystemConfig;

pub struct TextInputAttachment;

impl Attach for TextInputAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.add_system(position_ties);
        engen.frontend.main.add_system(read_area_from_text_bound);
        engen
            .frontend
            .container
            .spawn(IconMeshAddRequest::new(IconDescriptors::Cursor, 5));
        engen.frontend.main.add_system(set_cursor_location);
        engen.frontend.main.add_system(read_input_if_focused);
        engen.frontend.main.add_system(open_virtual_keyboard);
        engen.frontend.main.add_system(update_cursor_pos);
        engen.frontend.main.add_system(spawn);
        engen.frontend.main.add_system(cursor_letter_color_filter);
    }
}
