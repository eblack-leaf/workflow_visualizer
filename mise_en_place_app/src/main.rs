use mise_en_place::{
    Cook, DeliveryTicket, Position, Recipe, Stove, Text, TextBundle, TextRenderer,
    TextScaleAlignment,
};

struct Meal;

impl Cook for Meal {
    fn prepare(recipe: &mut Recipe) {
        recipe.container.spawn(TextBundle::new(
            "stove
    .frontend
    .startup
    .add_system_to_stage(FrontEndStartupStages::Startup, frontend_setup);
stove.frontend.main.add_stage_before(
    FrontEndStages::VisibilityPreparation,
    TextStages::CalcTextScale,
    SystemStage::single(calc_scale_from_alignment),
);
stove.frontend.main.add_stage_after(
    TextStages::CalcTextScale,
    TextStages::CalcArea,
    SystemStage::single(calc_area),
);
stove.frontend.main.add_stage_after(
    FrontEndStages::ResolveVisibility,
    TextStages::TextFrontEnd,
    SystemStage::parallel(),
);
",
            (10u32, 10u32),
            0u32,
            (1.0, 1.0, 1.0),
            TextScaleAlignment::Small,
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
            return;
        }
    }
    let mut stove = Stove::new();
    stove.add_ingredient::<TextRenderer>();
    stove.cook::<Meal>();
}
