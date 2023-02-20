use bevy_ecs::prelude::{Component, Entity, EventReader, Query, Res};

use crate::{
    Area, Attach, ClickEvent, ClickEventType, Engen, FrontEndStages, Position, ScaleFactor, UIView,
    Visibility, VisibleSection,
};

#[derive(Component)]
pub struct Clickable {
    pub clicked: bool,
}

#[derive(Component)]
pub struct ClickListener {
    pub ty: ClickEventType,
}

pub(crate) fn register_click(
    mut clickables: Query<(
        Entity,
        &mut Clickable,
        &Position<UIView>,
        &Area<UIView>,
        &ClickListener,
        &Visibility,
        &VisibleSection,
    )>,
    mut clicks: EventReader<ClickEvent>,
    scale_factor: Res<ScaleFactor>,
) {
    for (entity, mut clickable, position, area, listener, visibility, visible_section) in
        clickables.iter_mut()
    {
        if visibility.visible() {
            for click in clicks.iter() {
                if listener.ty == click.ty {
                    match click.ty {
                        ClickEventType::OnPress => {
                            if visible_section
                                .section
                                .contains(click.click.origin.to_ui(scale_factor.factor))
                            {
                                clickable.clicked = true;
                            }
                        }
                        ClickEventType::OnMove => {}
                        ClickEventType::OnRelease => {
                            if visible_section
                                .section
                                .contains(click.click.origin.to_ui(scale_factor.factor))
                                && visible_section
                                    .section
                                    .contains(click.click.end.unwrap().to_ui(scale_factor.factor))
                            {
                                clickable.clicked = true;
                            }
                        }
                        ClickEventType::Cancelled => {}
                    }
                }
            }
        }
    }
}

pub(crate) fn reset_click(mut clickables: Query<(&mut Clickable)>) {
    for mut clickable in clickables.iter_mut() {
        clickable.clicked = false;
    }
}

pub struct ClickablePlugin;

impl Attach for ClickablePlugin {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::PreProcess, register_click);
        engen
            .frontend
            .main
            .add_system_to_stage(FrontEndStages::Last, reset_click);
    }
}
