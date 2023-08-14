use crate::engen::Engen;
use crate::pad::PadAttachment;
use workflow_visualizer::{Color, GfxOptions, Runner, Theme, ThemeDescriptor, Visualizer};

mod engen;
mod pad;

fn main() {
    // tracing_subscriber::fmt().with_max_level(Level::TRACE).init();
    let mut visualizer = Visualizer::new(
        Theme::new(ThemeDescriptor::new().with_background(Color::OFF_BLACK)),
        GfxOptions::native_defaults().with_msaa(2),
    );
    visualizer.add_attachment::<PadAttachment>();
    Runner::new()
        .with_desktop_dimensions((600, 400))
        .native_run::<Engen>(visualizer);
}
