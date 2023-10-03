use bevy_ecs::component::Component;
use bevy_ecs::prelude::{Bundle, Resource};

pub(crate) use attachment::GridAttachment;
use marker::ColumnConfig;
use marker::GutterConfig;
pub use marker::RawMarker;
pub use marker::RawMarkerGrouping;
use marker::RowConfig;
pub use responsive::{ResponsiveGridPoint, ResponsiveGridView, ResponsiveUnit, ResponsiveView};

pub use view::{
    GridLocation, GridLocationDescriptor, GridLocationOffset, GridMarker, GridMarkerBias,
    GridPoint, GridPointBuilder, GridRange, GridView, GridViewBuilder, Placement,
};

use crate::bundling::{BundleBuilder, BundleExtension};

use crate::{Area, InterfaceContext, ResponsivePathView, Section};

mod attachment;
mod marker;
mod responsive;
mod system;
mod view;

/// Span used for setting the number of columns available in the Grid
#[derive(Resource, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum HorizontalSpan {
    Four,
    Eight,
    Twelve,
}

impl HorizontalSpan {
    pub fn gutter_base(&self) -> RawMarkerGrouping {
        match self {
            HorizontalSpan::Four => RawMarkerGrouping(4),
            HorizontalSpan::Eight => RawMarkerGrouping(4),
            HorizontalSpan::Twelve => RawMarkerGrouping(6),
        }
    }
    pub fn content_base(&self) -> RawMarkerGrouping {
        match self {
            HorizontalSpan::Four => RawMarkerGrouping(20),
            HorizontalSpan::Eight => RawMarkerGrouping(18),
            HorizontalSpan::Twelve => RawMarkerGrouping(20),
        }
    }
    pub const SMALL_BREAKPOINT: f32 = 720f32;
    pub const MEDIUM_BREAKPOINT: f32 = 1168f32;
}

