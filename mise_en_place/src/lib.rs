pub use bevy_ecs;

pub use animate::{Animate, Animation, start_animations};
pub use engen::{Attach, Engen, Launch};
pub use engen::{Attachment, EntityStore, ExecutionState, Exit, Idle, Job, Task};
pub use engen::BackendStages;
pub use engen::BackEndStartupStages;
pub use engen::EngenOptions;
pub use engen::FrontEndStages;
pub use engen::FrontEndStartupStages;
pub use request::{Request, spawn};
pub use visibility::VisibleBounds;
pub use wasm_compiler::WasmCompiler;
#[cfg(not(target_arch = "wasm32"))]
pub use wasm_server::WasmServer;

pub use crate::clickable::{Clickable, ClickListener, ClickState};
pub use crate::color::Color;
pub use crate::coord::{
    Area, AreaAdjust, Depth, DepthAdjust, DeviceView, GpuArea, GpuPosition, Location, Numerical,
    Position, PositionAdjust, PositionAdjustAnimator, Section, UIView,
};
pub use crate::gfx::{Viewport, ViewportAttachment};
pub use crate::icon::{
    ColorHooks, ColorInvert, Icon, IconAttachment, IconBundle, IconDescriptors, IconKey,
    IconMesh, IconMeshAddRequest, IconSize, IconVertex, read_mesh, write_mesh,
};
pub use crate::signal::Signal;
pub use crate::text::{
    Letter, LetterStyle, Text, TextAttachment, TextBundle, TextGridGuide, TextLine,
    TextScaleAlignment, TextScaleLetterDimensions, WrapStyleComponent, WrapStyleExpt,
};
pub use crate::text_input::{TextInput, TextInputPlugin};
pub use crate::theme::Theme;
pub use crate::time::{TimeDelta, TimeMarker, Timer};
pub use crate::visibility::{Visibility, VisibleSection};
pub use crate::window::{
    Click, ClickEvent, ClickEventType, Finger, MouseAdapter, MouseButtonExpt, Orientation, Resize,
    ScaleFactor, TouchAdapter, VirtualKeyboardAdapter, VirtualKeyboardType,
};

mod animate;
mod clickable;
mod color;
mod coord;
mod engen;
mod focus;
mod gfx;
mod icon;
mod instance;
mod r_button;
mod request;
mod signal;
mod text;
mod text_input;
mod theme;
mod time;
mod uniform;
mod visibility;
mod wasm_compiler;
#[cfg(not(target_arch = "wasm32"))]
#[allow(unused)]
mod wasm_server;
mod window;
