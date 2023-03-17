use std::net::SocketAddr;

use bevy_ecs::prelude::{Entity, IntoSystemConfig, Query, Res, ResMut};
use winit::event_loop::EventLoop;

use workflow_visualizer::{
    Color, Engen, EngenOptions, EntityStore, GfxOptions, Idle, InterfaceContext, Job, Launch,
    Location, Panel, Position, Request, Text, TextContent, TextContentView, TextGridDescriptor,
    TextInputRequest, TextInputText, TextScaleAlignment, Theme, ThemeDescriptor, Timer,
    UserSpaceSyncPoint, VisibleSection,
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
    mut text_query: Query<(&mut TextContent, &Position<InterfaceContext>)>,
    timer: Res<Timer>,
    _text_input: Query<(Entity, &TextInputText, &VisibleSection)>,
) {
    idle.can_idle = false;
    let text_entity = *entity_store.store.get("animated_text").unwrap();
    if let Ok((mut text, pos)) = text_query.get_mut(text_entity) {
        text.data = format!("text pos at: {:.2}, {:.2}", pos.x, pos.y);
    }
    let text_entity = *entity_store.store.get("timer_text").unwrap();
    if let Ok((mut text, _pos)) = text_query.get_mut(text_entity) {
        text.data = format!("timer: {:.2}", timer.mark().0);
    }
}
//
// fn post_anim_logic(
//     mut removed: RemovedComponents<Animation<PositionAdjustAnimator>>,
//     entity_store: Res<EntityStore>,
//     anim_start: Query<Entity, Added<Animation<PositionAdjustAnimator>>>,
//     mut text_query: Query<(&mut TextContent, &Position<InterfaceContext>)>,
//     timer: Res<Timer>,
// ) {
//     for _added in anim_start.iter() {
//         let text_entity = *entity_store.store.get("start_text").unwrap();
//         let (mut text, pos) = text_query.get_mut(text_entity).unwrap();
//         text.data = format!("start at: {:.2}", timer.mark().0)
//     }
//     for _remove in removed.iter() {
//         let text_entity = *entity_store.store.get("done_text").unwrap();
//         let (mut text, pos) = text_query.get_mut(text_entity).unwrap();
//         text.data = format!("done at: {:.2}", timer.mark().0);
//     }
// }
struct Launcher;
impl Launch for Launcher {
    fn options() -> EngenOptions {
        EngenOptions::new()
            .with_native_dimensions((500, 900))
            .with_theme(Theme::new(
                ThemeDescriptor::new().with_background(Color::DARK_CYAN),
            ))
            .with_gfx_options(GfxOptions::native().with_msaa(4))
    }

    fn preparation(frontend: &mut Job) {
        let id = frontend
            .container
            .spawn(Request::new(Panel::new(
                Location::from(((10.0, 10.0), 3)),
                (42 * 11, 200),
                Color::CYAN,
                1,
                Color::DARK_ORANGE,
            )))
            .id();
        frontend.store_entity("panel", id);
        let id = frontend
            .container
            .spawn(Request::new(Text::new(
                TextContent::new("animated text"),
                TextContentView::new(0, 50u32, Color::DARK_CYAN),
                Location::new((15.0, 15.0), 0),
                TextScaleAlignment::Medium,
                TextGridDescriptor::new(100, 1),
            )))
            .id();
        frontend.store_entity("animated_text", id);
        frontend
            .main
            .add_system(logic.in_set(UserSpaceSyncPoint::Process));
        let id = frontend
            .container
            .spawn(Request::new(Text::new(
                TextContent::new("timer:"),
                TextContentView::new(0, 50u32, Color::DARK_CYAN),
                Location::new((15.0, 40.0), 0),
                TextScaleAlignment::Medium,
                TextGridDescriptor::new(100, 1),
            )))
            .id();
        frontend.store_entity("timer_text", id);
        let id = frontend
            .container
            .spawn(Request::new(Text::new(
                TextContent::new("start at:"),
                TextContentView::new(0, 50u32, Color::DARK_CYAN),
                Location::new((15.0, 80.0), 0),
                TextScaleAlignment::Medium,
                TextGridDescriptor::new(100, 1),
            )))
            .id();
        frontend.store_entity("start_text", id);
        let id = frontend
            .container
            .spawn(Request::new(Text::new(
                TextContent::new("Address:"),
                TextContentView::new(0, 50u32, Color::DARK_CYAN),
                Location::new((15.0, 120.0), 0),
                TextScaleAlignment::Medium,
                TextGridDescriptor::new(100, 4),
            )))
            .id();
        frontend.store_entity("done_text", id);
        let id = frontend
            .container
            .spawn(Request::new(TextInputRequest::new(
                "".to_string(),
                TextScaleAlignment::Medium,
                TextGridDescriptor::new(43, 3),
                Location::from(((10, 260), 0)),
                Color::DARK_CYAN,
                Color::DARK_ORANGE,
            )))
            // .insert(PositionAdjust::<UIView>::new(400.0, 0.0).animate(4.0))
            .id();
        frontend.store_entity("text input", id);
    }
}
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    compile_and_serve();
    Engen::launch::<Launcher>(EventLoop::new());
}
