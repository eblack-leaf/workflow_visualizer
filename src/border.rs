use crate::Color;
use bevy_ecs::prelude::Component;

#[derive(Component, Copy, Clone)]
pub struct LineWidth(pub u32);
#[derive(Component, Copy, Clone)]
pub struct LineColor(pub Color);
