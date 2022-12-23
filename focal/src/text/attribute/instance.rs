use crate::text::attribute::Coordinator;

pub(crate) struct Indexer {
    pub(crate) current: u32,
    pub(crate) max: u32,
}
impl Indexer {
    pub(crate) fn new(max: u32) -> Self {
        Self { current: 0, max }
    }
    pub(crate) fn next(&mut self) -> Instance {
        self.current += 1;
        Instance(self.current)
    }
    pub(crate) fn decrement(&mut self) {
        self.current -= 1;
    }
}
#[derive(Eq, Hash, PartialEq)]
pub(crate) struct Instance(pub(crate) u32);
