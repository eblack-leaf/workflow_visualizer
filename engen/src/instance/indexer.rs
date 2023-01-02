#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub(crate) struct Index(pub(crate) usize);
#[derive(Copy, Clone)]
pub(crate) struct IndexedAttribute<
    Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
> {
    pub(crate) index: Index,
    pub(crate) attribute: Attribute,
}
impl<
        Attribute: bytemuck::Pod + bytemuck::Zeroable + Copy + Clone + Send + Sync + Default + PartialEq,
    > IndexedAttribute<Attribute>
{
    pub(crate) fn new(index: Index, attribute: Attribute) -> Self {
        Self { index, attribute }
    }
}
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
        Self::index(self.current)
    }
    pub(crate) fn decrement(&mut self) -> Index {
        let last = self.current;
        self.current -= 1;
        Self::index(last)
    }
    pub(crate) fn should_grow(&self) -> bool {
        self.current > self.max
    }
    pub(crate) fn grow(&mut self, growth_factor: usize) -> usize {
        while self.max < self.current {
            self.max += growth_factor;
        }
        self.max.abs_diff(self.current)
    }
    fn index(val: usize) -> Index {
        Index(val - 1)
    }
}
