#![allow(unused, dead_code)]

use std::ops::Add;

use bevy_ecs::prelude::{Commands, Entity, Query, Res, ResMut, Resource};

use mise_en_place::{Color, DepthAdjust, Engen, EngenOptions, Exit, FrontEndStages, Idle, Job, Launch, MotionAdapter, MouseAdapter, PartitionMetadata, PositionAdjust, Text, TextBoundGuide, TextBundle, TextPartition, TextRenderer, TextScaleAlignment, TouchAdapter, View, Visibility, WasmCompileDescriptor, WasmServer};

#[derive(Resource)]
struct Counter {
    count: u32,
}

fn update_text(
    mut text: Query<(Entity, &mut Text)>,
    mut counter: ResMut<Counter>,
    mut _idle: ResMut<Idle>,
    mut cmd: Commands,
    mut exit: ResMut<Exit>,
    touch_adapter: Res<TouchAdapter>,
    mouse_adapter: Res<MouseAdapter>,
    motion_adapter: Res<MotionAdapter>,
) {
    counter.count += 1;
    _idle.can_idle = false;
    for (entity, mut ent_text) in text.iter_mut() {
        if let Some(touch) = touch_adapter.current_touch {
            ent_text.partitions.first_mut().unwrap().characters =
                format!("touch location: {:?}", touch.location);
        }
        if let Some(location) = mouse_adapter.location {
            ent_text.partitions.first_mut().unwrap().characters =
                format!("mouse location: {:?}", location);
        }
        for (button, state) in mouse_adapter.button_state.iter() {
            ent_text.partitions.first_mut().unwrap().characters = format!("button: {:?}, state: {:?}", button, state);
        }
        for (axis, value) in motion_adapter.mapping.iter() {
            ent_text.partitions.first_mut().unwrap().characters =
                format!("motion axis: {:?}, value: {:?}", axis, value);
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
                Text::new(vec![("touch location: ", ((1.0, 1.0, 1.0), 0))]),
                (View {}, (0u32, 0u32), 0u32),
                TextScaleAlignment::Medium,
            ))
            .insert(TextBoundGuide::new(37, 10));
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
