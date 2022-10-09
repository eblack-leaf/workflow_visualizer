// const Info for stability in shaders and programs
// application level layout for bindings
pub mod bindings {
    pub const VIEWPORT: u32 = 0;
    pub const RASTERIZATION: u32 = 1;
}
pub mod shader_locations {}
pub mod buffers {
    pub const TEXT_VERTEX: u32 = 0;
    pub const TEXT_INSTANCE: u32 = 1;
}
pub mod attributes {
    pub const TEXT_VERTEX: u32 = 0;
    pub const TEXT_INSTANCE: u32 = 1;
    pub const TEXT_COLOR: u32 = 0;
    pub const TEXT_POSITION: u32 = 1;
    pub const TEXT_AREA: u32 = 2;
    pub const TEXT_DEPTH: u32 = 3;
}
