use bevy_ecs::prelude::{Bundle, Commands, Component, Entity, Query};

/// Wrapper for spawning bundles to ensure lifetime safety for
/// hooking into systems at the right time/place
#[derive(Component)]
pub struct Request<B> {
    pub req: Option<B>,
}

impl<B> Request<B> {
    pub fn new(bi: B) -> Self {
        Self { req: Some(bi) }
    }
}

/// spawner helper to spawn requests added to visualizer
pub fn spawn<B: Bundle>(mut requests: Query<(Entity, &mut Request<B>)>, mut cmd: Commands) {
    for (entity, mut request) in requests.iter_mut() {
        cmd.entity(entity).insert(request.req.take().unwrap());
        cmd.entity(entity).remove::<Request<B>>();
    }
}
