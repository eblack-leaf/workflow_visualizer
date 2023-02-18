#[derive(Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Debug)]
pub struct Key {
    identifier: u32,
}

impl Key {
    pub(crate) fn new(identifier: u32) -> Self {
        Self { identifier }
    }
    fn identifier(&self) -> u32 {
        self.identifier
    }
}

pub struct KeyFactory {
    current: u32,
}

impl KeyFactory {
    pub fn new() -> Self {
        Self { current: 0 }
    }
    pub fn generate(&mut self) -> Key {
        let key = Key::new(self.current);
        self.current += 1;
        key
    }
}
