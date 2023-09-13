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
macro_rules! medium {
    ($color:expr) => {
        (
            $color.0 * Self::MEDIUM_FACTOR,
            $color.1 * Self::MEDIUM_FACTOR,
            $color.2 * Self::MEDIUM_FACTOR,
        )
    };
}
macro_rules! dark {
    ($color:expr) => {
        (
            $color.0 * Self::DARK_FACTOR,
            $color.1 * Self::DARK_FACTOR,
            $color.2 * Self::DARK_FACTOR,
        )
    };
}
macro_rules! light {
    ($color:expr) => {
        (
            $color.0 * Self::LIGHT_FACTOR,
            $color.1 * Self::LIGHT_FACTOR,
            $color.2 * Self::LIGHT_FACTOR,
        )
    };
}
impl Color {
    pub(crate) const MEDIUM_FACTOR: f32 = 0.5;
    pub(crate) const DARK_FACTOR: f32 = 0.25;
    pub(crate) const LIGHT_FACTOR: f32 = 1.2;
    pub const WHITE: Rgb = (1.0, 1.0, 1.0);
    pub const DARK_ORANGE: Rgb = (0.035, 0.0125, 0.00);
    pub const CYAN_DARK: Rgb = dark!(Self::CYAN);
    pub const CYAN: Rgb = (0.0, 0.7, 0.58);
    pub const CYAN_MEDIUM: Rgb = medium!(Self::CYAN);
    pub const OFF_WHITE: Rgb = (0.8, 0.8, 0.8);
    pub const GREY_DARK: Rgb = dark!(Self::GREY);
    pub const GREY_MEDIUM: Rgb = medium!(Self::GREY);
    pub const GREY: Rgb = (0.25, 0.25, 0.25);
    pub const BLACK: Rgb = (0.0, 0.0, 0.0);
    pub const LIGHT_RED: Rgb = light!(Self::RED);
    pub const RED: Rgb = (0.6, 0.1, 0.0);
    pub const RED_MEDIUM: Rgb = medium!(Self::RED);
    pub const DARK_RED: Rgb = dark!(Self::RED);
    pub const LIGHT_RED_ORANGE: Rgb = light!(Self::RED_ORANGE);
    pub const RED_ORANGE: Rgb = (0.9, 0.45, 0.0);
    pub const RED_ORANGE_MEDIUM: Rgb = medium!(Self::RED_ORANGE);
    pub const RED_ORANGE_DARK: Rgb = dark!(Self::RED_ORANGE);
    pub const LIGHT_GREEN: Rgb = light!(Self::GREEN);
    pub const GREEN: Rgb = (0.0, 0.7, 0.0);
    pub const MEDIUM_GREEN: Rgb = medium!(Self::GREEN);
    pub const DARK_GREEN: Rgb = dark!(Self::GREEN);
    pub const BLUE_DARK: Rgb = dark!(Self::BLUE);
    pub const BLUE: Rgb = (0.0, 0.25, 0.5);
    pub const BLUE_MEDIUM: Rgb = medium!(Self::BLUE);
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
