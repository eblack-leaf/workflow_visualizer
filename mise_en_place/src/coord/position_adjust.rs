use std::marker::PhantomData;

use bevy_ecs::prelude::{Added, Commands, Component, Entity, ParamSet, Query, Res};

use crate::{animate, Attach, Engen, FrontEndStages, UIView};
use crate::animate::{Animate, Animation};
use crate::coord::{CoordContext, Position};
use crate::time::{TimeDelta, Timer};

#[derive(Component, Copy, Clone, Default, PartialEq)]
pub struct PositionAdjust<Context: CoordContext> {
    pub x: f32,
    pub y: f32,
    _context: PhantomData<Context>,
}

impl<Context: CoordContext> PositionAdjust<Context> {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            _context: PhantomData,
        }
    }
}

impl<Context: CoordContext> From<(f32, f32)> for PositionAdjust<Context> {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

pub(crate) fn position_adjust<Context: CoordContext>(
    mut adjusted: Query<(Entity, &mut Position<Context>, &PositionAdjust<Context>), ()>,
    mut cmd: Commands,
) {
    for (entity, mut position, position_adjust) in adjusted.iter_mut() {
        position.adjust(*position_adjust);
        cmd.entity(entity).remove::<PositionAdjust<Context>>();
    }
}

// IMPL TEST
#[derive(Component)]
pub struct PositionAdjustAnimator {
    pub total_adjust: PositionAdjust<UIView>,
    pub end: Position<UIView>,
}

impl PositionAdjustAnimator {
    pub fn new(total_adjust: PositionAdjust<UIView>) -> Self {
        Self {
            total_adjust,
            end: Position::default(),
        }
    }
}

pub(crate) fn animate_position_adjust(
    mut animators: ParamSet<(
        Query<
            (
                &Position<UIView>,
                &mut Animation<PositionAdjustAnimator>,
            ),
            Added<Animation<PositionAdjustAnimator>>,
        >,
        Query<(
            Entity,
            &Position<UIView>,
            &mut Animation<PositionAdjustAnimator>,
        )>,
    )>,
    mut cmd: Commands,
    timer: Res<Timer>,
) {
    for (pos, mut animation) in animators.p0().iter_mut() {
        animation.animator.end = pos.adjusted(animation.animator.total_adjust);
    }
    for (entity, pos, mut animation) in animators.p1().iter_mut() {
        let (delta, done) = animation.calc_delta_factor(&timer);
        let x_change = animation.animator.total_adjust.x * delta;
        let y_change = animation.animator.total_adjust.y * delta;
        let mut position_change = PositionAdjust::<UIView>::new(x_change, y_change);
        if done {
            cmd.entity(entity)
                .remove::<Animation<PositionAdjustAnimator>>();
            if pos.x + x_change != animation.animator.end.x {
                // TODO semantics review necessary
                position_change.x = pos.x + animation.animator.end.x - pos.x;
            }
            // same for y
        }
        cmd.entity(entity).insert(position_change);
    }
}

impl Animate for PositionAdjust<UIView> {
    type Animator = PositionAdjustAnimator;
    fn animate<T: Into<TimeDelta>>(self, total_time: T) -> Animation<Self::Animator> {
        Animation::new(total_time.into(), PositionAdjustAnimator::new(self))
    }
}

impl Attach for PositionAdjustAnimator {
    fn attach(engen: &mut Engen) {
        engen.frontend.main.add_system_to_stage(FrontEndStages::AnimationStart, animate::start_animations::<PositionAdjustAnimator>);
        engen.frontend.main.add_system_to_stage(FrontEndStages::AnimationUpdate, animate_position_adjust);
    }
}
