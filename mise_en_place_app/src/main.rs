use std::ops::Add;

use bevy_ecs::prelude::{Commands, Entity, Query, ResMut, Resource};

use mise_en_place::{Color, DepthAdjust, Engen, Exit, FrontEndStages, Idle, Job, Launch, PartitionMetadata, PositionAdjust, Text, TextBoundGuide, TextBundle, TextOffsetAdjustGuide, TextPartition, TextRenderer, TextScaleAlignment, Visibility, WasmCompileDescriptor, WasmServer};

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
    _idle.can_idle = false;
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
        if counter.count >= 250 {
            _idle.can_idle = true;
        }
    }
}

struct Launcher;

impl Launch for Launcher {
    fn prepare(job: &mut Job) {
        job.container.insert_resource(Counter { count: 0 });
        job
            .main
            .add_system_to_stage(FrontEndStages::Process, update_text);
        job
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
    #[cfg(not(target_arch = "wasm32"))] {
        let args: Vec<String> = std::env::args().collect();
        let wasm_compile_descriptor =
            WasmCompileDescriptor::new("mise_en_place_app", "release", "mise_en_place_app_web_build");
        let wasm_server = WasmServer::new(&wasm_compile_descriptor);
        if args.contains(&"build".to_string()) {
            wasm_compile_descriptor.compile().expect("could not compile wasm");
            if !args.contains(&"serve".to_string()) {
                return;
            }
        }
        if args.contains(&"serve".to_string()) {
            wasm_server.serve_at(([0, 0, 0, 0], 3030));
            return;
        }
    }
    let mut engen = Engen::new();
    engen.add_renderer::<TextRenderer>();
    engen.launch::<Launcher>();
}
