use bevy_ecs::prelude::IntoSystemConfig;

use crate::clickable::register_click;
use crate::focus::set_focused;
use crate::text_input::system;
use crate::text_input::system::set_cursor_location;
use crate::{Attach, Engen, FrontEndBuckets, IconDescriptors, IconMeshAddRequest};

pub struct TextInputAttachment;

impl Attach for TextInputAttachment {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .main
            .add_system(system::position_ties.in_set(FrontEndBuckets::ResolvePrepare));
        engen
            .frontend
            .main
            .add_system(system::read_area_from_text_bound.in_set(FrontEndBuckets::ResolveStart));
        engen
            .frontend
            .container
            .spawn(IconMeshAddRequest::new(IconDescriptors::Cursor, 5));
        engen
            .frontend
            .container
            .spawn(IconMeshAddRequest::new(IconDescriptors::Panel, 5));
        engen.frontend.main.add_system(
            system::set_cursor_location
                .in_set(FrontEndBuckets::Prepare)
                .after(register_click),
        );
        engen.frontend.main.add_system(
            system::read_input_if_focused
                .in_set(FrontEndBuckets::Prepare)
                .after(set_cursor_location)
                .after(set_focused),
        );
        engen.frontend.main.add_system(
            system::open_virtual_keyboard
                .in_set(FrontEndBuckets::Prepare)
                .after(set_focused),
        );
        engen
            .frontend
            .main
            .add_system(system::update_cursor_pos.in_set(FrontEndBuckets::CoordPrepare));
        engen
            .frontend
            .main
            .add_system(system::spawn.in_set(FrontEndBuckets::Spawn));
        engen
            .frontend
            .main
            .add_system(system::cursor_letter_color_filter.in_set(FrontEndBuckets::ResolvePrepare));
    }
}
