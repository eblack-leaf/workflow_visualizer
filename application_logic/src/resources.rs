use crate::attachment::IconHandles;
use workflow_visualizer::{BundledImageIcon, ImageRequest, Visualizer};
pub(crate) const TEST_HANDLE: i32 = 0;

pub(crate) fn add(visualizer: &mut Visualizer) {
    visualizer.spawn(ImageRequest::new(
        IconHandles::Edit.handle(),
        BundledImageIcon::AlignJustify.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Add.handle(),
        BundledImageIcon::AlertTriangle.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::PageLeft.handle(),
        BundledImageIcon::ArrowLeft.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::PageRight.handle(),
        BundledImageIcon::ArrowRight.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Run.handle(),
        BundledImageIcon::Activity.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Delete.handle(),
        BundledImageIcon::Airplay.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Generate.handle(),
        BundledImageIcon::AlertCircle.data(),
    ));
}
