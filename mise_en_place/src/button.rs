use bevy_ecs::prelude::{Bundle, Commands, Component, Entity, Query, Res, ResMut, Resource};

use crate::{Attach, Depth, Engen, FrontEndStartupStages, Location, Position, ScaleFactor, TextScaleAlignment, Theme, View};

#[derive(Component)]
pub struct ButtonText {
    pub(crate) text: Option<String>,
    pub(crate) alignment: Option<TextScaleAlignment>,
}

impl ButtonText {
    pub(crate) fn new() -> Self {
        Self {
            text: None,
            alignment: None,
        }
    }
}

#[derive(Component)]
pub struct ButtonIcon {
    pub(crate) icon: Option<()>,
}

impl ButtonIcon {
    pub(crate) fn new() -> Self {
        Self { icon: None }
    }
}

#[derive(Bundle)]
pub struct ButtonBundle {
    pub location: Location<View>,
    pub text: ButtonText,
    pub icon: ButtonIcon,
    // comps for storing primary/secondary colors (together cause comp overwrite if insert 2 Color)
    // colors can default to theme colors when read in bundle_insert
}

impl ButtonBundle {
    pub fn new(location: Location<View>) -> Self {
        Self {
            location,
            text: ButtonText::new(),
            icon: ButtonIcon::new(),
        }
    }
    pub fn with_text(mut self, text: String, alignment: TextScaleAlignment) -> Self {
        self.text.text.replace(text);
        self.text.alignment.replace(alignment);
        self
    }
    pub fn with_icon(mut self, icon: () /* size of icon too */) -> Self {
        self.icon.icon.replace(icon);
        self
    }
}

#[derive(Resource)]
pub(crate) struct ButtonMeshGuide {
    // z for text / icon to work
    // location adjusts for text + icon placement
    // padding for elements from scale factor
}

impl ButtonMeshGuide {
    pub(crate) fn new(line_width: u32, corner_precision: u32) -> Self {
        Self {}
    }
}

// put same func in both frontend + backend (for making vertex same from frontend)
pub(crate) fn general_setup(mut cmd: Commands) {
    cmd.insert_resource(ButtonMeshGuide::new(3, 16));
}

pub(crate) fn bundle_insert(
    theme: Res<Theme>,
    mut mesh_guide: ResMut<ButtonMeshGuide>,
    button_bundles: Query<(Entity, &Position<View>, &Depth, &ButtonText, &ButtonIcon), ()>,
    scale_factor: Res<ScaleFactor>,
    mut cmd: Commands,
) {
    for (entity, position, depth, text, icon) in button_bundles.iter() {
        cmd.entity(entity).insert(Button::new());
        if let Some(t) = &text.text {
            // max text for buttons is bound one line and 15 char
        }
        if let Some(i) = &icon.icon {
            // max icon size for this is max_text_scale for platform.character dimensions
            // if has text, size icon to text.character dimensions
            // if no text, scale icon size to scaled and set cause click registers on area as scaled
        }
        // calc area from biggest of icon size + text scale character_dimensions
        // or calc area cause text + icon could change so size needs to be adjusted
        // this just spawns correct stuff at guided locations
    }
}

pub(crate) fn calc_area() {
    // run after text + icon area calcs
}

// specific typed listeners need to be added to give functionality to click
// they read click and do what is needed
pub(crate) fn register_click() {
    // if origin + end of valid click is seen in this bound { clicked = true }
    // set color fill uniform for this entity to regular/inverted
    // click in Device CoordsContext so scale the area... but area is already scaled as text characters are scaled
    // so scale padding and initial icon size, then use area as is calc-ed from those scaled values to register click
}

pub(crate) fn reset_click() {
    // clicked = false for all
}

pub(crate) struct ButtonRenderer {
    // instance buffer
    // pipeline
    // layouts + descriptors (no bind groups per button as only instance attributes)
    // vertex
    // placement_instance_attribute - placement (position, depth)
    // split to position/depth attributes
    // color_base_instance_attribute - primary/inverted color instance attribute
    // color_fill_instance_attribute - 0/1 for regular/inverted - one for each entity
    // area_instance_attribute - calc-ed from icon + text + padding
}

#[derive(Component)]
pub struct Button {
    pub(crate) clicked: bool,
    pub disabled: bool,
}

impl Button {
    pub(crate) fn new() -> Self {
        Self {
            clicked: false,
            disabled: false,
        }
    }
}

pub struct ButtonPlugin;

impl Attach for ButtonPlugin {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .startup
            .add_system_to_stage(FrontEndStartupStages::Startup, general_setup);
        engen
            .frontend
            .startup
            .add_system_to_stage(FrontEndStartupStages::Initialization, bundle_insert);
    }
}
