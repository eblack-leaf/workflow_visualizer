use bevy_ecs::event::Events;
use bevy_ecs::prelude::IntoSystemConfig;

use crate::window::orientation;
use crate::{
    Attach, BackendBuckets, ClickEvent, Engen, FrontEndBuckets, FrontEndStartupBuckets,
    MouseAdapter, TouchAdapter, VirtualKeyboardAdapter, WindowResize,
};

pub struct WindowAttachment;

impl Attach for WindowAttachment {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .container
            .insert_resource(Events::<WindowResize>::default());
        engen
            .backend
            .container
            .insert_resource(Events::<WindowResize>::default());
        engen
            .frontend
            .container
            .insert_resource(Events::<ClickEvent>::default());
        engen.frontend.startup.add_system(
            orientation::setup_orientation.in_set(FrontEndStartupBuckets::Initialization),
        );
        engen
            .frontend
            .main
            .add_system(orientation::calc_orientation.in_set(FrontEndBuckets::Prepare));
        engen
            .frontend
            .main
            .add_system(Events::<WindowResize>::update_system.in_set(FrontEndBuckets::First));
        engen
            .frontend
            .main
            .add_system(Events::<ClickEvent>::update_system.in_set(FrontEndBuckets::First));
        engen
            .backend
            .main
            .add_system(Events::<WindowResize>::update_system.in_set(BackendBuckets::Initialize));
        engen
            .frontend
            .container
            .insert_resource(TouchAdapter::new());
        engen
            .frontend
            .container
            .insert_resource(MouseAdapter::new());
        engen
            .frontend
            .container
            .insert_resource(VirtualKeyboardAdapter::new());
    }
}
