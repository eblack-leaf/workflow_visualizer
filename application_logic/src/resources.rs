use crate::attachment::IconHandles;
use workflow_visualizer::{BundledImageIcon, ImageRequest, Visualizer};

pub(crate) fn add(visualizer: &mut Visualizer) {
    visualizer.spawn(ImageRequest::new(
        IconHandles::Edit.handle(),
        BundledImageIcon::Edit.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Add.handle(),
        BundledImageIcon::Plus.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::PageLeft.handle(),
        BundledImageIcon::ChevronLeft.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::PageRight.handle(),
        BundledImageIcon::ChevronRight.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Run.handle(),
        BundledImageIcon::Activity.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Delete.handle(),
        BundledImageIcon::Delete.data(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Generate.handle(),
        BundledImageIcon::Play.data(),
    ));
}
