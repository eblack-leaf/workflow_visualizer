use workflow_visualizer::{BundleExtension, GridMarkerBias};
use workflow_visualizer::{
    Area, BundledIcon, BundlePlacement, Button, ButtonType, Color, EntityName, Focus,
    FocusInputListener, GfxOptions, Grid, HorizontalSpan, Icon, IconBitmap,
    IconBitmapRequest, IconScale, Layer, Line, Panel, PanelType, PlacementReference, Position,
    ResponsivePathView, Text, TextScaleAlignment, TextWrapStyle, Theme, ThemeDescriptor,
    Touchable, TouchListener, UserSpaceSyncPoint, Visualizer,
};
use workflow_visualizer::{ResponsiveGridPoint, ResponsiveGridView};
use workflow_visualizer::bevy_ecs::prelude::IntoSystemConfig;
use workflow_visualizer::ResponsiveUnit;

use crate::system;

pub fn visualizer() -> Visualizer {
    let theme_desc = ThemeDescriptor::new().with_background(Color::OFF_BLACK);
    let mut visualizer = Visualizer::new(
        Theme::new(theme_desc),
        GfxOptions::native_defaults().with_msaa(1),
    );
    visualizer.job.task(Visualizer::TASK_STARTUP).add_systems((system::setup.in_set(UserSpaceSyncPoint::Initialization), ));
    visualizer
        .job
        .task(Visualizer::TASK_MAIN)
        .add_systems((system::send_event.in_set(UserSpaceSyncPoint::Process), ));
    visualizer
}
