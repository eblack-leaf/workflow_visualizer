use bevy_ecs::prelude::IntoSystemConfig;

use crate::engen::{Attach, Engen};
use crate::engen::{
    BackEndStartupBuckets, BackendBuckets, FrontEndBuckets, FrontEndStartupBuckets,
};
use crate::icon::backend_system::{process_differences, read_add_requests, setup};
use crate::icon::frontend_system::{
    area_cache_check, calc_area, color_cache_check, color_invert_cache_check, depth_cache_check,
    frontend_setup, icon_key_cache_check, initialization, position_cache_check,
    secondary_color_cache_check,
};
use crate::icon::IconRenderer;
use crate::{gfx, spawn, IconBundle};

pub struct IconAttachment;

impl Attach for IconAttachment {
    fn attach(engen: &mut Engen) {
        engen.add_renderer::<IconRenderer>();
        engen.backend.startup.add_system(
            setup
                .after(gfx::viewport_attach)
                .in_set(BackEndStartupBuckets::Prepare),
        );
        engen
            .backend
            .main
            .add_system(read_add_requests.in_set(BackendBuckets::Prepare));
        engen
            .backend
            .main
            .add_system(process_differences.after(read_add_requests));
        engen
            .frontend
            .startup
            .add_system(frontend_setup.in_set(FrontEndStartupBuckets::Startup));
        engen
            .frontend
            .main
            .add_system(spawn::<IconBundle>.in_set(FrontEndBuckets::Spawn));
        engen
            .frontend
            .main
            .add_system(calc_area.in_set(FrontEndBuckets::Resolve));
        engen
            .frontend
            .main
            .add_system(initialization.in_set(FrontEndBuckets::PushDiffs));
        engen
            .frontend
            .main
            .add_system(position_cache_check.in_set(FrontEndBuckets::PushDiffs));
        engen
            .frontend
            .main
            .add_system(color_invert_cache_check.in_set(FrontEndBuckets::PushDiffs));
        engen
            .frontend
            .main
            .add_system(area_cache_check.in_set(FrontEndBuckets::PushDiffs));
        engen
            .frontend
            .main
            .add_system(depth_cache_check.in_set(FrontEndBuckets::PushDiffs));
        engen
            .frontend
            .main
            .add_system(color_cache_check.in_set(FrontEndBuckets::PushDiffs));
        engen
            .frontend
            .main
            .add_system(secondary_color_cache_check.in_set(FrontEndBuckets::PushDiffs));
        engen
            .frontend
            .main
            .add_system(icon_key_cache_check.in_set(FrontEndBuckets::PushDiffs));
    }
}
