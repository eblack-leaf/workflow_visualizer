use crate::engen::Engen;
use crate::Job;

pub(crate) type ExtractFns = Vec<Box<fn(&mut Job, &mut Job)>>;

pub trait Extract {
    fn extract(frontend: &mut Job, backend: &mut Job);
}

pub(crate) fn invoke_extract<Extractor: Extract>(frontend: &mut Job, backend: &mut Job) {
    Extractor::extract(frontend, backend);
}

pub(crate) fn extract(engen: &mut Engen) {
    for extract_invoker in engen.extract_fns.iter_mut() {
        extract_invoker(&mut engen.frontend, &mut engen.backend);
    }
}
