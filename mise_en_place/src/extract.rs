use crate::{Job, Stove};

pub(crate) type ExtractFns = Vec<Box<fn(&mut Job, &mut Job)>>;

pub trait Extract {
    fn season(frontend: &mut Job, backend: &mut Job);
}

pub(crate) fn invoke_extract<Extractor: Extract>(frontend: &mut Job, backend: &mut Job) {
    Extractor::season(frontend, backend);
}

pub(crate) fn extract(stove: &mut Stove) {
    for seasoning in stove.extract_fns.iter_mut() {
        seasoning(&mut stove.frontend, &mut stove.backend);
    }
}
