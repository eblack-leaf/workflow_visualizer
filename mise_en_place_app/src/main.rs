#![allow(unused, dead_code)]

use std::ops::Add;

use bevy_ecs::prelude::{Commands, Entity, Query, ResMut, Resource};

use mise_en_place::{
    Color, DepthAdjust, Engen, EngenOptions, Exit, FrontEndStages, Idle, Job, Launch,
    PartitionMetadata, PositionAdjust, Text, TextBoundGuide, TextBundle, TextPartition,
    TextRenderer, TextScaleAlignment, Unscaled, Visibility, WasmCompileDescriptor, WasmServer,
};

#[derive(Resource)]
struct Counter {
    count: u32,
}

fn update_text(
    mut text: Query<(Entity, &mut Text, Option<&Visibility>)>,
    mut counter: ResMut<Counter>,
    mut _idle: ResMut<Idle>,
    mut cmd: Commands,
    mut exit: ResMut<Exit>,
) {
    counter.count += 1;
    _idle.can_idle = false;
    for (entity, mut ent_text, visibility) in text.iter_mut() {
        if counter.count > 0 {
            if counter.count % 1 == 0 {
                if entity.index() == 0 {
                    ent_text.partitions.first_mut().unwrap().characters =
                        format!("counter is: {:?}", counter.count);
                }
                if entity.index() == 1 {
                    cmd.entity(entity)
                        .insert(PositionAdjust::<Unscaled>::new(0.0, 1.0));
                }
            }
        }
    }
}

struct Launcher;

impl Launch for Launcher {
    fn prepare(job: &mut Job) {
        job.container.insert_resource(Counter { count: 0 });
        job.main
            .add_system_to_stage(FrontEndStages::Process, update_text);
        job.container
            .spawn(TextBundle::new(
                Text::new(vec![("", ((1.0, 1.0, 1.0), 0))]),
                (Unscaled {}, (0u32, 0u32), 0u32),
                TextScaleAlignment::Medium,
            ))
            .insert(TextBoundGuide::new(120, 1));
        job.container
            .spawn(TextBundle::new(
                Text::new(vec![("abcdefghijklmnopqrstuvwxyz", ((1.0, 1.0, 1.0), 0))]),
                (Unscaled {}, (0u32, 40u32), 10u32),
                TextScaleAlignment::Small,
            ))
            .insert(TextBoundGuide::new(120, 1));
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let args: Vec<String> = std::env::args().collect();
        let wasm_compile_descriptor = WasmCompileDescriptor::new(
            "mise_en_place_app",
            "release",
            "mise_en_place_app_web_build",
        );
        let wasm_server = WasmServer::new(&wasm_compile_descriptor);
        if args.contains(&"build".to_string()) {
            wasm_compile_descriptor
                .compile()
                .expect("could not compile wasm");
            if !args.contains(&"serve".to_string()) {
                return;
            }
        }
        if args.contains(&"serve".to_string()) {
            wasm_server.serve_at(([0, 0, 0, 0], 3030));
            return;
        }
    }
    let mut engen = Engen::new(EngenOptions::new().with_native_dimensions((500, 900)));
    engen.add_renderer::<TextRenderer>();
    engen.launch::<Launcher>();
}
