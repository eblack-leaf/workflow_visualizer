use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
pub(crate) struct NullBit {
    bit: u32,
}

impl Default for NullBit {
    fn default() -> Self {
        NullBit::null()
    }
}

impl NullBit {
    pub(crate) const NOT_NULL: u32 = 0u32;
    pub(crate) const NULL: u32 = 1u32;
    fn new(bit: u32) -> Self {
        Self { bit }
    }
    pub(crate) fn not_null() -> NullBit {
        Self::new(Self::NOT_NULL)
    }
    pub(crate) fn null() -> Self {
        Self::new(Self::NULL)
    }
}
