use placement_tool_logic::{Engen, LogicAttachment};
use workflow_visualizer::{Color, GfxOptions, Runner, Theme, ThemeDescriptor, Visualizer};

fn main() {
    let theme_desc = ThemeDescriptor::new().with_background(Color::OFF_BLACK);
    let visualizer = Visualizer::new(
        Theme::new(theme_desc),
        GfxOptions::native_defaults().with_msaa(1),
    );
    Runner::new()
        .with_attachment::<LogicAttachment>()
        .with_desktop_dimensions((1800, 900))
        .native_run::<Engen>(visualizer);
}
