use crate::attachment::IconHandles;
use workflow_visualizer::{ImageRequest, Visualizer};
pub(crate) const TEST_HANDLE: i32 = 0;

pub(crate) fn add(visualizer: &mut Visualizer) {
    visualizer.spawn(ImageRequest::new(
        IconHandles::Edit.handle(),
        include_bytes!("marker.png").to_vec(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Add.handle(),
        include_bytes!("marker.png").to_vec(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::PageLeft.handle(),
        include_bytes!("marker.png").to_vec(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::PageRight.handle(),
        include_bytes!("marker.png").to_vec(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Run.handle(),
        include_bytes!("marker.png").to_vec(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Delete.handle(),
        include_bytes!("marker.png").to_vec(),
    ));
    visualizer.spawn(ImageRequest::new(
        IconHandles::Generate.handle(),
        include_bytes!("marker.png").to_vec(),
    ));
}
