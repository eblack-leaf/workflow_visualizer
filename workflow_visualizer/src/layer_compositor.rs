use crate::Layer;
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum LayerCompositor {
    One = LayerCompositor::BEGIN,
    Two = LayerCompositor::BEGIN + LayerCompositor::LAYER_INTERVAL,
    Three = LayerCompositor::BEGIN + LayerCompositor::LAYER_INTERVAL * 2,
    Four = LayerCompositor::BEGIN + LayerCompositor::LAYER_INTERVAL * 3,
    Five = LayerCompositor::BEGIN + LayerCompositor::LAYER_INTERVAL * 4,
    Six = LayerCompositor::BEGIN + LayerCompositor::LAYER_INTERVAL * 5,
    Seven = LayerCompositor::BEGIN + LayerCompositor::LAYER_INTERVAL * 6,
    Eight = LayerCompositor::BEGIN + LayerCompositor::LAYER_INTERVAL * 7,
    Nine = LayerCompositor::BEGIN + LayerCompositor::LAYER_INTERVAL * 8,
    Ten = LayerCompositor::END,
}
impl LayerCompositor {
    pub(crate) const FAR_LAYER: u32 = 100u32;
    pub(crate) const NEAR_LAYER: u32 = 0u32;
    pub(crate) const LAYER_INTERVAL: u32 = 10u32;
    pub const BEGIN: u32 = 5u32;
    pub const END: u32 = 95u32;
    pub fn value(&self) -> Layer {
        (*self as u32).into()
    }
    pub fn arrangement(&self) -> LayerArrangement {
        LayerArrangement::new(self.value())
    }
}
pub struct LayerArrangement {
    target: Layer,
}
impl LayerArrangement {
    pub fn new<L: Into<Layer>>(target: L) -> Self {
        Self {
            target: target.into(),
        }
    }
    pub fn above(&self) -> Layer {
        self.target - 2.into()
    }
    pub fn behind(&self) -> Layer {
        self.target + 2.into()
    }
}
