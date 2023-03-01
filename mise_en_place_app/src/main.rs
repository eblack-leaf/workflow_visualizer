#![allow(unused, dead_code)]

use mise_en_place::{Animate, Attachment, Color, Engen, EngenOptions, IconAttachment, Job, Launch, Location, PositionAdjust, PositionAdjustAnimator, Text, TextAttachment, TextBundle, TextPartition, TextScaleAlignment, UIView};

#[cfg(not(target_arch = "wasm32"))]
mod server;

struct Launcher;

impl Launch for Launcher {
    fn options() -> EngenOptions {
        EngenOptions::new().with_native_dimensions((500, 900))
    }

    fn attachments() -> Vec<Attachment> {
        vec![
            Attachment::using::<TextAttachment>(),
            Attachment::using::<IconAttachment>(),
            Attachment::using::<PositionAdjustAnimator>(),
        ]
    }

    fn prepare(job: &mut Job) {
        job.container.spawn(
            TextBundle::new(
                Text::new(
                    vec![TextPartition::new(
                        "animated text", (Color::OFF_WHITE, 0),
                    )]
                ),
                Location::new((0, 0), 0),
                TextScaleAlignment::Medium,
            )
        ).insert(PositionAdjust::<UIView>::new(100.0, 0.0).animate(2.0));
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        server::compile_and_serve();
    }
    Engen::launch::<Launcher>();
}
