#![allow(unused, dead_code)]

use mise_en_place::{Animate, Animation, Attachment, Color, Engen, EngenOptions, EntityStore, FrontEndStages, IconAttachment, Idle, Job, Launch, LetterStyle, Location, Position, PositionAdjust, PositionAdjustAnimator, Request, Text, TextAttachment, TextBundle, TextGridGuide, TextInputRequest, TextInputText, TextLine, TextScaleAlignment, Timer, UIView, VisibleSection};
use mise_en_place::bevy_ecs::prelude::{Added, Entity, Query, RemovedComponents, Res, ResMut};

#[cfg(not(target_arch = "wasm32"))]
mod serve;

struct Launcher;

fn logic(
    mut idle: ResMut<Idle>,
    entity_store: Res<EntityStore>,
    mut text_query: Query<(&mut Text, &Position<UIView>)>,
    timer: Res<Timer>,
    text_input: Query<(Entity, &TextInputText, &VisibleSection)>,
) {
    idle.can_idle = false;
    let text_entity = *entity_store.store.get("animated_text").unwrap();
    if let Ok((mut text, pos)) = text_query.get_mut(text_entity) {
        *text.lines.first_mut().unwrap() = TextLine::from((
            format!("text pos at: {:.2}, {:.2}", pos.x, pos.y),
            Color::OFF_WHITE,
            LetterStyle::REGULAR,
        ));
    }
    let text_entity = *entity_store.store.get("timer_text").unwrap();
    if let Ok((mut text, pos)) = text_query.get_mut(text_entity) {
        *text.lines.first_mut().unwrap() = TextLine::from((
            format!("timer: {:.2}", timer.mark().0),
            Color::OFF_WHITE,
            LetterStyle::REGULAR,
        ));
    }
    for (entity, input_text, v_section) in text_input.iter() {
        let (mut text, pos) = text_query.get_mut(input_text.entity).unwrap();
        *text.lines.first_mut().unwrap() = TextLine::from((
            format!("visible section: {:?}", v_section.section()),
            Color::OFF_WHITE,
            LetterStyle::REGULAR,
        ));
    }
}

fn post_anim_logic(
    removed: RemovedComponents<Animation<PositionAdjustAnimator>>,
    entity_store: Res<EntityStore>,
    anim_start: Query<Entity, Added<Animation<PositionAdjustAnimator>>>,
    mut text_query: Query<(&mut Text, &Position<UIView>)>,
    timer: Res<Timer>,
) {
    for _added in anim_start.iter() {
        let text_entity = *entity_store.store.get("start_text").unwrap();
        let (mut text, pos) = text_query.get_mut(text_entity).unwrap();
        *text.lines.first_mut().unwrap() = TextLine::from((
            format!("start at: {:.2}", timer.mark().0),
            Color::OFF_WHITE,
            LetterStyle::REGULAR,
        ));
    }
    for _remove in removed.iter() {
        let text_entity = *entity_store.store.get("done_text").unwrap();
        let (mut text, pos) = text_query.get_mut(text_entity).unwrap();
        *text.lines.first_mut().unwrap() = TextLine::from((
            format!("done at: {:.2}", timer.mark().0),
            Color::OFF_WHITE,
            LetterStyle::REGULAR,
        ));
    }
}

impl Launch for Launcher {
    fn options() -> EngenOptions {
        EngenOptions::new().with_native_dimensions((1000, 900))
    }

    fn attachments() -> Vec<Attachment> {
        vec![]
    }

    fn prepare(job: &mut Job) {
        let id = job
            .container
            .spawn(Request::new(TextBundle::new(
                Text::new(vec![TextLine::from((
                    "animated_text".to_string(),
                    Color::OFF_WHITE,
                    LetterStyle::REGULAR,
                ))]),
                Location::new((0.0, 0.0), 0),
                TextScaleAlignment::Medium,
            )))
            .insert(PositionAdjust::<UIView>::new(212.30, 0.0).animate(1.75))
            .id();
        job.store_entity("animated_text", id);
        job.main.add_system_to_stage(FrontEndStages::Process, logic);
        job.main
            .add_system_to_stage(FrontEndStages::AnimationResolved, post_anim_logic);
        let id = job
            .container
            .spawn(Request::new(TextBundle::new(
                Text::new(vec![TextLine::from((
                    "timer:".to_string(),
                    Color::OFF_WHITE,
                    LetterStyle::REGULAR,
                ))]),
                Location::new((0.0, 40.0), 0),
                TextScaleAlignment::Medium,
            )))
            .id();
        job.store_entity("timer_text", id);
        let id = job
            .container
            .spawn(Request::new(TextBundle::new(
                Text::new(vec![TextLine::from((
                    "start at:".to_string(),
                    Color::OFF_WHITE,
                    LetterStyle::REGULAR,
                ))]),
                Location::new((0.0, 80.0), 0),
                TextScaleAlignment::Medium,
            )))
            .id();
        job.store_entity("start_text", id);
        let id = job
            .container
            .spawn(Request::new(TextBundle::new(
                Text::new(vec![TextLine::from((
                    "done at:".to_string(),
                    Color::OFF_WHITE,
                    LetterStyle::REGULAR,
                ))]),
                Location::new((0.0, 120.0), 0),
                TextScaleAlignment::Medium,
            )))
            .id();
        job.store_entity("done_text", id);
        let id = job.container.spawn(
            Request::new(
                TextInputRequest::new(
                    "".to_string(),
                    TextScaleAlignment::Medium,
                    TextGridGuide::new(50, 2),
                    Location::from(((0, 160), 0)),
                    Color::OFF_WHITE,
                )
            )
        ).id();
        job.store_entity("text input", id);
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        serve::compile_and_serve();
    }
    Engen::launch::<Launcher>();
}
