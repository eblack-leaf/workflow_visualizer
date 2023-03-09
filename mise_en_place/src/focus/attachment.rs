use bevy_ecs::prelude::IntoSystemConfig;

use crate::clickable::register_click;
use crate::focus::component::FocusedEntity;
use crate::focus::system::set_focused;
use crate::{Attach, Engen, FrontEndBuckets};

pub struct FocusAttachment;

impl Attach for FocusAttachment {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .container
            .insert_resource(FocusedEntity::new(None));
        engen.frontend.main.add_system(
            set_focused
                .in_set(FrontEndBuckets::Prepare)
                .after(register_click),
        );
    }
}
