use bevy_ecs::component::Component;

#[derive(Component, Clone, Copy, Hash, Eq, PartialEq)]
pub(crate) struct TextScale {
    pub(crate) scale: u32,
}

impl TextScale {
    pub(crate) fn new(scale: u32) -> Self {
        Self { scale }
    }
    pub(crate) fn px(&self) -> f32 {
        self.scale as f32
    }
    pub(crate) fn from_alignment(alignment: TextScaleAlignment, scale_factor: f64) -> Self {
        match alignment {
            TextScaleAlignment::Small => {
                Self::new((TEXT_SCALE_ALIGNMENT_GUIDE[0] as f64 * scale_factor) as u32)
            }
            TextScaleAlignment::Medium => {
                Self::new((TEXT_SCALE_ALIGNMENT_GUIDE[1] as f64 * scale_factor) as u32)
            }
            TextScaleAlignment::Large => {
                Self::new((TEXT_SCALE_ALIGNMENT_GUIDE[2] as f64 * scale_factor) as u32)
            }
        }
    }
}

impl From<f32> for TextScale {
    fn from(scale: f32) -> Self {
        Self {
            scale: scale as u32,
        }
    }
}

impl From<u32> for TextScale {
    fn from(scale: u32) -> Self {
        Self { scale }
    }
}

#[derive(Component, Copy, Clone)]
pub enum TextScaleAlignment {
    Small,
    Medium,
    Large,
}

const TEXT_SCALE_ALIGNMENT_GUIDE: [u32; 3] = [16, 22, 28];
