pub use bevy_ecs;

pub use animate::{start_animations, Animate, Animation};
pub use clickable::ClickListener;
pub use clickable::ClickState;
pub use clickable::Clickable;
pub use engen::BackEndStartupBuckets;
pub use engen::BackendBuckets;
pub use engen::EngenOptions;
pub use engen::FrontEndBuckets;
pub use engen::FrontEndStartupBuckets;
pub use engen::{Attach, Engen, Launch};
pub use engen::{Attachment, EntityStore, ExecutionState, Exit, Idle, Job, Task};
pub use request::{spawn, Request};
pub use text_input::TextGridLocation;
pub use text_input::TextInput;
pub use text_input::TextInputAttachment;
pub use text_input::TextInputRequest;
pub use text_input::TextInputText;
pub use visibility::VisibleBounds;
pub use wasm_compiler::WasmCompiler;
#[cfg(not(target_arch = "wasm32"))]
pub use wasm_server::WasmServer;
pub use window::Click;
pub use window::ClickEvent;
pub use window::ClickEventType;
pub use window::Finger;
pub use window::MouseAdapter;
pub use window::Orientation;
pub use window::ScaleFactor;
pub use window::TouchAdapter;
pub use window::VirtualKeyboardAdapter;
pub use window::VirtualKeyboardType;
pub use window::WindowResize;

pub use crate::color::Color;
pub use crate::coord::{
    Area, AreaAdjust, Depth, DepthAdjust, DeviceView, GpuArea, GpuPosition, Location, Numerical,
    Position, PositionAdjust, PositionAdjustAnimator, Section, UIView,
};
pub use crate::gfx::{Viewport, ViewportAttachment};
pub use crate::icon::{
    read_mesh, write_mesh, ColorHooks, ColorInvert, Icon, IconAttachment, IconBundle,
    IconDescriptors, IconKey, IconMesh, IconMeshAddRequest, IconSize, IconVertex,
};
pub use crate::signal::Signal;
pub use crate::text::{
    Letter, LetterStyle, TextAttachment, TextBuffer, TextBundle, TextContent, TextContentView,
    TextGridGuide, TextLineStructure, TextScaleAlignment, TextScaleLetterDimensions,
    WrapStyleComponent, WrapStyleExpt,
};
pub use crate::theme::Theme;
pub use crate::visibility::{Visibility, VisibleSection};

pub use self::time::TimeDelta;
pub use self::time::TimeMarker;
pub use self::time::Timer;

mod animate;
mod button;
mod clickable;
mod color;
mod coord;
mod engen;
mod focus;
mod gfx;
mod icon;
mod instance;
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
