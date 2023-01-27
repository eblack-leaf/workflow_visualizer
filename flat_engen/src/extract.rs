use crate::Task;
pub(crate) type ExtractFns = Vec<Box<fn(&mut Task, &mut Task)>>;
pub trait Extract {
    fn extract(frontend: &mut Task, backend: &mut Task);
}
pub(crate) fn invoke_extract<Extractor: Extract>(frontend: &mut Task, backend: &mut Task) {
    Extractor::extract(frontend, backend);
}
