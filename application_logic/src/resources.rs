use crate::attachment::IconHandles;
use workflow_visualizer::BundledIcon;
use workflow_visualizer::{IconRequest, Visualizer};

pub(crate) fn add(visualizer: &mut Visualizer) {
    visualizer.spawn(IconRequest::new(
        IconHandles::Edit.handle(),
        BundledIcon::EditThree.data(),
    ));
    visualizer.spawn(IconRequest::new(
        IconHandles::Add.handle(),
        BundledIcon::Plus.data(),
    ));
    visualizer.spawn(IconRequest::new(
        IconHandles::PageLeft.handle(),
        BundledIcon::ChevronLeft.data(),
    ));
    visualizer.spawn(IconRequest::new(
        IconHandles::PageRight.handle(),
        BundledIcon::ChevronRight.data(),
    ));
    visualizer.spawn(IconRequest::new(
        IconHandles::Run.handle(),
        BundledIcon::Activity.data(),
    ));
    visualizer.spawn(IconRequest::new(
        IconHandles::Delete.handle(),
        BundledIcon::Trash.data(),
    ));
    visualizer.spawn(IconRequest::new(
        IconHandles::Generate.handle(),
        BundledIcon::Code.data(),
    ));
}
