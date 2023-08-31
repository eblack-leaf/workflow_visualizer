use workflow_visualizer::{Color, GfxOptions, Runner, Theme, ThemeDescriptor, Visualizer};

use crate::engen::Engen;
use crate::pad::PadAttachment;

mod engen;
mod pad;

fn main() {
    // tracing_subscriber::fmt().with_max_level(Level::TRACE).init();
    let visualizer = Visualizer::new(
        Theme::new(ThemeDescriptor::new().with_background(Color::OFF_BLACK)),
        GfxOptions::native_defaults().with_msaa(2),
    );
    Runner::new()
        .with_attachment::<PadAttachment>()
        .with_desktop_dimensions((600, 400))
        .native_run::<Engen>(visualizer);
}
