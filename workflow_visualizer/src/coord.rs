pub trait CoordContext
where
    Self: Send + Sync + 'static + Copy + Clone,
{
}
#[derive(Copy, Clone, PartialOrd, PartialEq, Default)]
pub struct DeviceContext;
#[derive(Copy, Clone, PartialOrd, PartialEq, Default)]
pub struct InterfaceContext;
#[derive(Copy, Clone, PartialOrd, PartialEq, Default)]
pub struct NumericalContext;
impl CoordContext for DeviceContext {}
impl CoordContext for InterfaceContext {}
impl CoordContext for NumericalContext {}
