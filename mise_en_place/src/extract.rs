use crate::{Stove, RecipeDirections};

pub(crate) type ExtractFns = Vec<Box<fn(&mut RecipeDirections, &mut RecipeDirections)>>;

pub trait Season {
    fn extract(frontend: &mut RecipeDirections, backend: &mut RecipeDirections);
}

pub(crate) fn invoke_extract<Extractor: Season>(frontend: &mut RecipeDirections, backend: &mut RecipeDirections) {
    Extractor::extract(frontend, backend);
}

pub(crate) fn extract(engen: &mut Stove) {
    for invoke in engen.extract_fns.iter_mut() {
        invoke(&mut engen.frontend, &mut engen.backend);
    }
}
