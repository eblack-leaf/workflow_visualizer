#![allow(unused, dead_code)]

use std::ops::Add;

use bevy_ecs::prelude::{Commands, Entity, Query, Res, ResMut, Resource};

use mise_en_place::{Color, DepthAdjust, Engen, EngenOptions, Exit, FrontEndStages, IconKey, IconMesh, IconMeshAddRequest, Idle, Job, Launch, MouseAdapter, PartitionMetadata, PositionAdjust, Text, TextBoundGuide, TextBundle, TextPartition, TextPlugin, TextScaleAlignment, TouchAdapter, UIView, Visibility, WasmCompileDescriptor, WasmServer};

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
) {
    counter.count += 1;
    _idle.can_idle = false;
    for (entity, mut ent_text) in text.iter_mut() {
        if let Some(finger) = touch_adapter.primary.as_ref() {
            if entity.index() == 0 {
                let current = touch_adapter
                    .tracked
                    .get(finger)
                    .expect("no tracked click")
                    .current;
                if let Some(curr) = current {
                    ent_text.partitions.first_mut().unwrap().characters =
                        format!("touch location: {:.2}, {:.2}", curr.x, curr.y);
                }
            }
        }
        if let Some(location) = mouse_adapter.location {
            if entity.index() == 1 {
                ent_text.partitions.first_mut().unwrap().characters =
                    format!("mouse location: {:.2}, {:.2}", location.x, location.y);
            }
        }
        let mut button_click_text = String::new();
        for (button, click) in mouse_adapter.tracked_buttons.iter() {
            button_click_text = button_click_text
                .add(format!("button: {:?}, state: {:?}\n", button, click.current).as_str());
        }
        if entity.index() == 2 {
            ent_text.partitions.first_mut().unwrap().characters = button_click_text;
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
                (UIView {}, (0u32, 0u32), 0u32),
                TextScaleAlignment::Small,
            ))
            .insert(TextBoundGuide::new(44, 3));
        job.container
            .spawn(TextBundle::new(
                Text::new(vec![("mouse location: ", ((1.0, 1.0, 1.0), 0))]),
                (UIView {}, (0u32, 60u32), 0u32),
                TextScaleAlignment::Small,
            ))
            .insert(TextBoundGuide::new(44, 3));
        job.container
            .spawn(TextBundle::new(
                Text::new(vec![("mouse buttons: ", ((1.0, 1.0, 1.0), 0))]),
                (UIView {}, (0u32, 100u32), 0u32),
                TextScaleAlignment::Small,
            ))
            .insert(TextBoundGuide::new(44, 10));
        let mesh = Vec::new();
        job.container.spawn(IconMeshAddRequest::new(IconKey("mesh name"), IconMesh::new(mesh), 10));
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
    engen.add_plugin::<TextPlugin>();
    engen.launch::<Launcher>();
}
