// #![allow(unused, dead_code)]
//! Workflow Visualizer is a lib for cross-platform declarative UI
//! It is comprised of three major parts.
//! trait Workflow to setup application messaging.
//! Visualizer which handles rendering
//! Runner which invokes the visualizer's tools for multiple platforms
pub use animate::{Animate, Animation, Interpolation, InterpolationExtraction, QueuedAnimation};
pub use bevy_ecs;
pub use bundling::{
    despawn, BundleBuilder, BundleExtension, DelayedBundle, DelayedSpawn, Despawn, Despawned,
    Disabled,
};
pub use button::{BackgroundColor, Button, ButtonBorder, ButtonTag, ButtonType};
pub use grid::{
    BundlePlacement, Grid, GridLocation, GridLocationDescriptor, GridMarker, GridMarkerBias,
    GridPoint, GridPointBuilder, GridRange, GridView, GridViewBuilder, HorizontalSpan, Placement,
    RawMarker, RawMarkerGrouping, ResponsiveGridPoint, ResponsiveGridView, ResponsiveUnit,
    ResponsiveView,
};
pub use images::{
    AspectRatioAlignedDimension, Image, ImageFade, ImageHandle, ImageLoaded, ImageOrientations,
    ImageRequest, ImageSizes, ImageTag,
};
pub use instance::{
    AttributeWrite, CpuAttributeBuffer, GpuAttributeBuffer, Index, Indexer,
    InstanceAttributeManager, Key, KeyFactory, NullBit,
};
pub use interaction::{
    ActiveInteraction, Interactable, Interaction, InteractionEvent, InteractionLocation,
    InteractionLocations, InteractionPhase, InteractionPhases, InteractionTracker,
    PrimaryInteraction, PrimaryMouseButton, Toggled, Triggered,
};
pub use job::{Exit, Idle, JobSyncPoint, Tag};
pub use orientation::{AspectRatio, Orientation};
pub use render::{Render, RenderPassHandle, RenderPhase};
pub use viewport::{Viewport, ViewportHandle};
pub use visualizer::{Attach, Attachment, Visualizer};
pub use wgpu;
pub use winit;
pub use workflow::{start_web_worker, Runner, Sender, Workflow};

pub use crate::color::{Color, ColorBuilder, Rgb, Rgba};
pub use crate::coord::{
    area::Area, area::RawArea, layer::Layer, position::Position, position::RawPosition,
    section::Section, Coordinate, DeviceContext, InterfaceContext, NumericalContext,
    WindowAppearanceContext,
};
pub use crate::diagnostics::{Diagnostics, DiagnosticsHandle, Record};
pub use crate::focus::{Focus, FocusInputListener};
pub use crate::gfx::{GfxOptions, GfxSurface};
pub use crate::gfx::{GfxSurfaceConfiguration, MsaaRenderAdapter};
pub use crate::icon::{
    BundledIcon, Icon, IconBitmap, IconBitmapRequest, IconHandle, IconPixelData, IconScale, IconTag,
};
pub use crate::job::Job;
pub use crate::line::{Line, LineRender, LineTag};
pub use crate::panel::{BorderColor, Panel, PanelContentArea, PanelTag, PanelType};
pub use crate::path::{Path, PathView, ResponsivePathView};
pub use crate::scale_factor::{ScaleFactor, WindowAppearanceFactor};
pub use crate::sync::SyncPoint;
pub use crate::text::{
    KnownTextDimension, MonoSpacedFont, Text, TextGridLocation, TextGridPlacement,
    TextLetterDimensions, TextLineStructure, TextScale, TextTag, TextValue, TextWrapStyle,
};
pub use crate::texture_atlas::{
    AtlasBlock, AtlasDimension, AtlasFreeLocations, AtlasPosition, AtlasTexture,
    AtlasTextureDimensions, TextureAtlas, TextureBindGroup, TextureCoordinates,
};
pub use crate::theme::{Theme, ThemeDescriptor};
pub use crate::time::{TimeDelta, TimeMarker, TimeTracker, Timer};
pub use crate::uniform::{AlignedUniform, Uniform};
pub use crate::visibility::{EnableVisibility, Visibility, VisibleSection};
pub use crate::window::WindowResize;
#[cfg(target_os = "android")]
pub use crate::workflow::AndroidInterface;

mod animate;
mod bundling;
mod button;
mod color;
mod coord;
mod diagnostics;
mod focus;
mod gfx;
mod grid;
mod icon;
mod images;
mod instance;
mod interaction;
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
mod uniform;
mod viewport;
mod virtual_keyboard;
mod visibility;
mod visualizer;
mod window;
mod workflow;
