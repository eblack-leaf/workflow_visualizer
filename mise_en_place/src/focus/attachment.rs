use bevy_ecs::prelude::{IntoSystemDescriptor, SystemLabel};

use crate::clickable::ClickSystems;
use crate::focus::component::FocusedEntity;
use crate::focus::system;
use crate::{Attach, Engen, FrontEndStages};

pub struct FocusAttachment;

#[derive(SystemLabel)]
pub enum FocusSystems {
    SetFocused,
}

impl Attach for FocusAttachment {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .container
            .insert_resource(FocusedEntity::new(None));
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Prepare,
            system::set_focused
                .label(FocusSystems::SetFocused)
                .after(ClickSystems::RegisterClick),
        );
    }
}
