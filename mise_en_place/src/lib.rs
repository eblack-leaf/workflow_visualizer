pub use bevy_ecs;

pub use engen::BackEndStartupStages;
pub use engen::BackendStages;
pub use engen::EngenOptions;
pub use engen::FrontEndStages;
pub use engen::FrontEndStartupStages;
pub use engen::{Attach, Engen, Launch};
pub use engen::{ExecutionState, Exit, Idle, Job, Task};
pub use visibility::VisibleBounds;

pub use crate::clickable::{ClickListener, ClickState, Clickable};
pub use crate::color::Color;
pub use crate::coord::{
    Area, AreaAdjust, Depth, DepthAdjust, DeviceView, GpuArea, GpuPosition, Location, Numerical,
    Position, PositionAdjust, Section, UIView,
};
pub use crate::gfx::{Viewport, ViewportPlugin};
pub use crate::icon::{
    read_mesh, write_mesh, BundledIconKeys, ColorHooks, ColorInvert, Icon, IconBundle, IconKey,
    IconMesh, IconMeshAddRequest, IconPlugin, IconSize, IconVertex,
};
pub use crate::text::{
    PartitionMetadata, Text, TextBoundGuide, TextBundle, TextPartition, TextPlugin,
    TextScaleAlignment,
};
pub use crate::theme::Theme;
pub use crate::visibility::{Visibility, VisibleSection};
#[cfg(not(target_arch = "wasm32"))]
pub use crate::wasm::StatusCodeExpt;
pub use crate::wasm::{
    resolve_message, to_message, Message, MessageHandler, MessageReceiver, MessageRepr,
    MessageType, Password, Username, WasmCompiler, WasmServer,
};
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
mod wasm;
mod window;
