// const Info for stability in shaders and programs
// application level layout for bindings
pub struct BindingInfo {
    pub binding: u32,
}
pub const GPU_VIEWPORT: BindingInfo = BindingInfo { binding: 0 };
