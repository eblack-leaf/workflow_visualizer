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
pub use workflow::{start_web_worker, Runner, Sender, Workflow};

pub use crate::animate::Interpolator;
pub use crate::color::Color;
pub use crate::coord::{
    area::Area, area::RawArea, layer::Layer, position::Position, position::RawPosition,
    section::Section, Coordinate, DeviceContext, InterfaceContext, NumericalContext,
};
pub use crate::diagnostics::{Diagnostics, DiagnosticsHandle, Record};
pub use crate::focus::{Focus, FocusInputListener};
pub use crate::gfx::{GfxOptions, GfxSurface};
pub use crate::gfx::{GfxSurfaceConfiguration, MsaaRenderAdapter};
pub use crate::grid::{
    AbsoluteBundle, BundlePlacement, Grid, GridLocation, GridLocationDescriptor, GridMarker,
    GridMarkerBias, GridPoint, GridRange, GridView, HorizontalSpan, RawMarker, RawMarkerGrouping,
    ResponsiveBundle, ResponsiveGridPoint, ResponsiveGridView, ResponsivePointBundle,
    ResponsiveUnit, ResponsiveView,
};
pub use crate::icon::{
    BundledIcon, Icon, IconBitmap, IconBitmapRequest, IconId, IconPixelData, IconScale,
};
pub use crate::job::{EntityName, EntityStore, Job};
pub use crate::line::{Line, LineRender};
pub use crate::panel::{BorderColor, Panel, PanelColor, PanelContentArea, PanelType};
pub use crate::path::{Path, PathView, ResponsivePathView};
pub use crate::scale_factor::ScaleFactor;
pub use crate::sync::{SyncPoint, UserSpaceSyncPoint};
pub use crate::text::{
    Text, TextGridLocation, TextGridPlacement, TextLetterDimensions, TextLineStructure, TextScale,
    TextScaleAlignment, TextValue, TextWrapStyle,
};
pub use crate::texture_atlas::{
    AtlasBindGroup, AtlasBlock, AtlasDimension, AtlasFreeLocations, AtlasPosition, AtlasTexture,
    AtlasTextureDimensions, TextureAtlas, TextureCoordinates,
};
pub use crate::theme::{Theme, ThemeDescriptor};
pub use crate::time::{TimeDelta, TimeMarker, Timer};
pub use crate::touch::{
    CurrentlyPressed, PrimaryTouch, ToggleState, TouchListener, TouchLocation, TouchTrigger,
    Touchable,
};
pub use crate::uniform::Uniform;
pub use crate::visibility::{EnableVisibility, Visibility, VisibleSection};
pub use crate::window::WindowResize;
#[cfg(target_os = "android")]
pub use crate::workflow::AndroidInterface;

mod animate;
mod button;
mod color;
mod coord;
mod diagnostics;
mod focus;
mod gfx;
mod grid;
mod icon;
mod instance;
mod job;
mod line;
mod orientation;
mod panel;
mod path;
mod render;
mod scale_factor;
mod sync;
mod text;
mod texture_atlas;
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