/// Grid configuration of the Span + Column/Row/Gutter Configs
#[derive(Resource)]
pub struct Grid {
    pub(crate) span: HorizontalSpan,
    pub(crate) column_config: ColumnConfig,
    #[allow(unused)]
    pub(crate) row_config: RowConfig,
    pub(crate) gutter_config: GutterConfig,
    pub(crate) vertical_markers: u32,
    pub(crate) horizontal_markers: u32,
}
impl Grid {
    pub(crate) const SPAN_FOUR_EXT_BASE: f32 = 400f32;
    pub(crate) const SPAN_EIGHT_EXT_BASE: f32 = 720f32;
    pub(crate) const SPAN_TWELVE_EXT_BASE: f32 = 1168f32;
    pub(crate) const SPAN_TWELVE_COLUMNS: i32 = 12;
    pub(crate) const SPAN_EIGHT_COLUMNS: i32 = 8;
    pub(crate) const SPAN_FOUR_COLUMNS: i32 = 4;
    pub(crate) fn new(area: Area<InterfaceContext>) -> Self {
        let (span, extension) = {
            if area.width > HorizontalSpan::MEDIUM_BREAKPOINT {
                let extension = Self::calc_extension(
                    area.width,
                    Self::SPAN_TWELVE_EXT_BASE,
                    Self::SPAN_TWELVE_COLUMNS,
                );
                (HorizontalSpan::Twelve, RawMarkerGrouping(extension))
            } else if area.width > HorizontalSpan::SMALL_BREAKPOINT {
                let extension = Self::calc_extension(
                    area.width,
                    Self::SPAN_EIGHT_EXT_BASE,
                    Self::SPAN_EIGHT_COLUMNS,
                );
                (HorizontalSpan::Eight, RawMarkerGrouping(extension))
            } else {
                let extension = Self::calc_extension(
                    area.width,
                    Self::SPAN_FOUR_EXT_BASE,
                    Self::SPAN_FOUR_COLUMNS,
                );
                (HorizontalSpan::Four, RawMarkerGrouping(extension))
            }
        };
        Self {
            span,
            column_config: ColumnConfig {
                base: span.content_base(),
                extension,
            },
            row_config: RowConfig {
                base: span.content_base(),
            },
            gutter_config: GutterConfig {
                base: span.gutter_base(),
            },
            horizontal_markers: RawMarker::from_pixel_exclusive(area.width).0 as u32,
            vertical_markers: RawMarker::from_pixel_exclusive(area.height).0 as u32,
        }
    }
    pub fn percent_column(&self, percent: f32) -> RawMarker {
        ((self.markers_per_column() as f32 * percent).floor() as i32).into()
    }
    pub fn view_horizontal_markers(&self, view: &GridView) -> RawMarker {
        let end = self.calc_horizontal_location(view.horizontal.end);
        let begin = self.calc_horizontal_location(view.horizontal.begin);
        end - begin
    }
    pub fn view_vertical_markers(&self, view: &GridView) -> RawMarker {
        let end = self.calc_vertical_location(view.vertical.end);
        let begin = self.calc_vertical_location(view.vertical.begin);
        end - begin
    }
    pub fn span(&self) -> HorizontalSpan {
        self.span
    }
    pub fn distance(&self, first: GridPoint, second: GridPoint) -> (RawMarker, RawMarker) {
        let fx = self.calc_horizontal_location(first.x);
        let fy = self.calc_vertical_location(first.y);
        let sx = self.calc_horizontal_location(second.x);
        let sy = self.calc_vertical_location(second.y);
        (fx - sx, fy - sy)
    }
    pub fn horizontal_markers(&self) -> i32 {
        self.horizontal_markers as i32
    }
    pub fn vertical_markers(&self) -> i32 {
        self.vertical_markers as i32
    }
    pub fn last_full_row(&self) -> GridMarker {
        (self.vertical_markers() / (self.markers_per_row() + self.markers_per_gutter())).into()
    }
    fn calc_extension(width: f32, base: f32, columns: i32) -> i32 {
        RawMarker::from_pixel_exclusive(width - base).0 / columns
    }
    pub fn calc_section_from_responsive(
        &self,
        view: &ResponsiveGridView,
    ) -> Section<InterfaceContext> {
        let current_view = view.mapping.get(&self.span).expect("view mapping");
        self.calc_section(current_view)
    }
    pub fn calc_section(&self, current_view: &GridView) -> Section<InterfaceContext> {
        let left = {
            let grid_location = current_view.horizontal.begin;
            self.calc_horizontal_location(grid_location)
        };
        let top = {
            let grid_location = current_view.vertical.begin;
            self.calc_vertical_location(grid_location)
        };
        let right = {
            let grid_location = current_view.horizontal.end;
            self.calc_horizontal_location(grid_location)
        };
        let bottom = {
            let grid_location = current_view.vertical.end;
            self.calc_vertical_location(grid_location)
        };
        Section::from_left_top_right_bottom(
            left.to_pixel(),
            top.to_pixel(),
            right.to_pixel(),
            bottom.to_pixel(),
        )
    }
    pub fn markers_per_column(&self) -> i32 {
        self.column_config.base.0 + self.column_config.extension.0
    }
    pub fn markers_per_row(&self) -> i32 {
        self.markers_per_column()
    }
    pub fn markers_per_gutter(&self) -> i32 {
        self.gutter_config.base.0
    }
    pub fn calc_horizontal_location(&self, grid_location: GridLocation) -> RawMarker {
        let markers_per_column = self.markers_per_column();
        let content_location = grid_location.location;
        let location = content_location.marker.0 * markers_per_column
            + self.gutter_config.base.0 * content_location.marker.0;
        let location =
            if content_location.bias == GridMarkerBias::Near && content_location.marker.0 != 0 {
                location - markers_per_column
            } else {
                location
            };
        let location = if let Some(offset) = grid_location.offset {
            location + offset.0 .0
        } else {
            location
        };
        location.into()
    }
    pub fn calc_vertical_location(&self, grid_location: GridLocation) -> RawMarker {
        let content_location = grid_location.location;
        let markers_per_row = self.markers_per_row();
        let location = content_location.marker.0 * markers_per_row
            + self.gutter_config.base.0 * content_location.marker.0;
        let location =
            if content_location.bias == GridMarkerBias::Near && content_location.marker.0 != 0 {
                location - markers_per_row
            } else {
                location
            };
        let location = if let Some(offset) = grid_location.offset {
            location + offset.0 .0
        } else {
            location
        };
        location.into()
    }
}
#[derive(Component, Copy, Clone)]
pub struct AbsolutePlacement(pub Section<InterfaceContext>);
pub trait BundlePlacement
where
    Self: Sized + Bundle,
{
    fn responsively_viewed<R: Into<ResponsiveGridView>>(
        self,
        view: R,
    ) -> BundleBuilder<Self, ResponsiveGridView> {
        self.extend(view.into())
    }
    fn responsively_point_viewed<R: Into<ResponsiveGridPoint>>(
        self,
        view: R,
    ) -> BundleBuilder<Self, ResponsiveGridPoint> {
        self.extend(view.into())
    }
    fn absolute<S: Into<Section<InterfaceContext>>>(
        self,
        section: S,
    ) -> BundleBuilder<Self, AbsolutePlacement> {
        self.extend(AbsolutePlacement(section.into()))
    }
    fn responsively_path_viewed<P: Into<ResponsivePathView>>(
        self,
        view: P,
    ) -> BundleBuilder<Self, ResponsivePathView> {
        self.extend(view.into())
    }
}
impl<T: Bundle + Sized> BundlePlacement for T {}
