use crate::job::Job;

pub struct App {
    pub job: Job,
    // ... room for other instrumentation e.g. diagnostics
}
impl App {
    pub fn new() -> Self {
        Self { job: Job::new() }
    }
}
