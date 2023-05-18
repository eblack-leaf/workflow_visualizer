use crate::workflow::{Action, Engen, Response};
use workflow_visualizer::{GfxOptions, NativeRunner, Theme, Visualizer};

mod workflow;

fn main() {
    tracing_subscriber::fmt().init();
    let mut visualizer = Visualizer::new(Theme::default(), GfxOptions::native());
    // ... visualizer.something();
    // ...
    let runner = NativeRunner::<Engen>::desktop();
    runner.run(visualizer, Engen::runner);
}
