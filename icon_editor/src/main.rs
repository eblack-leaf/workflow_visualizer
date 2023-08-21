use workflow_visualizer::{
    BundledIcon, Color, GfxOptions, IconBitmap, IconBitmapRequest, Runner, Theme, ThemeDescriptor,
    Visualizer,
};

use crate::engen::Engen;
use crate::pad::PadAttachment;

mod engen;
mod pad;

fn main() {
    // tracing_subscriber::fmt().with_max_level(Level::TRACE).init();
    let mut visualizer = Visualizer::new(
        Theme::new(ThemeDescriptor::new().with_background(Color::OFF_BLACK)),
        GfxOptions::native_defaults().with_msaa(2),
    );
    visualizer.add_attachment::<PadAttachment>();
    visualizer.spawn(IconBitmapRequest::from((
        "edit",
        IconBitmap::bundled(BundledIcon::Edit),
    )));
    visualizer.spawn(IconBitmapRequest::from((
        "square",
        IconBitmap::bundled(BundledIcon::Square),
    )));
    Runner::new()
        .with_desktop_dimensions((600, 400))
        .native_run::<Engen>(visualizer);
}
