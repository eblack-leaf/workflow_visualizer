/// Interpreter for Diagnostic traces
pub struct Diagnostics {
    // regex for interpreting output (at least core output)
    // organized by @counter to show timeline
}

impl Diagnostics {}

/// Handle for use in Local<...> resources in systems to record info
pub struct DiagnosticsHandle<T: Record + Default + Send + 'static> {
    counter: usize,
    pub ext: T,
}

impl<T: Record + Default + Send + 'static> DiagnosticsHandle<T> {
    pub fn record(&mut self) -> String {
        self.counter += 1;
        let core_record = format!("@sys.counter:{:?}", self.counter);
        self.ext.record(core_record)
    }
}

impl<T: Record + Default + Send + 'static> Default for DiagnosticsHandle<T> {
    fn default() -> Self {
        Self {
            counter: 0,
            ext: T::default(),
        }
    }
}

/// How to record aspects to your recorder
pub trait Record {
    fn record(&self, core_record: String) -> String;
}
