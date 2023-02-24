use serde::{Deserialize, Serialize};

use mise_en_place::{bevy_ecs, BundledIconKeys, Clickable, ClickListener, Color, FrontEndStages, IconBundle, IconKey, IconMesh, IconMeshAddRequest, IconSize, Job, Launch, Location, TextBoundGuide, TextBundle, TextScaleAlignment};
use mise_en_place::{
    Area, ClickState, Exit, Icon, Idle, MessageReceiver, MessageRepr, MessageType, MouseAdapter,
    Position, Text, TouchAdapter, UIView, VirtualKeyboardAdapter, VirtualKeyboardType,
};
use mise_en_place::bevy_ecs::change_detection::ResMut;
use mise_en_place::bevy_ecs::entity::Entity;
use mise_en_place::bevy_ecs::prelude::{Commands, Query, Res, Resource};

#[derive(Resource)]
pub struct Counter {
    pub(crate) count: u32,
    pub(crate) state: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct IntMessage(pub(crate) i32);

#[repr(u8)]
pub(crate) enum MessageTypes {
    IntMessage,
}

impl MessageRepr for IntMessage {
    fn message_type() -> MessageType {
        MessageTypes::IntMessage as u8
    }
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
    receiver: Res<MessageReceiver>,
) {
    counter.count += 1;
    _idle.can_idle = false;
    let mut click_info = String::new();
    for (entity, icon, click_state, position, area) in click_icon.iter() {
        if click_state.clicked() {
            click_info += &*format!("entity: {:?}, clicked: {:?}", entity, click_state.clicked(), );
            let current = counter.count;
            counter.state.replace(current);
            virtual_keyboard.open(VirtualKeyboardType::Keyboard);
            receiver.post_message(
                IntMessage(10),
                "yomi".to_string(),
                "password-easy".to_string(),
            );
        } else {
            if let Some(state) = counter.state {
                if counter.count >= state + 100 {
                    click_info +=
                        &*format!("entity: {:?}, clicked: {:?}", entity, click_state.clicked(), );
                    counter.state.take();
                }
            }
        }
    }
    for (entity, mut text) in text.iter_mut() {
        if entity.index() == 0 {
            let mouse_position = mouse_adapter.location().unwrap_or_default();
            let touch_position = touch_adapter.primary_touch();
            text.partitions.first_mut().unwrap().characters = format!(
                "mouse location x:{:.2}, y:{:.2}",
                mouse_position.x, mouse_position.y
            );
            if let Some(touch) = touch_position {
                text.partitions.first_mut().unwrap().characters = format!(
                    "touch location x:{:.2}, y:{:.2}",
                    touch.current.unwrap().x,
                    touch.current.unwrap().y
                );
            }
        }
        if entity.index() == 1 {
            if !click_info.is_empty() {
                text.partitions.first_mut().unwrap().characters = click_info.clone();
            }
        }
        if entity.index() == 4 {
            let messages = receiver.messages();
            for (user, messages) in messages {
                text.partitions.first_mut().unwrap().characters +=
                    format!("message-ty: {:?}", messages).as_str();
            }
        }
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
                Text::new(vec![("", (Color::OFF_WHITE, 0))]),
                Location::from(((35u32, 260u32), 0u32)),
                TextScaleAlignment::Medium,
            ))
            .insert(TextBoundGuide::new(38, 20));
    }
}
