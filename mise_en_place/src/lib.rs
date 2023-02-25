pub use bevy_ecs;

pub use engen::{Attach, Engen, Launch};
pub use engen::{ExecutionState, Exit, Idle, Job, Task};
pub use engen::BackendStages;
pub use engen::BackEndStartupStages;
pub use engen::EngenOptions;
pub use engen::FrontEndStages;
pub use engen::FrontEndStartupStages;
pub use visibility::VisibleBounds;
pub use wasm_compiler::WasmCompiler;
#[cfg(not(target_arch = "wasm32"))]
pub use wasm_server::WasmServer;

pub use crate::clickable::{Clickable, ClickListener, ClickState};
pub use crate::color::Color;
pub use crate::coord::{
    Area, AreaAdjust, Depth, DepthAdjust, DeviceView, GpuArea, GpuPosition, Location, Numerical,
    Position, PositionAdjust, Section, UIView,
};
pub use crate::gfx::{Viewport, ViewportPlugin};
pub use crate::icon::{
    BundledIconKeys, ColorHooks, ColorInvert, Icon, IconBundle, IconKey, IconMesh, IconMeshAddRequest,
    IconPlugin, IconSize, IconVertex, read_mesh, write_mesh,
};
pub use crate::text::{
    PartitionMetadata, Text, TextBoundGuide, TextBundle, TextPartition, TextPlugin,
    TextScaleAlignment,
};
pub use crate::theme::Theme;
pub use crate::visibility::{Visibility, VisibleSection};
pub use crate::window::{
    Click, ClickEvent, ClickEventType, Finger, MouseAdapter, MouseButtonExpt, Orientation, Resize,
    ScaleFactor, TouchAdapter, VirtualKeyboardAdapter, VirtualKeyboardType,
};

#[allow(unused)]
mod button;
mod clickable;
mod color;
mod coord;
mod engen;
mod gfx;
mod icon;
mod instance;
mod r_button;
mod text;
mod theme;
mod uniform;
mod visibility;
mod wasm_compiler;
#[cfg(not(target_arch = "wasm32"))]
#[allow(unused)]
mod wasm_server;
mod window;
