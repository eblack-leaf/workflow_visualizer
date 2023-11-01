use workflow_visualizer::{
    Attach, BundleExtension, BundledIcon, Button, ButtonBorder, ButtonIcon, ButtonText, ButtonType,
    Color, IconRequest, SectionOutline, TextValue, Visualizer,
};
use workflow_visualizer::{GridUnit, Text, TextWrapStyle};
use workflow_visualizer::{ResponsiveGridLocation, ResponsiveGridRange, ResponsiveGridView};

pub struct EntryAttachment;
impl Attach for EntryAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.spawn(IconRequest::new(0, BundledIcon::Anchor.data()));
        visualizer.spawn(
            Text::new(
                5,
                "&Dgqu9 TetaT ete aseto acetuta oeuhtao",
                122,
                Color::CYAN_DARK,
                TextWrapStyle::letter(),
            )
            .extend(ResponsiveGridView::new(
                ResponsiveGridRange::new(
                    ResponsiveGridLocation::new(1.near()),
                    ResponsiveGridLocation::new(10.far()),
                ),
                ResponsiveGridRange::new(
                    ResponsiveGridLocation::new(4.near()),
                    ResponsiveGridLocation::new(8.far()),
                ),
            )), // .extend(SectionOutline::default()),
        );
        visualizer.spawn(
            Button::new(
                ButtonType::Press,
                5,
                Color::OFF_BLACK,
                Color::CYAN_DARK,
                ButtonText::some(TextValue("99999".to_string())),
                ButtonIcon::some(0.into()),
                ButtonBorder::None,
            )
            .extend(ResponsiveGridView::new(
                ResponsiveGridRange::new(
                    ResponsiveGridLocation::new(1.near())
                        .with_tablet(1.near())
                        .with_desktop(1.near()),
                    ResponsiveGridLocation::new(3.far())
                        .with_tablet(4.far())
                        .with_desktop(5.far()),
                ),
                ResponsiveGridRange::new(
                    ResponsiveGridLocation::new(1.near())
                        .with_tablet(1.near())
                        .with_desktop(1.near()),
                    ResponsiveGridLocation::new(1.far())
                        .with_tablet(2.far())
                        .with_desktop(3.far()),
                ),
            )),
        );
    }
}
