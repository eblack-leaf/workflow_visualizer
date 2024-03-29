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
    Disabled, ResourceHandle,
};
pub use button::{
    BackgroundColor, Button, ButtonBorder, ButtonIcon, ButtonTag, ButtonText, ButtonType,
};
pub use icon::{BundledIcon, Icon, IconRequest, IconScale, IconTag};
pub use images::{
    AspectRatioAlignedDimension, Image, ImageData, ImageFade, ImageLoaded, ImageOrientations,
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
pub use scene::Scene;
pub use snap_grid::{
    FloatArrangement, FloatLocation, FloatPlacementDescriptor, FloatPlacer, FloatRange, FloatView,
    GridBias, GridDirection, GridLocation, GridMarker, GridPoint, GridRange, GridUnit, GridView,
    ResponsiveGridLocation, ResponsiveGridPoint, ResponsiveGridRange, ResponsiveGridView,
    ResponsiveUnit,
};
pub use viewport::{Viewport, ViewportHandle};
pub use visualizer::{Attach, Attachment, Visualizer};
pub use wgpu;
pub use winit;
pub use workflow::{start_web_worker, NoOp, Runner, Sender, Workflow};

pub use crate::clipboard::Clipboard;
pub use crate::color::{Color, ColorBuilder, Rgb, Rgba};
pub use crate::coord::{
    area::Area, area::RawArea, layer::Layer, position::Position, position::RawPosition,
    section::Section, Coordinate, CoordinateUnit, DeviceContext, InterfaceContext,
    NumericalContext, WindowAppearanceContext,
};
pub use crate::diagnostics::{Diagnostics, DiagnosticsHandle, Record};
pub use crate::focus::{Focus, FocusInputListener};
pub use crate::gfx::{GfxOptions, GfxSurface};
pub use crate::gfx::{GfxSurfaceConfiguration, MsaaRenderAdapter};
pub use crate::job::Job;
pub use crate::layer_compositor::{LayerArrangement, LayerCompositor};
pub use crate::line::{Line, LineRender, LineTag};
pub use crate::media::Media;
pub use crate::panel::{BorderColor, Panel, PanelContentArea, PanelTag, PanelType};
pub use crate::path::{Path, PathView};
pub use crate::scale_factor::{ScaleFactor, WindowAppearanceFactor};
pub use crate::snap_grid::{Column, Row, SnapGrid};
pub use crate::sync::SyncPoint;
pub use crate::text::{
    KnownTextDimension, MonoSpacedFont, Text, TextGridLocation, TextGridPlacement,
    TextLetterDimensions, TextLineStructure, TextScale, TextSectionDescriptor,
    TextSectionDescriptorKnown, TextTag, TextValue, TextWrapStyle,
};
pub use crate::texture_atlas::{
    AtlasBlock, AtlasDimension, AtlasFreeLocations, AtlasPosition, AtlasTexture,
    AtlasTextureDimensions, TextureAtlas, TextureBindGroup, TextureCoordinates,
};
pub use crate::theme::{Theme, ThemeDescriptor};
pub use crate::time::{TimeDelta, TimeMarker, TimeTracker, Timer};
pub use crate::uniform::{AlignedUniform, Uniform};
pub use crate::visibility::{EnableVisibility, Visibility, VisibleSection};
pub use crate::visual_debug::SectionOutline;
pub use crate::window::WindowResize;
#[cfg(target_os = "android")]
pub use crate::workflow::AndroidInterface;

mod animate;
mod bundling;
mod button;
mod clipboard;
mod color;
mod coord;
mod diagnostics;
mod focus;
mod gfx;
mod icon;
mod images;
mod instance;
mod interaction;
mod job;
mod layer_compositor;
mod line;
mod media;
mod orientation;
mod panel;
mod path;
mod render;
mod scale_factor;
mod scene;
mod snap_grid;
mod sync;
mod text;
mod texture_atlas;
mod theme;
mod time;
mod uniform;
mod viewport;
mod virtual_keyboard;
mod visibility;
mod visual_debug;
mod visualizer;
mod window;
mod workflow;
