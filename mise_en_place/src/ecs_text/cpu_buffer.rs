use bevy_ecs::prelude::Component;

#[derive(Component)]
pub(crate) struct CpuBuffer<T: Default + Clone> {
    pub(crate) buffer: Vec<T>,
}

impl<T: Default + Clone> CpuBuffer<T> {
    pub(crate) fn new(max: u32) -> Self {
        Self {
            buffer: {
                let mut vec = Vec::new();
                vec.resize(max as usize, T::default());
                vec
            },
        }
    }
}
