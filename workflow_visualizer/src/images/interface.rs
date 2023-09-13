use std::collections::{HashMap, HashSet};

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{
    Bundle, Changed, Component, Or, Query, RemovedComponents, Res, Resource, Without,
};
use bevy_ecs::query::With;
use bevy_ecs::system::ResMut;

use crate::bundling::ResourceHandle;
use crate::images::renderer::{ImageFade, ImageOrientations};
use crate::{
    Animate, Animation, Area, Color, Disabled, EnableVisibility, IconScale, InterfaceContext,
    Interpolation, Layer, Orientation, Position, Section, Tag, Visibility,
};

pub type ImageTag = Tag<Image>;
#[derive(Bundle)]
pub struct Image {
    section: Section<InterfaceContext>,
    layer: Layer,
    visibility: EnableVisibility,
    handle: ResourceHandle,
    fade: ImageFade,
    cache: Cache,
    difference: Difference,
    color: Color,
    tag: ImageTag,
}
impl Image {
    pub fn new<IN: Into<ResourceHandle>, L: Into<Layer>, IF: Into<ImageFade>>(
        name: IN,
        layer: L,
        fade: IF,
    ) -> Self {
        Self {
            section: Section::default(),
            layer: layer.into(),
            visibility: EnableVisibility::default(),
            handle: name.into(),
            fade: fade.into(),
            cache: Cache::default(),
            difference: Difference::default(),
            color: ImageIcon::INVALID_COLOR,
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
impl Animate for AspectRatioAlignedDimension {
    fn interpolations(&self, end: &Self) -> Vec<Interpolation> {
        vec![
            Interpolation::new(end.dimension.width - self.dimension.width),
            Interpolation::new(end.dimension.height - self.dimension.height),
        ]
    }
}
pub(crate) fn apply_aspect_animations(
    mut anims: Query<(
        &mut AspectRatioAlignedDimension,
        &mut Animation<AspectRatioAlignedDimension>,
    )>,
) {
    for (mut dim, mut anim) in anims.iter_mut() {
        let extractions = anim.extractions();
        if let Some(extract) = extractions.get(0).unwrap() {
            dim.dimension.width += extract.0;
        }
        if let Some(extract) = extractions.get(1).unwrap() {
            dim.dimension.height += extract.0;
        }
    }
}
pub(crate) fn aspect_ratio_aligned_dimension(
    mut bound: Query<
        (
            &ResourceHandle,
            &AspectRatioAlignedDimension,
            &mut Area<InterfaceContext>,
        ),
        Or<(
            Changed<AspectRatioAlignedDimension>,
            Changed<Area<InterfaceContext>>,
            Changed<ResourceHandle>,
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
pub type ImageIconTag = Tag<ImageIcon>;
#[derive(Bundle)]
pub struct ImageIcon {
    section: Section<InterfaceContext>,
    layer: Layer,
    visibility: EnableVisibility,
    handle: ResourceHandle,
    fade: ImageFade,
    cache: Cache,
    difference: Difference,
    tag: ImageTag,
    image_icon_tag: ImageIconTag,
    scale: IconScale,
    color: Color,
}
impl ImageIcon {
    pub fn new<RH: Into<ResourceHandle>, IS: Into<IconScale>, L: Into<Layer>, C: Into<Color>>(
        handle: RH,
        scale: IS,
        layer: L,
        color: C,
    ) -> Self {
        Self {
            handle: handle.into(),
            scale: scale.into(),
            layer: layer.into(),
            color: color.into(),
            fade: ImageFade::OPAQUE,
            cache: Cache::default(),
            difference: Difference::default(),
            tag: ImageTag::new(),
            image_icon_tag: ImageIconTag::new(),
            visibility: EnableVisibility::new(),
            section: Section::default(),
        }
    }
    pub(crate) const INVALID_COLOR: Color = Color {
        red: -1.0,
        green: -1.0,
        blue: -1.0,
        alpha: -1.0,
    };
}
#[derive(Component, Default)]
pub(crate) struct Cache {
    pub(crate) name: Option<ResourceHandle>,
    pub(crate) fade: Option<ImageFade>,
    pub(crate) pos: Option<Position<InterfaceContext>>,
    pub(crate) area: Option<Area<InterfaceContext>>,
    pub(crate) layer: Option<Layer>,
    pub(crate) icon_color: Option<Color>,
}

#[derive(Component, Clone, Default)]
pub(crate) struct Difference {
    pub(crate) name: Option<ResourceHandle>,
    pub(crate) fade: Option<ImageFade>,
    pub(crate) pos: Option<Position<InterfaceContext>>,
    pub(crate) area: Option<Area<InterfaceContext>>,
    pub(crate) layer: Option<Layer>,
    pub(crate) icon_color: Option<Color>,
}
pub(crate) fn set_from_scale(
    mut image_icons: Query<(&IconScale, &mut Area<InterfaceContext>), Changed<IconScale>>,
) {
    for (scale, mut area) in image_icons.iter_mut() {
        area.width = scale.width();
        area.height = scale.height();
    }
}
pub(crate) fn icon_color_diff(
    mut image_icons: Query<
        (&Color, &mut Cache, &mut Difference),
        (Changed<Color>, With<ImageIconTag>),
    >,
) {
    for (color, mut cache, mut difference) in image_icons.iter_mut() {
        if let Some(cached) = cache.icon_color.as_ref() {
            if *cached != *color {
                difference.icon_color.replace(*color);
            }
        }
        cache.icon_color.replace(*color);
    }
}
pub(crate) fn name_diff(
    mut images: Query<(&ResourceHandle, &mut Cache, &mut Difference), Changed<ResourceHandle>>,
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
            &ResourceHandle,
            &ImageFade,
            &Visibility,
            &mut Cache,
            &mut Difference,
            &Color,
            Option<&ImageIconTag>,
        ),
        Changed<Visibility>,
    >,
    mut removed: RemovedComponents<ImageTag>,
    mut extraction: ResMut<Extraction>,
) {
    for (
        entity,
        pos,
        area,
        layer,
        name,
        fade,
        visibility,
        mut cache,
        mut difference,
        icon_color,
        image_icon,
    ) in images.iter_mut()
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
            if image_icon.is_some() {
                cache.icon_color.replace(*icon_color);
                difference.icon_color.replace(*icon_color);
            }
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
