#![allow(unused, dead_code)]

use mise_en_place::{Animate, Animation, Attachment, Color, Engen, EngenOptions, EntityStore, FrontEndStages, IconAttachment, Job, Launch, Location, Position, PositionAdjust, PositionAdjustAnimator, Text, TextAttachment, TextBundle, TextPartition, TextScaleAlignment, Timer, UIView};
use mise_en_place::bevy_ecs::prelude::{Query, RemovedComponents, Res};

#[cfg(not(target_arch = "wasm32"))]
mod server;

struct Launcher;

fn logic(entity_store: Res<EntityStore>, mut text_query: Query<(&mut Text, &Position<UIView>)>, timer: Res<Timer>, removed: RemovedComponents<Animation<PositionAdjustAnimator>>) {
    let text_entity = *entity_store.store.get("animated_text").unwrap();
    let (mut text, pos) = text_query.get_mut(text_entity).unwrap();
    text.partitions.first_mut().unwrap().characters = format!("migrating pos: {:.2}, {:.2}", pos.x, pos.y);
    for _ in removed.iter() {
        let text_entity = *entity_store.store.get("done_text").unwrap();
        let (mut text, pos) = text_query.get_mut(text_entity).unwrap();
        text.partitions.first_mut().unwrap().characters = format!("done at: {:.2}", timer.mark().0);
    }
    let text_entity = *entity_store.store.get("timer_text").unwrap();
    let (mut text, pos) = text_query.get_mut(text_entity).unwrap();
    text.partitions.first_mut().unwrap().characters = format!("timer: {:.2}", timer.mark().0);
}

impl Launch for Launcher {
    fn options() -> EngenOptions {
        EngenOptions::new().with_native_dimensions((500, 900))
    }

    fn attachments() -> Vec<Attachment> {
        vec![
            Attachment::using::<TextAttachment>(),
            Attachment::using::<IconAttachment>(),
            Attachment::using::<PositionAdjustAnimator>(),
        ]
    }

    fn prepare(job: &mut Job) {
        let id = job.container.spawn(
            TextBundle::new(
                Text::new(
                    vec![TextPartition::new(
                        "animated text", (Color::OFF_WHITE, 0),
                    )]
                ),
                Location::new((0.0, 0.0), 0),
                TextScaleAlignment::Medium,
            )
        ).insert(PositionAdjust::<UIView>::new(200.0, 0.0).animate(2.0)).id();
        job.store_entity("animated_text", id);
        job.main.add_system_to_stage(FrontEndStages::ProcessAndSpawn, logic);
        let id = job.container.spawn(
            TextBundle::new(
                Text::new(
                    vec![TextPartition::new(
                        "timer:", (Color::OFF_WHITE, 0),
                    )]
                ),
                Location::new((0.0, 40.0), 0),
                TextScaleAlignment::Medium,
            )
        ).id();
        job.store_entity("timer_text", id);
        let id = job.container.spawn(
            TextBundle::new(
                Text::new(
                    vec![TextPartition::new(
                        "done at:", (Color::OFF_WHITE, 0),
                    )]
                ),
                Location::new((0.0, 80.0), 0),
                TextScaleAlignment::Medium,
            )
        ).id();
        job.store_entity("done_text", id);
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        server::compile_and_serve();
    }
    Engen::launch::<Launcher>();
}
