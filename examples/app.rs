use std::net::SocketAddr;

use bevy_ecs::prelude::{Entity, IntoSystemConfig, Query, Res, ResMut};
use winit::event_loop::EventLoop;

use workflow_visualizer::{Area, Color, Coordinate, Engen, EngenOptions, EntityStore, FixedBreakPoint, GfxOptions, Idle, InterfaceContext, Job, Launch, Layer, Panel, PanelType, Position, RelativePoint, Request, Section, Text, TextInputRequest, TextRequest, TextScaleAlignment, TextWrapStyle, Theme, ThemeDescriptor, Timer, UserSpaceSyncPoint, ViewArea, ViewPoint, ViewPosition, WrapStyleExpt};

#[cfg(not(target_arch = "wasm32"))]
pub fn compile_and_serve() {
    use workflow_visualizer::{WasmCompiler, WasmServer};
    let args: Vec<String> = std::env::args().collect();
    let wasm_compiler = WasmCompiler::new("--example", "app", "release", "app_web_build");
    if args.contains(&"build".to_string()) {
        wasm_compiler.compile().expect("could not compile wasm");
        if !args.contains(&"serve".to_string()) {
            return;
        }
    }
    if args.contains(&"serve".to_string()) {
        let addr = ([0, 0, 0, 0], 3030);
        println!("serving at addr: https://{:?}/", SocketAddr::from(addr));
        WasmServer::serve_at("app_web_build", addr);
    }
}
fn logic(
    mut idle: ResMut<Idle>,
    entity_store: Res<EntityStore>,
    mut text_query: Query<(&mut Text, &Position<InterfaceContext>)>,
    timer: Res<Timer>,
) {
}
struct Launcher;
impl Launch for Launcher {
    fn options() -> EngenOptions {
        EngenOptions::new()
            .with_native_dimensions((500, 900))
            .with_theme(Theme::new(
                ThemeDescriptor::new().with_background(Color::BLUE_DARK),
            ))
            .with_gfx_options(GfxOptions::native().with_msaa(4))
    }

    fn preparation(frontend: &mut Job) {
        let id = frontend
            .container
            .spawn(Request::new(Panel::new(
                PanelType::BorderedPanel,
                ViewPosition::new(
                    ViewPoint::new(RelativePoint::new(0.025), Some(FixedBreakPoint(15.0))),
                    ViewPoint::new(RelativePoint::new(0.0139), Some(FixedBreakPoint(15.0))),
                ),
                ViewArea::new(
                    ViewPoint::new(RelativePoint::new(0.95), Some(FixedBreakPoint(490.0))),
                    ViewPoint::new(RelativePoint::new(0.4), None),
                ),
                Layer::new(10.0),
                Color::DARK_CYAN,
                Color::CYAN,
            )))
            .id();
        frontend.store_entity("panel", id);
        let id = frontend
            .container
            .spawn(Request::new(TextRequest::new(
                ViewPosition::new(
                    ViewPoint::new(RelativePoint::new(0.036), Some(FixedBreakPoint(40.0))),
                    ViewPoint::new(RelativePoint::new(40.0/1080.0), Some(FixedBreakPoint(40.0))),
                ),
                ViewArea::new(
                    ViewPoint::new(
                        RelativePoint::new(0.94),
                        Some(FixedBreakPoint(460.0)),
                    ),
                    ViewPoint::new(RelativePoint::new(0.38), None),
                ),
                Layer::new(0.0),
                String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, \
                sed do eiusmod tempor incididunt ut labore et dolore \
                magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi \
                ut aliquip ex ea commodo consequat. Duis aute irure dolor in\
                 reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur \
                 sint occaecat cupidatat non proident, sunt in culpa qui \
                 officia deserunt mollit anim id est laborum."),
                TextScaleAlignment::Medium,
                Color::CYAN,
                TextWrapStyle(WrapStyleExpt::Word),
            )))
            .id();
        frontend.store_entity("animated_text", id);
        let id = frontend.container.spawn(
            Request::new(TextInputRequest::new(
                ViewPosition::new(
                    ViewPoint::new(RelativePoint::new(0.025), Some(FixedBreakPoint(15.0))),
                    ViewPoint::new(RelativePoint::new(0.5139), None),
                ),
                ViewArea::new(
                    ViewPoint::new(RelativePoint::new(0.95), Some(FixedBreakPoint(490.0))),
                    ViewPoint::new(RelativePoint::new(0.12), None),
                ),
                Layer::new(0.0),
                "type here...".to_string(),
                TextScaleAlignment::Medium, Color::CYAN, Color::DARK_CYAN
            ))
        ).id();
        frontend.store_entity("timer_text", id);
        frontend
            .main
            .add_system(logic.in_set(UserSpaceSyncPoint::Process));
    }
}
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    compile_and_serve();
    Engen::launch::<Launcher>(EventLoop::new());
}
