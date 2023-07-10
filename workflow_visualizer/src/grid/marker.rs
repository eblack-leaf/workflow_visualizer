/// Index of 8px alignment location
#[derive(Copy, Clone, PartialEq)]
pub struct RawMarker(pub i32);

impl RawMarker {
    pub const PX: f32 = 4f32;
    pub fn to_pixel(&self) -> f32 {
        self.0 as f32 * Self::PX
    }
}

impl From<i32> for RawMarker {
    fn from(value: i32) -> Self {
        RawMarker(value)
    }
}

/// Number of markers to include in a logical group
pub struct RawMarkerGrouping(pub i32);

pub(crate) struct ColumnConfig {
    pub base: RawMarkerGrouping,
    pub extension: RawMarkerGrouping,
}

/// MarkerGrouping for deciding gutter size
pub(crate) struct GutterConfig {
    pub base: RawMarkerGrouping,
}

/// MarkerGrouping fro deciding row size
pub(crate) struct RowConfig {
    pub base: RawMarkerGrouping,
}
