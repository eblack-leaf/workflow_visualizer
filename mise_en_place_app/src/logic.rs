use mise_en_place::{Area, ClickState, Exit, Icon, Idle, MessageReceiver, MouseAdapter, Position, Text, TouchAdapter, UIView, VirtualKeyboardAdapter, VirtualKeyboardType};
use mise_en_place::bevy_ecs;
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
                "cornelius_fudge".to_string(),
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
            if let Some(mes) = messages.get("yomi").take() {
                text.partitions.first_mut().unwrap().characters += mes;
            }
        }
    }
}
