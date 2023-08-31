use std::collections::HashMap;

use bevy_ecs::component::Component;

use crate::grid::view::GridMarkerBias;
use crate::{GridLocation, GridMarker, GridPoint, GridRange, GridView, HorizontalSpan};

/// A mapping of GridView for each HorizontalSpan Option
#[derive(Component)]
pub struct ResponsiveView<T> {
    pub mapping: HashMap<HorizontalSpan, T>,
}

impl<F> ResponsiveView<F> {
    pub fn get_span(&self, span: &HorizontalSpan) -> &F {
        self.mapping.get(span).expect("no view")
    }
    pub fn with_span_four<T: Into<F>>(mut self, view: T) -> Self {
        self.mapping.insert(HorizontalSpan::Four, view.into());
        self
    }
    pub fn with_span_eight<T: Into<F>>(mut self, view: T) -> Self {
        self.mapping.insert(HorizontalSpan::Eight, view.into());
        self
    }
    pub fn with_span_twelve<T: Into<F>>(mut self, view: T) -> Self {
        self.mapping.insert(HorizontalSpan::Twelve, view.into());
        self
    }
    pub fn all_same<T: Into<F>>(view: T) -> Self
    where
        F: Clone,
    {
        let value = view.into();
        let mut mapping = HashMap::new();
        mapping.insert(HorizontalSpan::Four, value.clone());
        mapping.insert(HorizontalSpan::Eight, value.clone());
        mapping.insert(HorizontalSpan::Twelve, value);
        Self { mapping }
    }
    pub fn explicit<T: Into<F>>(four: T, eight: T, twelve: T) -> Self {
        let mut mapping = HashMap::new();
        mapping.insert(HorizontalSpan::Four, four.into());
        mapping.insert(HorizontalSpan::Eight, eight.into());
        mapping.insert(HorizontalSpan::Twelve, twelve.into());
        Self { mapping }
    }
}

/// Convenience type for mapping to ContentViews
pub type ResponsiveGridView = ResponsiveView<GridView>;

/// Convenience type for `ResponsiveView<GridPoint>;`
pub type ResponsiveGridPoint = ResponsiveView<GridPoint>;

/// Shorthand for specifying a GridLocation using near/far bias
pub trait ResponsiveUnit {
    fn near(self) -> GridLocation;
    fn far(self) -> GridLocation;
}

impl ResponsiveUnit for i32 {
    fn near(self) -> GridLocation {
        (self, GridMarkerBias::Near).into()
    }
    fn far(self) -> GridLocation {
        (self, GridMarkerBias::Far).into()
    }
}
impl ResponsiveUnit for GridMarker {
    fn near(self) -> GridLocation {
        self.0.near()
    }

    fn far(self) -> GridLocation {
        self.0.far()
    }
}
