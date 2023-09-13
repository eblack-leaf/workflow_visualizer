use crate::attachment::IconHandles;
use workflow_visualizer::{BundledImageIcon, ImageRequest, Visualizer};
pub(crate) const TEST_HANDLE: i32 = 0;

pub(crate) fn add(visualizer: &mut Visualizer) {
    visualizer.spawn(ImageRequest::new(
        IconHandles::Edit.handle(),
        BundledImageIcon::Pencil.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Add.handle(),
        BundledImageIcon::Plus.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::PageLeft.handle(),
        BundledImageIcon::AngleLeft.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::PageRight.handle(),
        BundledImageIcon::AngleRight.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Run.handle(),
        BundledImageIcon::Plus.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Delete.handle(),
        BundledImageIcon::AngleRight.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Generate.handle(),
        BundledImageIcon::AngleLeft.data(),
    ));
}
