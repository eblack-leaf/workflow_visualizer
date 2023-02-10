use std::ops::Add;

use bevy_ecs::prelude::{Commands, Entity, Query, ResMut, Resource};

use mise_en_place::{
    Color, Cook, DeliveryTicket, DepthAdjust, Exit, FrontEndStages, Idle, PartitionMetadata,
    PositionAdjust, Recipe, Stove, Text, TextBoundGuide, TextBundle, TextOffsetAdjustGuide,
    TextPartition, TextRenderer, TextScaleAlignment, Visibility,
};

#[derive(Resource)]
struct Counter {
    count: u32,
}

fn update_text(
    mut text: Query<(Entity, &mut Text, &Visibility)>,
    mut counter: ResMut<Counter>,
    mut _idle: ResMut<Idle>,
    mut cmd: Commands,
) {
    counter.count += 1;
    for (entity, mut ent_text, visibility) in text.iter_mut() {
        if counter.count == 100 {
            ent_text.partitions.push(TextPartition::new(
                " ok then",
                PartitionMetadata::new((0.5, 1.0, 0.5), 0),
            ));
        }
        if counter.count == 200 {
            ent_text.partitions.pop();
        }
    }
}

struct Meal;

impl Cook for Meal {
    fn prepare(recipe: &mut Recipe) {
        recipe.container.insert_resource(Counter { count: 0 });
        recipe
            .main
            .add_system_to_stage(FrontEndStages::Process, update_text);
        recipe
            .container
            .spawn(TextBundle::new(
                Text::new(vec![TextPartition::new(
                    "hello",
                    PartitionMetadata::new((1.0, 1.0, 1.0), 0),
                )]),
                (0u32, 0u32),
                10u32,
                TextScaleAlignment::Medium,
            ))
            .insert(TextBoundGuide::new(120, 112));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"web".to_string()) {
        let delivery_ticket =
            DeliveryTicket::new("mise_en_place_app", "debug", "mise_en_place_app_web_build");
        let delivery_service =
            Stove::order_delivery(delivery_ticket).expect("could not compile to wasm");
        #[cfg(not(target_arch = "wasm32"))]
        if args.contains(&"serve".to_string()) {
            delivery_service.deliver_to(([0, 0, 0, 0], 3030));
        } else {
            println!("order only placed, deliver at your leisure");
            return;
        }
    }
    let mut stove = Stove::new();
    stove.add_ingredient::<TextRenderer>();
    stove.cook::<Meal>();
}
