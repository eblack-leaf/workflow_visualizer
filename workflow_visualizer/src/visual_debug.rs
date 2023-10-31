use crate::{
    Area, Attach, Color, InterfaceContext, Layer, Line, Position, Section, SyncPoint, Visualizer,
};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::IntoSystemConfigs;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::{Commands, Query};

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct SectionOutline(Option<Entity>);
pub(crate) fn section_outline(
    mut query: Query<
        (
            &mut SectionOutline,
            &Position<InterfaceContext>,
            &Area<InterfaceContext>,
            &Layer,
        ),
        Changed<SectionOutline>,
    >,
    mut cmd: Commands,
) {
    for (mut outline, pos, area, layer) in query.iter_mut() {
        let section = Section::new(*pos, *area);
        let path = vec![
            Position::new(section.left(), section.top()),
            Position::new(section.left(), section.bottom()),
            Position::new(section.right(), section.bottom()),
            Position::new(section.right(), section.top()),
            Position::new(section.left(), section.top()),
        ];
        let line = Line::new(path, *layer - 1.into(), Color::OFF_WHITE);
        let id = cmd.spawn(line).id();
        outline.0.replace(id);
    }
}
pub(crate) fn changed_outline(
    query: Query<
        (
            &SectionOutline,
            &Position<InterfaceContext>,
            &Area<InterfaceContext>,
        ),
        Or<(
            Changed<Position<InterfaceContext>>,
            Changed<Area<InterfaceContext>>,
        )>,
    >,
) {
}
pub(crate) struct VisualDebugAttachment;
impl Attach for VisualDebugAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.task(Visualizer::TASK_MAIN).add_systems((
            section_outline.in_set(SyncPoint::PostSpawn),
            changed_outline.in_set(SyncPoint::Reconfigure),
        ));
    }
}
