use crate::{Animate, Animation, Attach, Interpolation, SyncPoint, Visualizer};
use bevy_ecs::prelude::{Component, IntoSystemConfigs};
use bevy_ecs::system::Query;

/// RGBA colors
#[repr(C)]
#[derive(Component, bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}
pub type Rgb = (f32, f32, f32);
pub type Rgba = (f32, f32, f32, f32);
impl Default for Color {
    fn default() -> Self {
        Self::from_rgb(1.0, 1.0, 1.0)
    }
}
#[derive(Default, Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct ColorBuilder {
    pub red: Option<f32>,
    pub green: Option<f32>,
    pub blue: Option<f32>,
    pub alpha: Option<f32>,
}
impl ColorBuilder {
    pub fn with_red(mut self, red: f32) -> Self {
        self.red.replace(red);
        self
    }
    pub fn with_green(mut self, green: f32) -> Self {
        self.green.replace(green);
        self
    }
    pub fn with_blue(mut self, blue: f32) -> Self {
        self.blue.replace(blue);
        self
    }
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha.replace(alpha);
        self
    }
    pub fn build(&self) -> Color {
        Color::from_rgba(
            self.red.unwrap_or_default(),
            self.green.unwrap_or_default(),
            self.blue.unwrap_or_default(),
            self.alpha.unwrap_or_default(),
        )
    }
}
impl Color {
    pub const WHITE: Rgb = (1.0, 1.0, 1.0);
    pub const DARK_ORANGE: Rgb = (0.035, 0.0125, 0.00);
    pub const DARK_CYAN: Rgb = (0.0, 0.025, 0.0125);
    pub const CYAN: Rgb = (0.0, 0.75, 0.35);
    pub const CYAN_MEDIUM: Rgb = (0.0, 0.45, 0.145);
    pub const OFF_WHITE: Rgb = (0.8, 0.8, 0.8);
    pub const DARK_GREY: Rgb = (0.025, 0.025, 0.025);
    pub const MEDIUM_GREY: Rgb = (0.05, 0.05, 0.05);
    pub const GREY: Rgb = (0.25, 0.25, 0.25);
    pub const BLACK: Rgb = (0.0, 0.0, 0.0);
    pub const LIGHT_RED: Rgb = (0.9, 0.02, 0.02);
    pub const RED: Rgb = (0.8, 0.01, 0.01);
    pub const MEDIUM_RED: Rgb = (0.4, 0.005, 0.005);
    pub const DARK_RED: Rgb = (0.2, 0.0025, 0.0025);
    pub const LIGHT_RED_ORANGE: Rgb = (0.9, 0.35, 0.1);
    pub const RED_ORANGE: Rgb = (0.7, 0.225, 0.045);
    pub const MEDIUM_RED_ORANGE: Rgb = (0.4, 0.125, 0.025);
    pub const DARK_RED_ORANGE: Rgb = (0.2, 0.07, 0.0125);
    pub const LIGHT_GREEN: Rgb = (0.02, 0.9, 0.02);
    pub const GREEN: Rgb = (0.01, 0.7, 0.01);
    pub const MEDIUM_GREEN: Rgb = (0.001, 0.16, 0.001);
    pub const DARK_GREEN: Rgb = (0.0005, 0.08, 0.0005);
    pub const BLUE_DARK: Rgb = (0.0, 0.05, 0.10);
    pub const BLUE: Rgb = (0.1, 0.1, 0.9);
    pub const OFF_BLACK: Rgb = (0.005, 0.005, 0.005);
    pub const BLANK: Rgba = (0.0, 0.0, 0.0, 0.0);
    pub fn from_rgb(red: f32, green: f32, blue: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 1f32,
        }
    }
    pub fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
    pub fn red(&self) -> f32 {
        self.red
    }
    pub fn green(&self) -> f32 {
        self.green
    }
    pub fn blue(&self) -> f32 {
        self.blue
    }
    pub fn alpha(&self) -> f32 {
        self.alpha
    }
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha.min(1.0).max(0.0);
        self
    }
}

impl From<Color> for wgpu::Color {
    fn from(color: Color) -> Self {
        Self {
            r: color.red as f64,
            g: color.green as f64,
            b: color.blue as f64,
            a: color.alpha as f64,
        }
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        Self::from_rgb(rgb.0, rgb.1, rgb.2)
    }
}

impl From<Rgba> for Color {
    fn from(rgba: Rgba) -> Self {
        Self::from_rgba(rgba.0, rgba.1, rgba.2, rgba.3)
    }
}
impl Animate for Color {
    fn interpolations(&self, end: &Self) -> Vec<Interpolation> {
        vec![
            Interpolation::new(end.red - self.red),
            Interpolation::new(end.green - self.green),
            Interpolation::new(end.blue - self.blue),
            Interpolation::new(end.alpha - self.alpha),
        ]
    }
}
pub(crate) fn apply_animations(mut anims: Query<(&mut Color, &mut Animation<Color>)>) {
    for (mut color, mut anim) in anims.iter_mut() {
        let extracts = anim.extractions();
        if let Some(extract) = extracts.get(0).expect("red") {
            color.red += extract.value();
        }
        if let Some(extract) = extracts.get(1).expect("green") {
            color.green += extract.value();
        }
        if let Some(extract) = extracts.get(2).expect("blue") {
            color.blue += extract.value();
        }
        if let Some(extract) = extracts.get(3).expect("alpha") {
            color.alpha += extract.value();
        }
    }
}

pub(crate) struct ColorAttachment;
impl Attach for ColorAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.register_animation::<Color>();
        visualizer
            .task(Visualizer::TASK_MAIN)
            .add_systems((apply_animations.in_set(SyncPoint::Animation),));
    }
}
