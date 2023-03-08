use bevy_ecs::prelude::{IntoSystemDescriptor, SystemLabel};

use crate::clickable::ClickSystems;
use crate::focus::FocusSystems;
use crate::text_input::system;
use crate::{Attach, Engen, FrontEndStages, IconDescriptors, IconMeshAddRequest};

pub struct TextInputAttachment;

#[derive(SystemLabel)]
pub enum TextInputSystems {
    CursorLocation,
    ReadInput,
}

impl Attach for TextInputAttachment {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::ResolvePrepare, system::position_ties);
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::ResolveStart,
            system::read_area_from_text_bound,
        );
        engen
            .frontend
            .container
            .spawn(IconMeshAddRequest::new(IconDescriptors::Cursor, 5));
        engen
            .frontend
            .container
            .spawn(IconMeshAddRequest::new(IconDescriptors::Panel, 5));
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Prepare,
            system::set_cursor_location
                .label(TextInputSystems::CursorLocation)
                .after(ClickSystems::RegisterClick),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Prepare,
            system::read_input_if_focused
                .label(TextInputSystems::ReadInput)
                .after(TextInputSystems::CursorLocation)
                .after(FocusSystems::SetFocused),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Prepare,
            system::open_virtual_keyboard.after(FocusSystems::SetFocused),
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::CoordPrepare, system::update_cursor_pos);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Spawn, system::spawn);
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::ResolvePrepare,
            system::cursor_letter_color_filter,
        );
    }
}
