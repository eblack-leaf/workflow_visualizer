// pub(crate) fn manage<T: Send + Sync + 'static>(
//     mut animations: Query<(Entity, &mut Animation<T>), Changed<Animation<T>>>,
//     timer: Res<Timer>,
// ) {
//     for (entity, anim) in animations.iter_mut() {
//         if anim.start.is_none() {
//             anim.start.replace(timer.mark());
//             anim.start
//                 .unwrap()
//                 .offset(anim.start_offset.unwrap_or_default());
//         }
//         let mut time_since_start = timer.time_since(anim.start.unwrap());
//         if time_since_start.0.is_sign_positive() {
//             let animation_time_over = time_since_start >= anim.animation_time;
//             let mut anim_delta: TimeDelta = timer.frame_diff().0.min(time_since_start.0).into();
//             if animation_time_over {
//                 let past_total = time_since_start - anim.animation_time;
//                 anim_delta -= past_total;
//                 anim.overage.replace(past_total.as_f32());
//                 anim.animation_time_over = true;
//             }
//             let anim_delta = anim_delta / anim.animation_time;
//             let delta = anim_delta.0.min(1f64) as f32;
//             anim.delta.replace(delta);
//             if animation_time_over {
//                 anim.animator.finish()
//             } else {
//                 anim.animator.extract(delta)
//             }
//         }
//     }
// }
// pub type AnimationTagValue = u32;
// #[derive(Copy, Clone)]
// pub struct AnimationTag(pub AnimationTagValue);
// impl From<u32> for AnimationTag {
//     fn from(value: u32) -> Self {
//         AnimationTag(value)
//     }
// }
// pub struct AnimationDone<T> {
//     pub entity: Entity,
//     pub overage: Option<TimeDelta>,
//     pub tag: AnimationTag,
//     _phantom: PhantomData<T>,
// }
// impl<T> AnimationDone<T> {
//     pub fn new<TD: Into<TimeDelta>>(
//         entity: Entity,
//         overage: Option<TD>,
//         tag: AnimationTag,
//     ) -> Self {
//         Self {
//             entity,
//             overage: overage.and_then(|o| Option::from(o.into())),
//             tag,
//             _phantom: PhantomData,
//         }
//     }
// }
// pub(crate) fn end_animations<T: Send + Sync + 'static>(
//     mut animations: Query<(Entity, &mut Animation<T>)>,
//     mut event_sender: EventWriter<AnimationDone<T>>,
//     mut cmd: Commands,
// ) {
//     for (entity, mut anim) in animations.iter_mut() {
//         if anim.done() {
//             event_sender.send(AnimationDone::<T>::new(entity, anim.overage(), anim.tag));
//             cmd.entity(entity).remove::<Animation<T>>();
//         }
//     }
// }
// #[derive(Clone)]
// pub struct Animator(pub Vec<Interpolator>);
// impl Animator {
//     pub fn all_finished(&self) -> bool {
//         let mut finished = true;
//         for inter in self.0.iter() {
//             if !inter.done() {
//                 finished = false;
//             }
//         }
//         finished
//     }
//     pub fn extract(&mut self, delta: f32) {
//         for inter in self.0.iter_mut() {
//             inter.extract(delta);
//         }
//     }
//     pub fn finish(&mut self) {
//         for inter in self.0.iter_mut() {
//             inter.finish();
//         }
//     }
// }
// #[derive(Clone)]
// pub struct Animation<T> {
//     pub(crate) start: Option<TimeMarker>,
//     pub(crate) animation_time: TimeDelta,
//     pub animator: Animator,
//     pub(crate) delta: Option<f32>,
//     pub(crate) overage: Option<f32>,
//     pub tag: AnimationTag,
//     pub(crate) animation_time_over: bool,
//     pub(crate) start_offset: Option<f32>,
//     _phantom: PhantomData<T>,
// }
//
// impl<T> Animation<T> {
//     pub(crate) fn new<TD: Into<TimeDelta>, AT: Into<AnimationTag>, A: Into<Animator>>(
//         anim_time: TD,
//         animator: A,
//         anim_tag: AT,
//         start_offset: Option<f32>,
//     ) -> Self {
//         Self {
//             start: None,
//             animation_time: anim_time.into(),
//             animator,
//             delta: None,
//             overage: None,
//             tag: anim_tag.into(),
//             animation_time_over: false,
//             start_offset,
//             _phantom: PhantomData,
//         }
//     }
//     pub fn done(&self) -> bool {
//         self.animation_time_over && self.animator.all_finished()
//     }
//     pub fn delta(&self) -> Option<f32> {
//         self.delta
//     }
//     pub fn overage(&self) -> Option<f32> {
//         self.overage
//     }
// }
#[derive(Clone, Copy)]
pub struct InterpolationExtraction(pub f32, pub bool);

impl InterpolationExtraction {
    pub fn done(&self) -> bool {
        self.1
    }
    pub fn amount(&self) -> f32 {
        self.0
    }
}
/// Interpolates a value over an interval
#[derive(Copy, Clone)]
pub struct Interpolator {
    pub value: f32,
    total: f32,
    sign_positive: bool,
    extraction: Option<InterpolationExtraction>,
    done: bool,
}

impl Interpolator {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            total: value,
            sign_positive: value.is_sign_positive(),
            extraction: None,
            done: false,
        }
    }
    pub fn extract(&mut self, delta: f32) -> InterpolationExtraction {
        if !self.done {
            let segment = self.total * delta;
            self.value -= segment;
            let overage = match self.sign_positive {
                true => {
                    let mut val = None;
                    if self.value.is_sign_negative() {
                        val = Some(self.value)
                    }
                    val
                }
                false => {
                    let mut val = None;
                    if self.value.is_sign_positive() {
                        val = Some(self.value)
                    }
                    val
                }
            };
            let mut extract = segment;
            let mut done = false;
            if let Some(over) = overage {
                extract += over;
                done = true;
            }
            if extract == 0.0 {
                done = true;
            }
            let extract = InterpolationExtraction(extract, done);
            self.extraction.replace(extract);
            self.done = done;
            return extract;
        }
        InterpolationExtraction(0.0, false)
    }
    pub fn done(&self) -> bool {
        self.done
    }
    pub fn finish(&mut self) {
        let extract = InterpolationExtraction(self.value, true);
        self.value = 0f32;
        self.extraction.replace(extract);
        self.done = true;
    }
}

#[cfg(test)]
#[test]
pub(crate) fn interpolator_test() {
    let mut interpolator = Interpolator::new(1f32);
    let extraction = interpolator.extract(0.25);
    assert_eq!(extraction.0, 0.25f32);
}
