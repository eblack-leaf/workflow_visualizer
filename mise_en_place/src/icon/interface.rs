use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;

use crate::icon::IconKey;

#[derive(Bundle)]
pub struct Icon {
    pub size: IconSize,
    pub key: IconKey,
}

#[derive(Component)]
pub enum IconSize {
    Small,
    Medium,
    Large,
}
