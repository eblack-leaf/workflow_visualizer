use crate::task::Stage;
use crate::text::{compute, extract, grow, rasterization, render, Renderer};
use crate::{text, Attach, Engen};
use bevy_ecs::prelude::*;

impl Attach for Renderer {
    fn attach(engen: &mut Engen) {
        let compute_startup = &mut engen.compute.startup.schedule;
        compute_startup.add_system_to_stage(Stage::Before, compute::compute_setup);
        let compute_main = &mut engen.compute.main.schedule;
        compute_main.add_system_to_stage(Stage::After, compute::text_entity_changes);
        compute_main.add_system_to_stage(Stage::After, compute::visibility);
        compute_main.add_system_to_stage(Stage::Last, compute::push_compute_changes);
        engen
            .render
            .startup
            .schedule
            .add_system_to_stage(Stage::Before, render::render_setup);
        let engen_main = &mut engen.render.main.schedule;
        engen_main.add_system_to_stage(Stage::First, rasterization::add_remove_rasterizations);
        engen_main.add_system_to_stage(Stage::Before, grow::grow);
        engen_main.add_system_to_stage(
            Stage::During,
            rasterization::rasterize
                .label("rasterization")
                .before("integration"),
        );
        engen_main.add_system_to_stage(
            Stage::During,
            extract::integrate_extraction.label("integration"),
        );
        engen_main.add_system_to_stage(Stage::Last, extract::reset_extraction);
    }
}
