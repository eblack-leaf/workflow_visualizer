use std::net::SocketAddr;

use bevy_ecs::prelude::{Entity, IntoSystemConfig, Query, Res, ResMut};
use winit::event_loop::EventLoop;

use workflow_visualizer::{
    Area, Color, Coordinate, Engen, EngenOptions, EntityStore, GfxOptions, Idle, InterfaceContext,
    Job, Launch, Layer, Location, Panel, Position, Request, Section, Text, TextRequest,
    TextScaleAlignment, TextWrapStyle, Theme, ThemeDescriptor, Timer, UserSpaceSyncPoint,
    VisibleSection, WrapStyleExpt,
};

#[cfg(not(target_arch = "wasm32"))]
pub fn compile_and_serve() {
    use workflow_visualizer::{WasmCompiler, WasmServer};
    let args: Vec<String> = std::env::args().collect();
    let wasm_compiler = WasmCompiler::new(
        "workflow_visualizer",
        "--example",
        "app",
        "release",
        "app_web_build",
    );
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
    idle.can_idle = false;
    let text_entity = *entity_store.store.get("timer_text").unwrap();
    if let Ok((mut text, _pos)) = text_query.get_mut(text_entity) {
        text.0 = format!("timer: {:.2}", timer.mark().0);
    }
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
                Location::from(((10.0, 10.0), 3)),
                (480, 300),
                Color::DARK_CYAN,
            )))
            .id();
        frontend.store_entity("panel", id);
        let id = frontend
            .container
            .spawn(Request::new(TextRequest::new(
                Coordinate::new(Section::new(Position::new(15.0, 15.0), Area::new(380.0, 260.0)), Layer::new(0.0)),
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
        let id = frontend
            .container
            .spawn(Request::new(TextRequest::new(
                Coordinate::new(
                    Section::new(Position::new(15.0, 615.0), Area::new(380.0, 260.0)),
                    Layer::new(0.0),
                ),
                String::from("timer: not started yet"),
                TextScaleAlignment::Medium,
                Color::DARK_ORANGE,
                TextWrapStyle(WrapStyleExpt::Word),
            )))
            .id();
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
