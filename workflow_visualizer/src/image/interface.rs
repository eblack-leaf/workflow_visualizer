use crate::image::renderer::{ImageFade, ImageName};
use crate::{
    Area, Disabled, EnableVisibility, InterfaceContext, Layer, Position, Section, Tag, Visibility,
};
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{
    Added, Bundle, Changed, Component, Query, RemovedComponents, Resource, With, Without,
};
use bevy_ecs::system::ResMut;
use std::collections::{HashMap, HashSet};
pub type ImageTag = Tag<Image>;
#[derive(Bundle)]
pub struct Image {
    coordinate: Section<InterfaceContext>,
    layer: Layer,
    visibility: EnableVisibility,
    name: ImageName,
    fade: ImageFade,
    cache: Cache,
    difference: Difference,
    tag: ImageTag,
}
#[derive(Component)]
pub(crate) struct Cache {
    name: ImageName,
    fade: ImageFade,
    pos: Position<InterfaceContext>,
    area: Area<InterfaceContext>,
    layer: Layer,
}
#[derive(Component, Clone, Default)]
pub(crate) struct Difference {
    pub(crate) name: Option<ImageName>,
    pub(crate) fade: Option<ImageFade>,
    pub(crate) pos: Option<Position<InterfaceContext>>,
    pub(crate) area: Option<Area<InterfaceContext>>,
    pub(crate) layer: Option<Layer>,
}
pub(crate) fn name_diff(
    mut images: Query<(&ImageName, &mut Cache, &mut Difference), Changed<ImageName>>,
) {
    for (name, mut cache, mut difference) in images.iter_mut() {
        if name.0 != cache.name.0 {
            cache.0 = name.0;
            difference.name.replace(name.clone());
        }
    }
}
pub(crate) fn fade_diff(
    mut images: Query<(&ImageFade, &mut Cache, &mut Difference), Changed<ImageFade>>,
) {
    for (fade, mut cache, mut difference) in images.iter_mut() {
        if *fade != cache.fade {
            difference.fade.replace(*fade);
            cache.fade = *fade;
        }
    }
}
pub(crate) fn pos_diff(
    mut images: Query<
        (&Position<InterfaceContext>, &mut Cache, &mut Difference),
        Changed<Position<InterfaceContext>>,
    >,
) {
    for (pos, mut cache, mut difference) in images.iter_mut() {
        if *pos != cache.pos {
            cache.pos = *pos;
            difference.pos.replace(*pos);
        }
    }
}
pub(crate) fn area_diff(
    mut images: Query<
        (&Area<InterfaceContext>, &mut Cache, &mut Difference),
        Changed<Area<InterfaceContext>>,
    >,
) {
    for (area, mut cache, mut difference) in images.iter_mut() {
        if *area != cache.area {
            cache.area = *area;
            difference.area.replace(*area);
        }
    }
}
pub(crate) fn layer_diff(mut images: Query<(&Layer, &mut Cache, &mut Difference), Changed<Layer>>) {
    for (layer, mut cache, mut difference) in images.iter_mut() {
        if *layer != cache.layer {
            cache.layer = *layer;
            difference.layer.replace(*layer);
        }
    }
}
#[derive(Resource, Default)]
pub(crate) struct Extraction {
    pub(crate) differences: HashMap<Entity, Difference>,
    pub(crate) queued_remove: HashSet<Entity>,
}
impl Extraction {
    pub(crate) fn remove(&mut self, entity: Entity) {
        self.queued_remove.insert(entity);
        self.differences.remove(&entity);
    }
}
pub(crate) fn management(
    mut images: Query<
        (
            Entity,
            &Position<InterfaceContext>,
            &Area<InterfaceContext>,
            &Layer,
            &ImageName,
            &ImageFade,
            &Visibility,
            &mut Cache,
            &mut Difference,
        ),
        Changed<Visibility>,
    >,
    mut removed: RemovedComponents<ImageTag>,
    mut extraction: ResMut<Extraction>,
) {
    for (entity, pos, area, layer, name, fade, visibility, mut cache, mut difference) in
        images.iter_mut()
    {
        if visibility.visible() {
            cache.pos = *pos;
            cache.area = *area;
            cache.layer = *layer;
            cache.fade = *fade;
            cache.name.0 = name.0;
            difference.pos.replace(cache.pos);
            difference.area.replace(cache.area);
            difference.layer.replace(cache.layer);
            difference.fade.replace(cache.fade);
            difference.name.replace(cache.name.clone());
        } else {
            extraction.remove(entity);
        }
    }
    for entity in removed.iter() {
        extraction.remove(entity);
    }
}
pub(crate) fn extract(
    mut extraction: ResMut<Extraction>,
    mut images: Query<
        (Entity, &mut Difference, &Visibility),
        (Changed<Difference>, Without<Disabled>),
    >,
) {
    for (entity, mut diff, visibility) in images.iter_mut() {
        if visibility.visible() {
            extraction.differences.insert(entity, diff.clone());
        }
        *diff = Difference::default();
    }
}
