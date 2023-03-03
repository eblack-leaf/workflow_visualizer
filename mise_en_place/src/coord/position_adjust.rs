use std::marker::PhantomData;

use bevy_ecs::prelude::{Commands, Component, Entity, Query, Res};

use crate::{animate, Attach, Engen, FrontEndStages, UIView};
use crate::animate::{Animate, Animation, Interpolator};
use crate::coord::{CoordContext, Position};
use crate::time::{TimeDelta, Timer};

#[derive(Component, Copy, Clone, Default, PartialEq, Debug)]
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
    pub x_interpolator: Interpolator,
    pub y_interpolator: Interpolator,
}

impl PositionAdjustAnimator {
    pub fn new(total_adjust: PositionAdjust<UIView>) -> Self {
        Self {
            x_interpolator: Interpolator::new(total_adjust.x),
            y_interpolator: Interpolator::new(total_adjust.y),
        }
    }
}

pub(crate) fn animate_position_adjust(
    mut animators: Query<(
        Entity,
        &Position<UIView>,
        &mut Animation<PositionAdjustAnimator>,
    )>,
    mut cmd: Commands,
    timer: Res<Timer>,
) {
    for (entity, pos, mut animation) in animators.iter_mut() {
        let (delta, anim_done) = animation.calc_delta_factor(&timer);
        if anim_done {
            println!("ending anim: {:.2}", timer.mark().0);
        }
        let (x_change, x_done) = animation.animator.x_interpolator.extract(delta);
        let (y_change, y_done) = animation.animator.y_interpolator.extract(delta);
        let mut position_change = PositionAdjust::<UIView>::new(x_change, y_change);
        if anim_done || (x_done && y_done) {
            println!("removing at: {:.2}", timer.mark().0);
            cmd.entity(entity)
                .remove::<Animation<PositionAdjustAnimator>>();
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
