use crate::{Stove, RecipeDirections};

pub(crate) type Spices = Vec<Box<fn(&mut RecipeDirections, &mut RecipeDirections)>>;

pub trait Season {
    fn season(frontend: &mut RecipeDirections, backend: &mut RecipeDirections);
}

pub(crate) fn add_seasoning<Extractor: Season>(frontend: &mut RecipeDirections, backend: &mut RecipeDirections) {
    Extractor::season(frontend, backend);
}

pub(crate) fn season(engen: &mut Stove) {
    for seasoning in engen.spices.iter_mut() {
        seasoning(&mut engen.frontend, &mut engen.backend);
    }
}
