#[repr(i32)]
#[derive(Copy, Clone)]
pub(crate) enum ResourceHandles {
    NodeIcon,
}
impl ResourceHandles {
    pub(crate) fn handle(&self) -> workflow_visualizer::ResourceHandle {
        (*self as i32).into()
    }
}
