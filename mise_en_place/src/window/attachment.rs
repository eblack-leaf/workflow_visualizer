use bevy_ecs::event::Events;

use crate::window::orientation;
use crate::{
    Attach, BackendStages, ClickEvent, Engen, FrontEndStages, FrontEndStartupStages, MouseAdapter,
    TouchAdapter, VirtualKeyboardAdapter, WindowResize,
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
        engen.frontend.startup.add_system_to_stage(
            FrontEndStartupStages::Initialization,
            orientation::setup_orientation,
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Prepare, orientation::calc_orientation);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::First, Events::<WindowResize>::update_system);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::First, Events::<ClickEvent>::update_system);
        engen.backend.main.add_system_to_stage(
            BackendStages::Initialize,
            Events::<WindowResize>::update_system,
        );
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
