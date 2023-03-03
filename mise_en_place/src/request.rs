use bevy_ecs::prelude::{Bundle, Commands, Component, Entity, Query};

#[derive(Component)]
pub struct Request<B: Bundle> {
    pub req: Option<B>,
}

impl<B: Bundle> Request<B> {
    pub fn new(bi: B) -> Self {
        Self {
            req: Some(bi.into()),
        }
    }
}

// to be run in spawn to align all creation
pub fn spawn<B: Bundle>(mut requests: Query<(Entity, &mut Request<B>)>, mut cmd: Commands) {
    for (entity, mut request) in requests.iter_mut() {
        println!("spawning");
        cmd.entity(entity).insert(request.req.take().unwrap());
        cmd.entity(entity).remove::<Request<B>>();
    }
}
