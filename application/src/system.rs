use workflow_visualizer::{
    Area, BundledIcon, BundleExtension, BundlePlacement, Button, ButtonType, Color, Focus,
    FocusInputListener, Icon, IconBitmap, IconBitmapRequest, IconScale, InterfaceContext, Layer,
    Line, Panel, PanelType, PlacementReference, Position, ResponsiveGridPoint, ResponsiveGridView,
    ResponsivePathView, ResponsiveUnit, ScaleFactor, Sender, Text, TextScaleAlignment, TextValue,
    TextWrapStyle, Touchable, TouchListener, TouchTrigger, ViewportHandle, Workflow,
};
use workflow_visualizer::bevy_ecs::prelude::{Commands, Local, NonSend, Query, Res};

use crate::controller::SlotController;
use crate::workflow::{Engen, TokenName};

pub(crate) fn setup(mut cmd: Commands, viewport: Res<ViewportHandle>) {
    let mut placement_ref = PlacementReference::new();
    let slot_markers = ();
    let num_slots = ();
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
