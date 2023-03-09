use bevy_ecs::prelude::IntoSystemConfig;

use crate::clickable::system;
use crate::clickable::system::register_click;
use crate::{Attach, Engen, FrontEndBuckets};

pub struct ClickableAttachment;

impl Attach for ClickableAttachment {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .main
            .add_system(register_click.in_set(FrontEndBuckets::Prepare));
        engen
            .frontend
            .main
            .add_system(system::reset_click.in_set(FrontEndBuckets::Last));
    }
}
