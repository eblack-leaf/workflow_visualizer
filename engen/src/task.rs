use crate::job::Job;

pub struct Task {
    pub job: Job,
    // ... room for other instrumentation e.g. diagnostics
}
impl Task {
    pub fn new() -> Self {
        Self { job: Job::new() }
    }
}
