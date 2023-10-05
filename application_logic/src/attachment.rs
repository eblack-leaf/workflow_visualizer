use workflow_visualizer::GridUnit;
use workflow_visualizer::{
    Attach, BundleExtension, BundledIcon, Button, ButtonBorder, ButtonIcon, ButtonText, ButtonType,
    Color, IconRequest, TextValue, Visualizer,
};
use workflow_visualizer::{ResponsiveGridLocation, ResponsiveGridRange, ResponsiveGridView};

pub struct EntryAttachment;
impl Attach for EntryAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.spawn(IconRequest::new(0, BundledIcon::Anchor.data()));
        visualizer.spawn(
            Button::new(
                ButtonType::Press,
                5,
                Color::GREY,
                Color::RED_ORANGE_DARK,
                ButtonText::some(TextValue("hello".to_string())),
                ButtonIcon::some(0.into()),
                ButtonBorder::None,
            )
            .extend(ResponsiveGridView::new(
                ResponsiveGridRange::new(
                    ResponsiveGridLocation::new(1.near())
                        .with_tablet(1.near())
                        .with_desktop(1.near()),
                    ResponsiveGridLocation::new(3.far())
                        .with_tablet(3.far())
                        .with_desktop(9.far()),
                ),
                ResponsiveGridRange::new(
                    ResponsiveGridLocation::new(1.near())
                        .with_tablet(1.near())
                        .with_desktop(1.near()),
                    ResponsiveGridLocation::new(1.far())
                        .with_tablet(2.far())
                        .with_desktop(6.far()),
                ),
            )),
        );
    }
}
