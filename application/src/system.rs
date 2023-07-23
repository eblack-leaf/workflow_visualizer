use application::controller::{Slot, SlotFillEvent};
use workflow_visualizer::bevy_ecs::event::EventReader;
use workflow_visualizer::bevy_ecs::prelude::{Commands, Local, NonSend, Query, Res};
use workflow_visualizer::bevy_ecs::system::ResMut;
use workflow_visualizer::{
    Area, BundleExtension, BundlePlacement, BundledIcon, Button, ButtonType, Color, Focus,
    FocusInputListener, Grid, Icon, IconBitmap, IconBitmapRequest, IconScale, InterfaceContext,
    Layer, Line, Panel, PanelType, PlacementReference, Position, RawMarker, ResponsiveGridPoint,
    ResponsiveGridView, ResponsivePathView, ResponsiveUnit, ScaleFactor, Sender, Text, TextScale,
    TextScaleAlignment, TextValue, TextWrapStyle, TouchListener, TouchTrigger, Touchable,
    ViewportHandle, Workflow,
};

use crate::controller::{SlotBlueprint, Slots};
use crate::workflow::{Action, Engen, TokenName};

pub(crate) fn setup(mut cmd: Commands, grid: Res<Grid>, sender: NonSend<Sender<Engen>>) {
    let slot_controller = Slots::new(&grid);
    sender.send(Action::RequestTokenNames);
    cmd.insert_resource(slot_controller);
}
pub(crate) fn read_fill_event(
    mut cmd: Commands,
    mut events: EventReader<SlotFillEvent>,
    mut slots: ResMut<Slots>,
) {
    for event in events.iter() {
        slots.fill(event.tokens.clone());
    }
}
pub(crate) fn send_event(
    sender: NonSend<Sender<Engen>>,
    mut text: Query<(
        &mut TextValue,
        &Position<InterfaceContext>,
        &Area<InterfaceContext>,
    )>,
    buttons: Query<(&TouchTrigger)>,
    mut limiter: Local<bool>,
    scale_factor: Res<ScaleFactor>,
    controller: Res<Slots>,
) {
    if !*limiter {
        let action = <Engen as Workflow>::Action::GenerateOtp(TokenName("not there".to_string()));
        sender.send(action);
        *limiter = true;
    }
}
