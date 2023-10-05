use workflow_visualizer::{
    Attach, BundleExtension, BundledIcon, Button, ButtonBorder, ButtonIcon, ButtonText, ButtonType,
    Color, GridUnit, IconRequest, ResponsiveGridLocation, ResponsiveGridRange, ResponsiveGridView,
    TextValue, Visualizer,
};

pub struct EntryAttachment;
impl Attach for EntryAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.spawn(IconRequest::new(0, BundledIcon::Anchor.data()));
        visualizer.spawn(
            Button::new(
                ButtonType::Press,
                5,
                Color::OFF_WHITE,
                Color::GREY_DARK,
                ButtonText::some(TextValue("test".to_string())),
                ButtonIcon::some(0.into()),
                ButtonBorder::None,
            )
            .extend(ResponsiveGridView::new(
                ResponsiveGridRange::new(
                    ResponsiveGridLocation::new(1.near()),
                    ResponsiveGridLocation::new(4.far()),
                ),
                ResponsiveGridRange::new(
                    ResponsiveGridLocation::new(1.near()),
                    ResponsiveGridLocation::new(2.far()),
                ),
            )),
        );
    }
}
