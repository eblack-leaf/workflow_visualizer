#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct Index(pub(crate) usize);

pub(crate) struct Indexer {
    pub(crate) current: usize,
    pub(crate) max: usize,
}

impl Indexer {
    pub(crate) fn new(max: usize) -> Self {
        Self { current: 0, max }
    }
    pub(crate) fn next(&mut self) -> Index {
        self.current += 1;
        Index(self.current)
    }
    pub(crate) fn decrement(&mut self) {
        self.current -= 1;
    }
    pub(crate) fn should_grow(&self) -> bool {
        self.current > self.max
    }
}
