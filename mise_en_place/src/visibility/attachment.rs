use bevy_ecs::prelude::IntoSystemConfig;

use crate::engen::{Attach, Engen};
use crate::engen::{BackendBuckets, FrontEndBuckets};
use crate::gfx::GfxSurfaceConfiguration;
use crate::visibility::spacial_hasher::{update_spacial_hash, SpacialHasher};
use crate::visibility::system::{calc_visible_section, update_visible_bounds};
use crate::visibility::visible_bounds::adjust_position;
use crate::visibility::{
    collision, system, visible_bounds, ViewportOffsetUpdate, VisibleBoundsPositionAdjust,
};
use crate::window::ScaleFactor;
use crate::{Area, DeviceView, VisibleBounds};

pub struct VisibilityAttachment;

impl Attach for VisibilityAttachment {
    fn attach(engen: &mut Engen) {
        engen
            .backend
            .container
            .insert_resource(ViewportOffsetUpdate::new());
        engen.add_extraction::<VisibleBounds>();
        let gfx_surface_configuration = engen
            .backend
            .container
            .get_resource::<GfxSurfaceConfiguration>()
            .expect("no gfx surface config");
        let scale_factor = engen
            .frontend
            .container
            .get_resource::<ScaleFactor>()
            .expect("no scale factor")
            .factor;
        let surface_area: Area<DeviceView> = (
            gfx_surface_configuration.configuration.width,
            gfx_surface_configuration.configuration.height,
        )
            .into();
        let visible_section = ((0u32, 0u32), surface_area.to_ui(scale_factor)).into();
        engen
            .frontend
            .container
            .insert_resource(VisibleBounds::new(visible_section));
        engen
            .frontend
            .container
            .insert_resource(SpacialHasher::new(500f32, visible_section));
        engen
            .frontend
            .container
            .insert_resource(VisibleBoundsPositionAdjust::new());
        engen
            .backend
            .main
            .add_system(visible_bounds::viewport_read_offset.in_set(BackendBuckets::Resize));
        engen.frontend.main.add_system(update_visible_bounds);
        engen.frontend.main.add_system(
            calc_visible_section
                .in_set(FrontEndBuckets::Resize)
                .after(update_visible_bounds),
        );
        engen
            .frontend
            .main
            .add_system(system::visibility_setup.in_set(FrontEndBuckets::VisibilityPreparation));
        engen
            .frontend
            .main
            .add_system(system::visibility_cleanup.in_set(FrontEndBuckets::VisibilityPreparation));
        engen
            .frontend
            .main
            .add_system(visible_bounds::adjust_position.in_set(FrontEndBuckets::ResolveVisibility));
        engen.frontend.main.add_system(
            update_spacial_hash
                .in_set(FrontEndBuckets::ResolveVisibility)
                .after(adjust_position),
        );
        engen.frontend.main.add_system(
            collision::collision_responses
                .in_set(FrontEndBuckets::ResolveVisibility)
                .after(update_spacial_hash),
        );
        engen.frontend.main.add_system(
            calc_visible_section
                .in_set(FrontEndBuckets::ResolveVisibility)
                .after(update_spacial_hash),
        );
        engen
            .frontend
            .main
            .add_system(collision::clean_collision_responses.in_set(FrontEndBuckets::Last));
    }
}
