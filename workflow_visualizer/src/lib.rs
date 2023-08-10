#![allow(unused, dead_code)]
//! Workflow Visualizer is a lib for cross-platform declarative UI
//! It is comprised of three major parts.
//! trait Workflow to setup application messaging.
//! Visualizer which handles rendering
//! Runner which invokes the visualizer's tools for multiple platforms
pub use bevy_ecs;
pub use wgpu;
pub use winit;

pub use animation::{InterpolationExtraction, Interpolator};
pub use bundling::{BundleBuilder, BundleExtension};
pub use button::{BackgroundColor, Button, ButtonBorder, ButtonDespawn, ButtonTag, ButtonType};
pub use grid::GridLocation;
pub use grid::GridLocationDescriptor;
pub use grid::GridMarker;
pub use grid::GridMarkerBias;
pub use grid::GridPoint;
pub use grid::GridRange;
pub use grid::GridView;
pub use grid::RawMarker;
pub use grid::RawMarkerGrouping;
pub use grid::ResponsiveGridPoint;
pub use grid::ResponsiveGridView;
pub use grid::ResponsiveUnit;
pub use grid::ResponsiveView;
pub use instance::{
    AttributeWrite, CpuAttributeBuffer, GpuAttributeBuffer, Index, Indexer,
    InstanceAttributeManager, Key, KeyFactory, NullBit,
};
pub use job::{Exit, Idle, JobSyncPoint, Tag};
pub use render::{Render, RenderPassHandle, RenderPhase};
pub use touch::CurrentlyPressed;
pub use touch::PrimaryTouch;
pub use touch::ToggleState;
pub use touch::Touchable;
pub use touch::TouchListener;
pub use touch::TouchLocation;
pub use touch::TouchTrigger;
pub use viewport::{Viewport, ViewportHandle};
pub use visualizer::{Attach, Attachment, Visualizer};
pub use workflow::{Runner, Sender, start_web_worker, Workflow};

pub use crate::color::Color;
pub use crate::coord::{
    area::Area, area::RawArea, Coordinate, DeviceContext, InterfaceContext,
    layer::Layer, NumericalContext, position::Position, position::RawPosition, section::Section,
};
pub use crate::diagnostics::{Diagnostics, DiagnosticsHandle, Record};
pub use crate::disable::Disabled;
pub use crate::focus::{Focus, FocusInputListener};
pub use crate::gfx::{GfxOptions, GfxSurface};
pub use crate::gfx::{GfxSurfaceConfiguration, MsaaRenderAdapter};
pub use crate::grid::{BundlePlacement, Grid, HorizontalSpan, PlacementReference};
pub use crate::icon::{
    BundledIcon, Icon, IconBitmap, IconBitmapRequest, IconId, IconPixelData, IconScale, IconTag,
};
pub use crate::job::Job;
pub use crate::line::{Line, LineRender, LineTag};
pub use crate::panel::{BorderColor, Panel, PanelContentArea, PanelTag, PanelType};
pub use crate::path::{Path, PathView, ResponsivePathView};
pub use crate::scale_factor::ScaleFactor;
pub use crate::sync::SyncPoint;
pub use crate::text::{
    Text, TextGridLocation, TextGridPlacement, TextLetterDimensions, TextLineStructure, TextScale,
    TextScaleAlignment, TextTag, TextValue, TextWrapStyle,
};
pub use crate::texture_atlas::{
    AtlasBindGroup, AtlasBlock, AtlasDimension, AtlasFreeLocations, AtlasPosition, AtlasTexture,
    AtlasTextureDimensions, TextureAtlas, TextureCoordinates,
};
pub use crate::theme::{Theme, ThemeDescriptor};
pub use crate::time::{TimeDelta, TimeMarker, Timer};
pub use crate::uniform::Uniform;
pub use crate::visibility::{EnableVisibility, Visibility, VisibleSection};
pub use crate::window::WindowResize;
#[cfg(target_os = "android")]
pub use crate::workflow::AndroidInterface;

mod animation;
mod bundling;
mod button;
mod color;
mod coord;
mod diagnostics;
mod disable;
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
