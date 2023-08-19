use bevy_ecs::prelude::Bundle;

#[derive(Bundle)]
pub struct BundleBuilder<T: Bundle, S: Bundle> {
    pub original: T,
    pub extension: S,
}

impl<T: Bundle, S: Bundle> BundleBuilder<T, S> {
    pub fn new(t: T, s: S) -> Self {
        Self {
            original: t,
            extension: s,
        }
    }
}

pub trait BundleExtension
where
    Self: Bundle + Sized,
{
    fn extend<E: Bundle>(self, handle: E) -> BundleBuilder<Self, E>;
}

impl<I: Bundle> BundleExtension for I {
    fn extend<E: Bundle>(self, handle: E) -> BundleBuilder<I, E> {
        BundleBuilder::new(self, handle)
    }
}
