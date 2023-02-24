use mise_en_place::{BundledIconKeys, Clickable, ClickListener, Color, FrontEndStages, Icon, IconBundle, IconKey, IconMesh, IconMeshAddRequest, IconSize, Job, Launch, Location, Text, TextBoundGuide, TextBundle, TextScaleAlignment};

use crate::logic::Counter;

pub struct Launcher;

impl Launch for Launcher {
    fn prepare(job: &mut Job) {
        job.container.insert_resource(Counter {
            count: 0,
            state: None,
        });
        job.main
            .add_system_to_stage(FrontEndStages::Process, crate::logic::update_text);
        job.container
            .spawn(TextBundle::new(
                Text::new(vec![("mouse location: ", (Color::OFF_WHITE, 0))]),
                Location::from(((35u32, 10u32), 0u32)),
                TextScaleAlignment::Medium,
            ))
            .insert(TextBoundGuide::new(18, 6));
        job.container
            .spawn(TextBundle::new(
                Text::new(vec![("click info: ", (Color::OFF_WHITE, 0))]),
                Location::from(((35u32, 160u32), 0u32)),
                TextScaleAlignment::Medium,
            ))
            .insert(TextBoundGuide::new(44, 3));
        job.container.spawn(IconMeshAddRequest::new(
            IconKey("bundled box"),
            IconMesh::bundled(BundledIconKeys::Box),
            10,
        ));
        let id = job
            .container
            .spawn(IconBundle::new(
                Icon {},
                IconSize::Large,
                IconKey("bundled box"),
                Location::from(((10u32, 17u32), 0u32)),
                Color::OFF_WHITE,
            ))
            .insert(Clickable::new(ClickListener::on_press(), false))
            .id();
        job.container
            .spawn(TextBundle::new(
                Text::new(vec![("messages: ", (Color::OFF_WHITE, 0))]),
                Location::from(((35u32, 260u32), 0u32)),
                TextScaleAlignment::Medium,
            ))
            .insert(TextBoundGuide::new(38, 20));
    }
}
