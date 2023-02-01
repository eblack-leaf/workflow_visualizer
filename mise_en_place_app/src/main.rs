use mise_en_place::{Stove, RecipeDirections, Cook, DeliveryTicket};

struct Recipe;
impl Cook for Recipe {
    fn recipe(recipe_directions: &mut RecipeDirections) {}
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"web".to_string()) {
        let delivery_ticket =
            DeliveryTicket::new("mise_en_place_app", "debug", "mise_en_place_app_web_build");
        let delivery_service = Stove::order_delivery(delivery_ticket).expect("could not compile to wasm");
        #[cfg(not(target_arch = "wasm32"))]{
            if args.contains(&"serve".to_string()) {
                delivery_service.deliver_to(([0, 0, 0, 0], 3030));
            }
        }
    }
    let stove = Stove::new();
    stove.cook::<Recipe>();
}
