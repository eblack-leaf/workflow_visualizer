use bevy_ecs::prelude::{Query, ResMut, Resource};

use mise_en_place::{
    Cook, DeliveryTicket, FrontEndStages, Position, Recipe, Stove, Text, TextBundle, TextRenderer,
    TextScaleAlignment,
};

#[derive(Resource)]
struct Counter {
    count: u32,
}

fn update_text(mut text: Query<&mut Text>, mut counter: ResMut<Counter>) {
    counter.count += 1;
    for mut ent_text in text.iter_mut() {
        ent_text.string = format!("counter is: {}", counter.count);
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
            Text::new("counter is: 0123456789"),
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
