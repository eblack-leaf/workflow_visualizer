use mise_en_place::bevy_ecs::change_detection::ResMut;
use mise_en_place::bevy_ecs::entity::Entity;
use mise_en_place::bevy_ecs::prelude::{Commands, Query, Res, Resource};
use mise_en_place::{
    bevy_ecs, BundledIconKeys, ClickListener, Clickable, Color, FrontEndStages, IconBundle,
    IconKey, IconMesh, IconMeshAddRequest, IconSize, Job, Launch, Location, TextBoundGuide,
    TextBundle, TextScaleAlignment,
};
use mise_en_place::{
    Area, ClickState, Exit, Icon, Idle, MouseAdapter, Position, Text, TouchAdapter, UIView,
    VirtualKeyboardAdapter, VirtualKeyboardType,
};

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
) {
    counter.count += 1;
    _idle.can_idle = false;
    let mut click_info = String::new();
    let mut second_info = String::new();
    for (entity, icon, click_state, position, area) in click_icon.iter() {
        if entity.index() == 3 {
            if click_state.clicked() {
                click_info += &*format!("email copied to clipboard: {:?}", "someone@example.com");
                let current = counter.count;
                counter.state.replace(current);
            } else {
                if let Some(state) = counter.state {
                    if counter.count >= state + 100 {
                        click_info += &*format!("someone@example.com (copied)");
                        counter.state.take();
                    }
                }
            }
        }
        if entity.index() == 5 {
            if click_state.clicked() {
                second_info += &*format!("other button pushed at counter: {:?}", counter.count);
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
        if entity.index() == 4 {
            if !second_info.is_empty() {
                text.partitions.first_mut().unwrap().characters = second_info.clone();
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
                Text::new(vec![("Lorem ipsum dolor sit amet", (Color::OFF_WHITE, 0))]),
                Location::from(((10u32, 10u32), 0u32)),
                TextScaleAlignment::Large,
            ))
            .insert(TextBoundGuide::new(18, 6));
        job.container
            .spawn(TextBundle::new(
                Text::new(vec![("someone@example.com", (Color::OFF_WHITE, 0))]),
                Location::from(((40u32, 600u32), 0u32)),
                TextScaleAlignment::Medium,
            ))
            .insert(TextBoundGuide::new(38, 3));
        job.container.spawn(IconMeshAddRequest::new(
            IconKey("bundled box"),
            IconMesh::bundled(BundledIconKeys::Box),
            10,
        ));
        let id = job
            .container
            .spawn(IconBundle::new(
                Icon::new(Color::OFF_BLACK),
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
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
                    Pellentesque odio dui, cursus vel commodo id, sollicitudin \
                    nec ipsum. Aenean nec ante ac arcu interdum porttitor. \
                    Praesent suscipit quis libero sed pellentesque. \
                    Suspendisse feugiat egestas nulla sed semper.\
                     Ut id quam volutpat, mollis mauris quis, cursus enim. \
                     Curabitur ultrices id metus. ",
                    (Color::OFF_WHITE, 0),
                )]),
                Location::from(((10u32, 100u32), 0u32)),
                TextScaleAlignment::Medium,
            ))
            .insert(TextBoundGuide::new(38, 20));
        job.container
            .spawn(IconBundle::new(
                Icon::new(Color::OFF_BLACK),
                IconSize::Large,
                IconKey("bundled box"),
                Location::from(((350u32, 10u32), 0u32)),
                Color::OFF_WHITE,
            ))
            .insert(Clickable::new(ClickListener::on_release(), false))
            .id();
    }
}
