use bevy_ecs::prelude::{IntoSystemDescriptor, SystemLabel};

use crate::clickable::system;
use crate::{Attach, Engen, FrontEndStages};

pub struct ClickableAttachment;

#[derive(SystemLabel)]
pub enum ClickSystems {
    RegisterClick,
}

impl Attach for ClickableAttachment {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Prepare,
            system::register_click.label(ClickSystems::RegisterClick),
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Last, system::reset_click);
    }
}
