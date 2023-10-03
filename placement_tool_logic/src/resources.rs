use workflow_visualizer::{BundledIcon, IconRequest, Visualizer};

#[repr(i32)]
#[derive(Copy, Clone)]
pub(crate) enum ResourceHandles {
    NodeIcon,
    Left,
    Right,
    Add,
    Up,
    Down,
}
pub(crate) fn requests(visualizer: &mut Visualizer) {
    visualizer.spawn(IconRequest::new(
        ResourceHandles::NodeIcon.handle(),
        BundledIcon::Navigation.data(),
    ));
    visualizer.spawn(IconRequest::new(
        ResourceHandles::Left.handle(),
        BundledIcon::ChevronLeft.data(),
    ));
    visualizer.spawn(IconRequest::new(
        ResourceHandles::Right.handle(),
        BundledIcon::ChevronRight.data(),
    ));
    visualizer.spawn(IconRequest::new(
        ResourceHandles::Up.handle(),
        BundledIcon::ChevronUp.data(),
    ));
    visualizer.spawn(IconRequest::new(
        ResourceHandles::Down.handle(),
        BundledIcon::ChevronDown.data(),
    ));
    visualizer.spawn(IconRequest::new(
        ResourceHandles::Add.handle(),
        BundledIcon::PlusSquare.data(),
    ));
}
impl ResourceHandles {
    pub(crate) fn handle(&self) -> workflow_visualizer::ResourceHandle {
        (*self as i32).into()
    }
}
