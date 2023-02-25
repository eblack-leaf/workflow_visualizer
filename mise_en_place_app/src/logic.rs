use serde::{Deserialize, Serialize};

use mise_en_place::{
    bevy_ecs, BundledIconKeys, Clickable, ClickListener, Color, FrontEndStages, IconBundle,
    IconKey, IconMesh, IconMeshAddRequest, IconSize, Job, Launch, Location, TextBoundGuide,
    TextBundle, TextScaleAlignment,
};
use mise_en_place::{
    Area, ClickState, Exit, Icon, Idle, MouseAdapter, Position, Text, TouchAdapter, UIView,
    VirtualKeyboardAdapter, VirtualKeyboardType,
};
use mise_en_place::bevy_ecs::change_detection::ResMut;
use mise_en_place::bevy_ecs::entity::Entity;
use mise_en_place::bevy_ecs::prelude::{Commands, Query, Res, Resource};

#[derive(Resource)]
pub struct Counter {
    pub(crate) count: u32,
    pub(crate) state: Option<u32>,
}

pub fn update_text(
    click_icon: Query<(Entity, &Icon, &ClickState, &Position<UIView>, &Area<UIView>)>,
    mut text: Query<(Entity, &mut Text)>,
    mut counter: ResMut<Counter>,
    mut _idle: ResMut<Idle>,
    mut cmd: Commands,
    mut exit: ResMut<Exit>,
    mouse_adapter: Res<MouseAdapter>,
    touch_adapter: Res<TouchAdapter>,
    virtual_keyboard: Res<VirtualKeyboardAdapter>,
) {
    counter.count += 1;
    _idle.can_idle = false;
    let mut click_info = String::new();
    for (entity, icon, click_state, position, area) in click_icon.iter() {
        if entity.index() == 3 {
            if click_state.clicked() {
                click_info += &*format!("email copied to clipboard:\n{:?}", "jimblack@example.com");
                let current = counter.count;
                counter.state.replace(current);
                // virtual_keyboard.open(VirtualKeyboardType::Keyboard);
            } else {
                if let Some(state) = counter.state {
                    if counter.count >= state + 100 {
                        click_info +=
                            &*format!("jimblack@example.com (copied)");
                        counter.state.take();
                    }
                }
            }
        }
        if entity.index() == 5 {
            if click_state.clicked() {
                virtual_keyboard.open(VirtualKeyboardType::Keyboard);
            }
        }
    }
    for (entity, mut text) in text.iter_mut() {
        if entity.index() == 0 {
            // let mouse_position = mouse_adapter.location().unwrap_or_default();
            // let touch_position = touch_adapter.primary_touch();
            // if let Some(touch) = touch_position {
            //     text.partitions.first_mut().unwrap().characters = format!(
            //         "touch location x:{:.2}, y:{:.2}",
            //         touch.current.unwrap().x,
            //         touch.current.unwrap().y
            //     );
            // } else {
            //     text.partitions.first_mut().unwrap().characters = format!(
            //         "mouse location x:{:.2}, y:{:.2}",
            //         mouse_position.x, mouse_position.y
            //     );
            // }
        }
        if entity.index() == 1 {
            if !click_info.is_empty() {
                text.partitions.first_mut().unwrap().characters = click_info.clone();
            }
        }
        if entity.index() == 4 {}
    }
}

pub struct Launcher;

impl Launch for Launcher {
    fn prepare(job: &mut Job) {
        job.container.insert_resource(Counter {
            count: 0,
            state: None,
        });
        job.main
            .add_system_to_stage(FrontEndStages::Process, update_text);
        job.container
            .spawn(TextBundle::new(
                Text::new(vec![("Jim Black - Richmond Artist", (Color::OFF_WHITE, 0))]),
                Location::from(((10u32, 10u32), 0u32)),
                TextScaleAlignment::Large,
            ))
            .insert(TextBoundGuide::new(18, 6));
        job.container
            .spawn(TextBundle::new(
                Text::new(vec![("jimblack@example.com", (Color::OFF_WHITE, 0))]),
                Location::from(((40u32, 600u32), 0u32)),
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
                Location::from(((10u32, 600u32), 0u32)),
                Color::OFF_WHITE,
            ))
            .insert(Clickable::new(ClickListener::on_press(), false))
            .id();
        job.container
            .spawn(TextBundle::new(
                Text::new(vec![(
                    "atoeh aoneco oceceon ubsa onetas u hotena netu\
                    aset ececn osanote otecu ou onecu onteecuho anec\
                    ocunon ancenohu nocenosa uceocna uceocnau ehucuo \
                    ocauhnc enou ononotctjwmq'v' 'nwj jwm qvj ktqvj k\
                    jwv qjtktqvwjk qtjv kqtjw teecor .roll,.l. .co  t\
                    aset ececn osanote otecu ou onecu onteecuho anec\
                    ocunon ancenohu nocenosa uceocna uceocnau ehucuo \
                    ocauhnc enou ononotctjwmq'v' 'nwj jwm qvj ktqvj k\
                    et anen cis 'vq 'ut' conrec ateno ocetu utet ucano",
                    (Color::OFF_WHITE, 0))]),
                Location::from(((10u32, 100u32), 0u32)),
                TextScaleAlignment::Medium,
            ))
            .insert(TextBoundGuide::new(38, 20));
        job
            .container
            .spawn(IconBundle::new(
                Icon {},
                IconSize::Large,
                IconKey("bundled box"),
                Location::from(((350u32, 10u32), 0u32)),
                Color::OFF_WHITE,
            ))
            .insert(Clickable::new(ClickListener::on_release(), false))
            .id();
    }
}
