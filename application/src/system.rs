use workflow_visualizer::{Area, BundledIcon, BundleExtension, BundlePlacement, Button, ButtonType, Color, Focus, FocusInputListener, Grid, Icon, IconBitmap, IconBitmapRequest, IconScale, InterfaceContext, Layer, Line, Panel, PanelType, PlacementReference, Position, RawMarker, ResponsiveGridPoint, ResponsiveGridView, ResponsivePathView, ResponsiveUnit, ScaleFactor, Sender, Text, TextScale, TextScaleAlignment, TextValue, TextWrapStyle, Touchable, TouchListener, TouchTrigger, ViewportHandle, Workflow};
use workflow_visualizer::bevy_ecs::prelude::{Commands, Local, NonSend, Query, Res};

use crate::controller::{SlotBlueprint, SlotController};
use crate::workflow::{Action, Engen, TokenName};

pub(crate) fn setup(mut cmd: Commands, grid: Res<Grid>, sender: NonSend<Sender<Engen>>) {
    let slot_controller = SlotController::new(&grid);
    sender.send(Action::RequestTokenNames);
    cmd.insert_resource(slot_controller);
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
    controller: Res<SlotController>,
) {
    if !*limiter {
        let action = <Engen as Workflow>::Action::GenerateOtp(TokenName("not there".to_string()));
        sender.send(action);
        *limiter = true;
    }
}
