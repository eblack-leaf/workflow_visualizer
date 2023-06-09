#![allow(unused, dead_code)]
//! Workflow Visualizer is a lib for cross-platform declarative UI
//! It is comprised of three major parts.
//! trait Workflow to setup application messaging.
//! Visualizer which handles rendering
//! Runner which invokes the visualizer's tools for multiple platforms
pub use bevy_ecs;
pub use wgpu;
pub use winit;

pub use instance::{
    AttributeWrite, CpuAttributeBuffer, GpuAttributeBuffer, Index, Indexer,
    InstanceAttributeManager, Key, KeyFactory, NullBit,
};
pub use job::JobSyncPoint;
pub use render::{Render, RenderPassHandle, RenderPhase};
pub use viewport::{Viewport, ViewportHandle};
pub use visualizer::{Attach, Attachment, Visualizer};
pub use workflow::{Runner, Sender, start_web_worker, Workflow};

pub use crate::color::Color;
pub use crate::coord::{
    area::Area, area::RawArea, Coordinate, DeviceContext, InterfaceContext,
    layer::Layer, NumericalContext, position::Position, position::RawPosition, section::Section,
};
pub use crate::focus::{Focus, FocusInputListener};
pub use crate::gfx::{GfxOptions, GfxSurface};
pub(crate) use crate::gfx::{GfxSurfaceConfiguration, MsaaRenderAdapter};
pub use crate::grid::{
    Grid, GridLocation, GridLocationDescriptor, GridMarker, GridMarkerBias, GridRange, GridView,
    RawMarker, RawMarkerGrouping, ResponsiveGridView, ResponsiveUnit, ResponsiveView,
};
pub use crate::job::{EntityName, Job};
pub use crate::line::{Line, LineRender};
pub use crate::path::{Path, PathView, PathViewPoint, ResponsivePathView};
pub use crate::request::{Request, spawn};
pub use crate::scale_factor::ScaleFactor;
pub use crate::sync::{SyncPoint, UserSpaceSyncPoint};
pub use crate::text::{
    Text, TextGridLocation, TextGridPlacement, TextLetterDimensions, TextLineStructure, TextScale,
    TextScaleAlignment, TextValue, TextWrapStyle,
};
pub use crate::theme::{Theme, ThemeDescriptor};
pub use crate::time::{TimeDelta, TimeMarker, Timer};
pub use crate::touch::{Touchable, TouchListener};
pub use crate::uniform::Uniform;
pub use crate::visibility::{EnableVisibility, Visibility, VisibleSection};
pub use crate::window::WindowResize;
#[cfg(target_os = "android")]
pub use crate::workflow::AndroidInterface;

mod animate;
mod color;
mod coord;
mod focus;
mod gfx;
mod grid;
mod instance;
mod job;
mod line;
mod orientation;
mod path;
mod render;
mod request;
mod scale_factor;
mod sync;
mod text;
mod theme;
mod time;
mod touch;
mod uniform;
mod viewport;
mod virtual_keyboard;
mod visibility;
mod visualizer;
mod window;
mod workflow;

