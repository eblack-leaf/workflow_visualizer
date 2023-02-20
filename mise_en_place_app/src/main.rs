#![allow(unused, dead_code)]

use std::ops::Add;

use bevy_ecs::prelude::{Commands, Entity, EventReader, Query, Res, ResMut, Resource};

use mise_en_place::{
    Color, ColorHooks, ColorInvert, DepthAdjust, Engen, EngenOptions, Exit, FrontEndStages,
    GpuPosition, Icon, IconBundle, IconPlugin, IconSize, Idle, Job, Launch, MouseButtonExpt,
    PartitionMetadata, PositionAdjust, Text, TextBoundGuide, TextBundle, TextPartition, TextPlugin,
    TextScaleAlignment, UIView, VirtualKeyboardAdapter, Visibility, WasmCompileDescriptor,
    WasmServer,
};
use mise_en_place::{IconKey, IconMesh, IconMeshAddRequest, IconVertex};

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
) {
    counter.count += 1;
    _idle.can_idle = false;
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
                (UIView {}, (35u32, 10u32), 0u32),
                TextScaleAlignment::Large,
            ))
            .insert(TextBoundGuide::new(44, 3));
        job.container
            .spawn(TextBundle::new(
                Text::new(vec![("mouse location: ", ((1.0, 1.0, 1.0), 0))]),
                (UIView {}, (35u32, 60u32), 0u32),
                TextScaleAlignment::Medium,
            ))
            .insert(TextBoundGuide::new(44, 3));
        job.container
            .spawn(TextBundle::new(
                Text::new(vec![("mouse buttons: ", ((1.0, 1.0, 1.0), 0))]),
                (UIView {}, (35u32, 100u32), 0u32),
                TextScaleAlignment::Small,
            ))
            .insert(TextBoundGuide::new(44, 10));
        job.container.spawn(IconMeshAddRequest::new(
            IconKey("mesh name"),
            IconMesh::new(ICON_MESH.iter().map(|v| *v).collect::<Vec<IconVertex>>()),
            10,
        ));
        let id = job
            .container
            .spawn(IconBundle::new(
                Icon {},
                IconSize::Large,
                IconKey("mesh name"),
                (UIView {}, (10u32, 17u32), 0u32),
                (1.0, 1.0, 1.0),
            ))
            .id();
        job.container.spawn(IconBundle::new(
            Icon {},
            IconSize::Medium,
            IconKey("mesh name"),
            (UIView {}, (10u32, 65u32), 0u32),
            (1.0, 1.0, 1.0),
        ));
        job.container.spawn(IconBundle::new(
            Icon {},
            IconSize::Small,
            IconKey("mesh name"),
            (UIView {}, (10u32, 105u32), 0u32),
            (1.0, 1.0, 1.0),
        ));
    }
}

pub(crate) const ICON_MESH: [IconVertex; 6] = [
    IconVertex::new(
        GpuPosition { x: 0.0, y: 0.0 },
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::CONSTANT),
    ),
    IconVertex::new(
        GpuPosition { x: 0.0, y: 1.0 },
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::CONSTANT),
    ),
    IconVertex::new(
        GpuPosition { x: 1.0, y: 0.0 },
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::CONSTANT),
    ),
    IconVertex::new(
        GpuPosition { x: 1.0, y: 0.0 },
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ),
    IconVertex::new(
        GpuPosition { x: 0.0, y: 1.0 },
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ),
    IconVertex::new(
        GpuPosition { x: 1.0, y: 1.0 },
        ColorHooks::new(ColorHooks::POSITIVE_SPACE, ColorHooks::HOOKABLE),
    ),
];

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
    engen.add_plugin::<IconPlugin>();
    engen.launch::<Launcher>();
}
