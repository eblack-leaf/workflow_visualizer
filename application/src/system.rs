use workflow_visualizer::{Grid, Sender, TextValue, TouchTrigger};
use workflow_visualizer::bevy_ecs::event::EventReader;
use workflow_visualizer::bevy_ecs::prelude::{Commands, DetectChanges, NonSend, Query, Res};
use workflow_visualizer::bevy_ecs::system::ResMut;

use crate::slots::{SlotBlueprint, SlotFillEvent, SlotFills, SlotPool, Slots};
use crate::workflow::{Action, Engen};

pub(crate) fn setup(mut cmd: Commands, grid: Res<Grid>, sender: NonSend<Sender<Engen>>) {
    let blueprint = SlotBlueprint::new(&grid);
    sender.send(Action::RequestTokenNames);
    cmd.insert_resource(blueprint);
    cmd.insert_resource(SlotPool(vec![]));
    cmd.insert_resource(Slots(vec![]));
    cmd.insert_resource(SlotFills(vec![]));
}

pub(crate) fn update_blueprint(mut blueprint: ResMut<SlotBlueprint>, grid: Res<Grid>) {
    if grid.is_changed() {
        *blueprint = SlotBlueprint::new(&grid);
        let current = slots.0.len();
        let needed = blueprint.slots_per_page;
        let diff = needed as i32 - current as i32;
        if diff > 0 {
            // create slot and entities
        } else {
            // remove slot and entities
        }
    }
}

pub(crate) fn read_fill_event(
    mut events: EventReader<SlotFillEvent>,
    mut slot_pool: ResMut<SlotPool>,
) {
    for event in events.iter() {
        slot_pool.0 = event.tokens.clone();
    }
}

pub(crate) fn fill_slots(
    mut slot_fills: ResMut<SlotFills>,
    slot_pool: Res<SlotPool>,
    slot_blueprint: Res<SlotBlueprint>,
    // paging: Res<SlotPaging>,
) {
    if slot_pool.is_changed() || slot_blueprint.is_changed() { // or paging changed
        // align names to current slots
    }
}

pub(crate) fn process(
    slots: Res<Slots>,
    sender: NonSend<Sender<Engen>>,
    buttons: Query<(&TouchTrigger)>,
    text: Query<(&mut TextValue)>,
) {
    // check buttons and send actions of each slot
    // update text value with responses
}
