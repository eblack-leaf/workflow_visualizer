use winit::event_loop::EventLoop;
use workflow_visualizer::{Engen, EngenOptions, Job, Launch};

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
        WasmServer::serve_at("app_web_build", ([0, 0, 0, 0], 3030));
        return;
    }
}
fn logic(
    mut idle: ResMut<Idle>,
    entity_store: Res<EntityStore>,
    mut text_query: Query<(&mut TextContent, &Position<InterfaceContext>)>,
    timer: Res<Timer>,
    text_input: Query<(Entity, &TextInputText, &VisibleSection)>,
) {
    idle.can_idle = false;
    let text_entity = *entity_store.store.get("animated_text").unwrap();
    if let Ok((mut text, pos)) = text_query.get_mut(text_entity) {
        text.data = format!("text pos at: {:.2}, {:.2}", pos.x, pos.y);
    }
    let text_entity = *entity_store.store.get("timer_text").unwrap();
    if let Ok((mut text, pos)) = text_query.get_mut(text_entity) {
        text.data = format!("timer: {:.2}", timer.mark().0);
    }
}

fn post_anim_logic(
    mut removed: RemovedComponents<Animation<PositionAdjustAnimator>>,
    entity_store: Res<EntityStore>,
    anim_start: Query<Entity, Added<Animation<PositionAdjustAnimator>>>,
    mut text_query: Query<(&mut TextContent, &Position<InterfaceContext>)>,
    timer: Res<Timer>,
) {
    for _added in anim_start.iter() {
        let text_entity = *entity_store.store.get("start_text").unwrap();
        let (mut text, pos) = text_query.get_mut(text_entity).unwrap();
        text.data = format!("start at: {:.2}", timer.mark().0)
    }
    for _remove in removed.iter() {
        let text_entity = *entity_store.store.get("done_text").unwrap();
        let (mut text, pos) = text_query.get_mut(text_entity).unwrap();
        text.data = format!("done at: {:.2}", timer.mark().0);
    }
}
struct Launcher;
impl Launch for Launcher {
    fn options() -> EngenOptions {
        EngenOptions::new().with_native_dimensions((500, 900))
    }

    fn preparation(frontend: &mut Job) {
        let id = job
            .container
            .spawn(Request::new(TextBundle::new(
                TextContent::new("animated text"),
                TextContentView::new(0, 50u32, Color::OFF_WHITE),
                Location::new((0.0, 0.0), 0),
                TextScaleAlignment::Medium,
                TextGridGuide::new(100, 1),
            )))
            .id();
        job.store_entity("animated_text", id);
        job.main.add_system(logic.in_set(FrontEndBuckets::Process));
        job.main
            .add_system(post_anim_logic.in_set(FrontEndBuckets::AnimationResolved));
        let id = job
            .container
            .spawn(Request::new(TextBundle::new(
                TextContent::new("timer:"),
                TextContentView::new(0, 50u32, Color::OFF_WHITE),
                Location::new((0.0, 40.0), 0),
                TextScaleAlignment::Medium,
                TextGridGuide::new(100, 1),
            )))
            .id();
        job.store_entity("timer_text", id);
        let id = job
            .container
            .spawn(Request::new(TextBundle::new(
                TextContent::new("start at:"),
                TextContentView::new(0, 50u32, Color::OFF_WHITE),
                Location::new((0.0, 80.0), 0),
                TextScaleAlignment::Medium,
                TextGridGuide::new(100, 1),
            )))
            .id();
        job.store_entity("start_text", id);
        let id = job
            .container
            .spawn(Request::new(TextBundle::new(
                TextContent::new("done at:"),
                TextContentView::new(0, 50u32, Color::OFF_WHITE),
                Location::new((0.0, 120.0), 0),
                TextScaleAlignment::Medium,
                TextGridGuide::new(100, 4),
            )))
            .id();
        job.store_entity("done_text", id);
        let id = job
            .container
            .spawn(Request::new(TextInputRequest::new(
                "".to_string(),
                TextScaleAlignment::Medium,
                TextGridGuide::new(32, 12),
                Location::from(((100, 120), 0)),
                Color::OFF_WHITE,
                Color::DARK_GREY,
            )))
            // .insert(PositionAdjust::<UIView>::new(400.0, 0.0).animate(4.0))
            .id();
        job.store_entity("text input", id);
    }
    }
}
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    compile_and_serve();
    Engen::launch::<Launcher>(EventLoop::new());
}
