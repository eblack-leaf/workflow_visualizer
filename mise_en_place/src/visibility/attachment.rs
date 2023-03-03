use bevy_ecs::prelude::{IntoSystemDescriptor, SystemLabel};

use crate::engen::{Attach, Engen, FrontEndSystems};
use crate::engen::{BackendStages, FrontEndStages};
use crate::gfx::GfxSurfaceConfiguration;
use crate::visibility::spacial_hasher::SpacialHasher;
use crate::visibility::system::calc_visible_section;
use crate::visibility::{
    collision, spacial_hasher, system, visible_bounds, ViewportOffsetUpdate,
    VisibleBoundsPositionAdjust,
};
use crate::{Area, DeviceView, ScaleFactor, VisibleBounds};

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
            .add_system_to_stage(BackendStages::Resize, visible_bounds::viewport_read_offset);
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Resize,
            system::resize.label(FrontEndSystems::UpdateVisibleBounds),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::Resize,
            calc_visible_section.after(FrontEndSystems::UpdateVisibleBounds),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::VisibilityPreparation,
            system::visibility_setup,
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::VisibilityPreparation,
            system::visibility_cleanup,
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::ResolveVisibility,
            visible_bounds::adjust_position.label(VisibilitySystems::AdjustPosition),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::ResolveVisibility,
            spacial_hasher::update_spacial_hash
                .label(VisibilitySystems::UpdateSpacialHash)
                .after(VisibilitySystems::AdjustPosition),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::ResolveVisibility,
            collision::collision_responses.after(VisibilitySystems::UpdateSpacialHash),
        );
        engen.frontend.main.add_system_to_stage(
            FrontEndStages::ResolveVisibility,
            system::calc_visible_section.after(VisibilitySystems::UpdateSpacialHash),
        );
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Last, collision::clean_collision_responses);
    }
}

#[derive(SystemLabel)]
enum VisibilitySystems {
    AdjustPosition,
    UpdateSpacialHash,
}
