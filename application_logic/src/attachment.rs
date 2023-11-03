use workflow_visualizer::{Attach, BundleExtension, BundledIcon, Button, ButtonBorder, ButtonIcon, ButtonText, ButtonType, Color, IconRequest, SectionOutline, TextValue, Visualizer, Icon, Image, ImageFade, IconScale, ResponsiveGridPoint};
use workflow_visualizer::{GridUnit, Text, TextWrapStyle};
use workflow_visualizer::{ResponsiveGridLocation, ResponsiveGridRange, ResponsiveGridView};

pub struct EntryAttachment;
impl Attach for EntryAttachment {
    fn attach(visualizer: &mut Visualizer) {
        visualizer.spawn(IconRequest::new(0, BundledIcon::EditTwo.data()));
        visualizer.spawn(IconRequest::new(1, BundledIcon::PaintTest.data()));
        visualizer.spawn(IconRequest::new(2, BundledIcon::PaintTest2.data()));
        let view = ResponsiveGridView::new(
            ResponsiveGridRange::new(
                ResponsiveGridLocation::new(1.near()),
                ResponsiveGridLocation::new(15.far()),
            ),
            ResponsiveGridRange::new(
                ResponsiveGridLocation::new(1.near()),
                ResponsiveGridLocation::new(15.far()),
            ),
        );
        // visualizer.spawn(
        //     Icon::new(1, IconScale::Asymmetrical((600, 300)), 7, Color::OFF_WHITE).extend(
        //         ResponsiveGridPoint::new(ResponsiveGridLocation::new(1.near()), ResponsiveGridLocation::new(1.near()))
        //     )
        // );
        visualizer.spawn(
          Image::new(1, 10, ImageFade::OPAQUE).extend(view)
        );
        visualizer.spawn(
            Image::new(2, 8, ImageFade::OPAQUE).extend(
                ResponsiveGridView::new(
                    ResponsiveGridRange::new(
                        ResponsiveGridLocation::new(2.near()),
                        ResponsiveGridLocation::new(12.far()),
                    ),
                    ResponsiveGridRange::new(
                        ResponsiveGridLocation::new(2.near()),
                        ResponsiveGridLocation::new(4.far()),
                    ),
                )
            )
        );
        // visualizer.spawn(
        //     Text::new(
        //         5,
        //         "&Debug",
        //         96,
        //         Color::CYAN_DARK,
        //         TextWrapStyle::letter(),
        //     )
        //     .extend(view), // .extend(SectionOutline::default()),
        // );
        visualizer.spawn(
            Button::new(
                ButtonType::Press,
                4,
                Color::GREY_DARK,
                Color::OFF_WHITE,
                ButtonText::some(TextValue("Edits".to_string())),
                ButtonIcon::some(0.into()),
                ButtonBorder::None,
            )
            .extend(ResponsiveGridView::new(
                ResponsiveGridRange::new(
                    ResponsiveGridLocation::new(3.near())
                        .with_tablet(2.near())
                        .with_desktop(2.near()),
                    ResponsiveGridLocation::new(5.far())
                        .with_tablet(5.far())
                        .with_desktop(6.far()),
                ),
                ResponsiveGridRange::new(
                    ResponsiveGridLocation::new(3.near())
                        .with_tablet(2.near())
                        .with_desktop(2.near()),
                    ResponsiveGridLocation::new(3.far())
                        .with_tablet(3.far())
                        .with_desktop(3.far()),
                ),
            )),
        );
    }
}
