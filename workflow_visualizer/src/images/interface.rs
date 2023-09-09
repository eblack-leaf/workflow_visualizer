use std::collections::{HashMap, HashSet};

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{
    Bundle, Changed, Component, Or, Query, RemovedComponents, Res, Resource, Without,
};
use bevy_ecs::system::ResMut;

use crate::images::renderer::{ImageFade, ImageHandle, ImageOrientations};
use crate::{
    Area, Disabled, EnableVisibility, InterfaceContext, Layer, Orientation, Position, Section, Tag,
    Visibility,
};

pub type ImageTag = Tag<Image>;
#[derive(Bundle)]
pub struct Image {
    section: Section<InterfaceContext>,
    layer: Layer,
    visibility: EnableVisibility,
    name: ImageHandle,
    fade: ImageFade,
    cache: Cache,
    difference: Difference,
    tag: ImageTag,
}
impl Image {
    pub fn new<IN: Into<ImageHandle>, L: Into<Layer>, IF: Into<ImageFade>>(
        name: IN,
        layer: L,
        fade: IF,
    ) -> Self {
        Self {
            section: Section::default(),
            layer: layer.into(),
            visibility: EnableVisibility::default(),
            name: name.into(),
            fade: fade.into(),
            cache: Cache::default(),
            difference: Difference::default(),
            tag: ImageTag::new(),
        }
    }
}
#[derive(Component, Copy, Clone)]
pub struct AspectRatioAlignedDimension {
    pub dimension: Area<InterfaceContext>,
}
impl AspectRatioAlignedDimension {
    pub fn new<A: Into<Area<InterfaceContext>>>(dimension: A) -> Self {
        Self {
            dimension: dimension.into(),
        }
    }
}
pub(crate) fn aspect_ratio_aligned_dimension(
    mut bound: Query<
        (
            &ImageHandle,
            &AspectRatioAlignedDimension,
            &mut Area<InterfaceContext>,
        ),
        Or<(
            Changed<AspectRatioAlignedDimension>,
            Changed<Area<InterfaceContext>>,
            Changed<ImageHandle>,
        )>,
    >,
    orientations: Res<ImageOrientations>,
) {
    for (name, max_dim, mut area) in bound.iter_mut() {
        let orientation = orientations.get(*name);
        let _dimensions_orientation = Orientation::new(max_dim.dimension.as_numerical());
        let mut attempted_width = max_dim.dimension.width;
        let mut attempted_height = attempted_width * orientation.value().reciprocal();
        while attempted_height > max_dim.dimension.height {
            attempted_width -= 1f32;
            attempted_height = attempted_width * orientation.value().reciprocal();
        }
        *area = Area::new(attempted_width, attempted_height);
    }
}
#[derive(Component, Default)]
pub(crate) struct Cache {
    pub(crate) name: Option<ImageHandle>,
    pub(crate) fade: Option<ImageFade>,
    pub(crate) pos: Option<Position<InterfaceContext>>,
    pub(crate) area: Option<Area<InterfaceContext>>,
    pub(crate) layer: Option<Layer>,
}

#[derive(Component, Clone, Default)]
pub(crate) struct Difference {
    pub(crate) name: Option<ImageHandle>,
    pub(crate) fade: Option<ImageFade>,
    pub(crate) pos: Option<Position<InterfaceContext>>,
    pub(crate) area: Option<Area<InterfaceContext>>,
    pub(crate) layer: Option<Layer>,
}
pub(crate) fn name_diff(
    mut images: Query<(&ImageHandle, &mut Cache, &mut Difference), Changed<ImageHandle>>,
) {
    for (name, mut cache, mut difference) in images.iter_mut() {
        if let Some(cached) = cache.name.as_ref() {
            if cached.0 != name.0 {
                difference.name.replace(*name);
            }
        }
        cache.name.replace(*name);
    }
}
pub(crate) fn fade_diff(
    mut images: Query<(&ImageFade, &mut Cache, &mut Difference), Changed<ImageFade>>,
) {
    for (fade, mut cache, mut difference) in images.iter_mut() {
        if let Some(cached) = cache.fade.as_ref() {
            if *cached != *fade {
                difference.fade.replace(*fade);
            }
        }
        cache.fade.replace(*fade);
    }
}
pub(crate) fn pos_diff(
    mut images: Query<
        (&Position<InterfaceContext>, &mut Cache, &mut Difference),
        Changed<Position<InterfaceContext>>,
    >,
) {
    for (pos, mut cache, mut difference) in images.iter_mut() {
        if let Some(cached) = cache.pos.as_ref() {
            if *cached != *pos {
                difference.pos.replace(*pos);
            }
        }
        cache.pos.replace(*pos);
    }
}
pub(crate) fn area_diff(
    mut images: Query<
        (&Area<InterfaceContext>, &mut Cache, &mut Difference),
        Changed<Area<InterfaceContext>>,
    >,
) {
    for (area, mut cache, mut difference) in images.iter_mut() {
        if let Some(cached) = cache.area.as_ref() {
            if *cached != *area {
                difference.area.replace(*area);
            }
        }
        cache.area.replace(*area);
    }
}
pub(crate) fn layer_diff(mut images: Query<(&Layer, &mut Cache, &mut Difference), Changed<Layer>>) {
    for (layer, mut cache, mut difference) in images.iter_mut() {
        if let Some(cached) = cache.layer.as_ref() {
            if *cached != *layer {
                difference.layer.replace(*layer);
            }
        }
        cache.layer.replace(*layer);
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
            &ImageHandle,
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
            cache.pos.replace(*pos);
            cache.area.replace(*area);
            cache.layer.replace(*layer);
            cache.name.replace(*name);
            cache.fade.replace(*fade);
            difference.pos.replace(cache.pos.unwrap());
            difference.area.replace(cache.area.unwrap());
            difference.layer.replace(cache.layer.unwrap());
            difference.fade.replace(cache.fade.unwrap());
            difference.name.replace(cache.name.unwrap());
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
