use mise_en_place::{Cook, DeliveryTicket, Recipe, Stove};

struct Meal;

impl Cook for Meal {
    fn recipe(recipe: &mut Recipe) {}
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"web".to_string()) {
        let delivery_ticket =
            DeliveryTicket::new("mise_en_place_app", "debug", "mise_en_place_app_web_build");
        let delivery_service =
            Stove::order_delivery(delivery_ticket).expect("could not compile to wasm");
        #[cfg(not(target_arch = "wasm32"))]
        {
            if args.contains(&"serve".to_string()) {
                delivery_service.deliver_to(([0, 0, 0, 0], 3030));
            }
        }
    }
    let mut stove = Stove::new();
    // stove.add_ingredient::<()>();
    stove.cook::<Meal>();
}
