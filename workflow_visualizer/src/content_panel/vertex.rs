use bytemuck::{Pod, Zeroable};
use crate::RawPosition;
#[derive(Pod, Zeroable, Copy, Clone, Default)]
pub struct ListenOffset {
    pub listen_x: bool,
    pub listen_y: bool,
}
#[derive(Pod, Zeroable, Copy, Clone, Default)]
pub struct ContentPanelVertex {
    pub position: RawPosition,
    pub listen_offset: ListenOffset,
}