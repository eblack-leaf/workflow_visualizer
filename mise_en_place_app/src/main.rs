use bevy_ecs::prelude::{Commands, Entity, Query, ResMut, Resource};

use mise_en_place::{Cook, DeliveryTicket, DepthAdjust, Exit, FrontEndStages, Idle, PositionAdjust, Recipe, Stove, Text, TextBound, TextBundle, TextRenderer, TextScaleAlignment, Visibility};

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
        ent_text.string = format!("counter is: {} and there is a stress test for this \nto update and still be present", counter.count);
        if counter.count >= 200 && counter.count < 400 {
            cmd.entity(entity).insert(PositionAdjust::new(2.0, 1.0));
        }
        if counter.count >= 400 && counter.count < 600 {
            cmd.entity(entity).insert(PositionAdjust::new(-2.0, -1.0));
        }
        if counter.count == 100 {
            // bound section is not receiving position updates
            // visible section is not being updated cause not resized
            // make render group text_bound to area only and change based on position
            cmd.entity(entity).insert(TextBound::new((400, 300)));
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
        recipe.container.spawn(TextBundle::new(
            Text::new("counter is: 00"),
            (10u32, 10u32),
            0u32,
            (1.0, 1.0, 1.0),
            TextScaleAlignment::Medium,
        ));
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
